use std::collections::{HashSet, VecDeque};
use std::sync::atomic::{AtomicBool, AtomicI64, Ordering};
use std::sync::{Arc, Mutex};

use log::{debug, info, warn};
use tauri::tray::TrayIconId;
use tauri::{AppHandle, Emitter, Manager};
use tokio::time::{sleep, Duration};

use super::classifier;
use super::detector;
use crate::commands::settings_commands::SettingsState;
use crate::db::models::SessionUpdate;
use crate::db::queries;
use crate::db::DbState;

#[derive(Clone)]
struct WorkflowSample {
    app_name: String,
    category: classifier::ActivityCategory,
    focus_kind: String,
    tick_secs: i64,
}

struct WorkflowRuntime {
    current_session_id: Option<i64>,
    recent_samples: VecDeque<WorkflowSample>,
    last_app_name: Option<String>,
    last_focus_kind: Option<String>,
    last_category: Option<classifier::ActivityCategory>,
    context_switches_10m: i64,
    ai_streak_secs: i64,
    coding_streak_secs: i64,
}

impl Default for WorkflowRuntime {
    fn default() -> Self {
        Self {
            current_session_id: None,
            recent_samples: VecDeque::new(),
            last_app_name: None,
            last_focus_kind: None,
            last_category: None,
            context_switches_10m: 0,
            ai_streak_secs: 0,
            coding_streak_secs: 0,
        }
    }
}

#[derive(Clone)]
struct WorkflowSignals {
    workflow_state: String,
    context_switches_10m: i64,
    ai_streak_secs: i64,
    coding_streak_secs: i64,
    prompt_loop_score: i64,
}

pub struct MonitorState {
    pub is_running: AtomicBool,
    pub is_paused: AtomicBool,
    pub auto_paused: AtomicBool,
    pub continuous_ai_secs: AtomicI64,
    workflow: Mutex<WorkflowRuntime>,
}

impl Default for MonitorState {
    fn default() -> Self {
        Self {
            is_running: AtomicBool::new(false),
            is_paused: AtomicBool::new(false),
            auto_paused: AtomicBool::new(false),
            continuous_ai_secs: AtomicI64::new(0),
            workflow: Mutex::new(WorkflowRuntime::default()),
        }
    }
}

pub fn start_monitoring(app_handle: AppHandle) {
    let monitor_state = app_handle.state::<Arc<MonitorState>>();
    if monitor_state.is_running.swap(true, Ordering::SeqCst) {
        return;
    }
    monitor_state.is_paused.store(false, Ordering::SeqCst);
    info!("Background monitoring started");

    let handle = app_handle.clone();
    tauri::async_runtime::spawn(async move {
        let mut summary_tick = 0u32;

        loop {
            if !handle
                .state::<Arc<MonitorState>>()
                .is_running
                .load(Ordering::SeqCst)
            {
                break;
            }

            let polling_secs = handle
                .try_state::<Arc<SettingsState>>()
                .and_then(|settings| settings.get_i64("polling_interval_secs"))
                .unwrap_or(5)
                .clamp(1, 30) as u64;

            tick_monitor(&handle, polling_secs, &mut summary_tick);
            sleep(Duration::from_secs(polling_secs)).await;
        }
    });
}

fn tick_monitor(handle: &AppHandle, polling_secs: u64, summary_tick: &mut u32) {
    let state = handle.state::<Arc<MonitorState>>();
    let db = handle.state::<DbState>();

    let idle_secs = detector::get_idle_seconds();

    if state.is_paused.load(Ordering::SeqCst) {
        if state.auto_paused.load(Ordering::SeqCst) {
            if maybe_auto_stop(handle, idle_secs) {
                return;
            }
            if maybe_auto_resume(handle, idle_secs) {
                return;
            }
        }
        return;
    }

    if maybe_auto_pause(handle, idle_secs) {
        return;
    }

    let window = match detector::get_active_window_info() {
        Ok(w) => w,
        Err(e) => {
            warn!("Window detection failed: {}", e);
            return;
        }
    };

    let (category, app_name) = detect_activity(&window);
    let focus_kind = classify_focus_kind(&window, category, &app_name);

    let conn = match db.conn.lock() {
        Ok(c) => c,
        Err(_) => return,
    };

    let mut session = match queries::get_active_session(&conn) {
        Ok(found) => found,
        Err(e) => {
            warn!("Failed to read active session: {}", e);
            return;
        }
    };

    if session.is_none() {
        if should_auto_start(handle, category) {
            match queries::create_session(&conn) {
                Ok(created) => {
                    info!("Auto-started session due to coding activity");
                    clear_runtime_state(handle);
                    let _ = handle.emit("session-started", &created);
                    session = Some(created);
                }
                Err(e) => {
                    warn!("Failed to auto-start session: {}", e);
                    return;
                }
            }
        } else {
            return;
        }
    }

    let session = match session {
        Some(current) if current.status == "active" => current,
        _ => return,
    };

    let tick_secs = polling_secs as i64;
    let workflow = update_workflow_state(handle, session.id, &app_name, category, &focus_kind, tick_secs);

    let _ = queries::insert_activity(
        &conn,
        session.id,
        &app_name,
        &window.window_title,
        category.as_str(),
    );

    if let Ok(stats) = queries::get_session_stats(&conn, session.id) {
        let update = SessionUpdate {
            session_id: session.id,
            duration_secs: stats.total_duration_secs,
            current_activity: category.as_str().to_string(),
            current_app: app_name.clone(),
            ai_assisted_secs: stats.ai_assisted_secs,
            manual_coding_secs: stats.manual_coding_secs,
            non_coding_secs: stats.non_coding_secs,
            workflow_state: workflow.workflow_state,
            context_switches_10m: workflow.context_switches_10m,
            ai_streak_secs: workflow.ai_streak_secs,
            coding_streak_secs: workflow.coding_streak_secs,
            prompt_loop_score: workflow.prompt_loop_score,
        };
        debug!(
            "Session update: {}s total, {} active",
            update.duration_secs, update.current_app
        );
        let _ = handle.emit("session-update", &update);
        update_tray_tooltip(handle, stats.total_duration_secs);
    }

    update_break_state(handle, category, tick_secs);

    *summary_tick += 1;
    let flush_every = ((300 / polling_secs.max(1)) as u32).max(1);
    if *summary_tick >= flush_every {
        let _ = queries::upsert_daily_summary(&conn);
        *summary_tick = 0;
    }
}

fn detect_activity(window: &detector::DetectedWindow) -> (classifier::ActivityCategory, String) {
    let mut category = classifier::classify(window);
    let mut app_name = window.app_name.clone();

    if category != classifier::ActivityCategory::AiAssisted {
        if let Some(tool_name) = detector::find_running_ai_tool() {
            category = classifier::ActivityCategory::AiAssisted;
            app_name = format!("{} (bg)", tool_name);
        }
    }

    (category, app_name)
}

fn classify_focus_kind(
    window: &detector::DetectedWindow,
    category: classifier::ActivityCategory,
    app_name: &str,
) -> String {
    let app = window.app_name.to_lowercase();
    let title = window.window_title.to_lowercase();
    let app_name_lower = app_name.to_lowercase();

    if app_name_lower.contains("(bg)") {
        return "agent_background".into();
    }
    if app.contains("chrome")
        || app.contains("arc")
        || app.contains("brave")
        || app.contains("edge")
        || app.contains("firefox")
        || app.contains("safari")
    {
        return if category == classifier::ActivityCategory::AiAssisted {
            "browser_ai".into()
        } else {
            "browser_other".into()
        };
    }
    if app.contains("terminal")
        || app.contains("iterm")
        || app.contains("warp")
        || app.contains("ghostty")
        || app.contains("kitty")
        || app.contains("alacritty")
        || app.contains("hyper")
    {
        if title.contains("claude")
            || title.contains("aider")
            || title.contains("codex")
            || title.contains("copilot")
        {
            return "terminal_ai".into();
        }
        if title.contains("nvim")
            || title.contains("vim")
            || title.contains("helix")
            || title.contains("emacs")
        {
            return "terminal_editor".into();
        }
        return "terminal_shell".into();
    }
    if app.contains("code") || app.contains("zed") || app.contains("visual studio") {
        return if category == classifier::ActivityCategory::AiAssisted {
            "editor_ai".into()
        } else {
            "editor".into()
        };
    }
    if app.contains("idea")
        || app.contains("intellij")
        || app.contains("pycharm")
        || app.contains("webstorm")
        || app.contains("goland")
        || app.contains("rustrover")
        || app.contains("clion")
    {
        return if category == classifier::ActivityCategory::AiAssisted {
            "ide_ai".into()
        } else {
            "ide".into()
        };
    }

    match category {
        classifier::ActivityCategory::AiAssisted => "ai_misc".into(),
        classifier::ActivityCategory::ManualCoding => "manual_misc".into(),
        classifier::ActivityCategory::NonCoding => "other".into(),
    }
}

fn update_workflow_state(
    handle: &AppHandle,
    session_id: i64,
    app_name: &str,
    category: classifier::ActivityCategory,
    focus_kind: &str,
    tick_secs: i64,
) -> WorkflowSignals {
    let state = handle.state::<Arc<MonitorState>>();
    let mut workflow = match state.workflow.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    };

    if workflow.current_session_id != Some(session_id) {
        *workflow = WorkflowRuntime::default();
        workflow.current_session_id = Some(session_id);
    }

    let is_switch = workflow
        .last_app_name
        .as_ref()
        .map(|last| last != app_name)
        .unwrap_or(false);
    if is_switch {
        workflow.context_switches_10m += 1;
    }

    let sample = WorkflowSample {
        app_name: app_name.to_string(),
        category,
        focus_kind: focus_kind.to_string(),
        tick_secs,
    };
    workflow.recent_samples.push_back(sample);
    trim_recent_samples(&mut workflow.recent_samples, 600);

    workflow.last_app_name = Some(app_name.to_string());
    workflow.last_focus_kind = Some(focus_kind.to_string());
    workflow.last_category = Some(category);

    workflow.context_switches_10m = compute_context_switches(&workflow.recent_samples);
    workflow.ai_streak_secs = if category == classifier::ActivityCategory::AiAssisted {
        workflow.ai_streak_secs + tick_secs
    } else {
        0
    };
    workflow.coding_streak_secs = if category == classifier::ActivityCategory::NonCoding {
        0
    } else {
        workflow.coding_streak_secs + tick_secs
    };

    derive_workflow_signals(&workflow)
}

fn trim_recent_samples(samples: &mut VecDeque<WorkflowSample>, max_secs: i64) {
    let mut total: i64 = samples.iter().map(|sample| sample.tick_secs).sum();
    while total > max_secs {
        if let Some(front) = samples.pop_front() {
            total -= front.tick_secs;
        } else {
            break;
        }
    }
}

fn compute_context_switches(samples: &VecDeque<WorkflowSample>) -> i64 {
    let mut previous: Option<&str> = None;
    let mut switches = 0;
    for sample in samples {
        let current = sample.app_name.as_str();
        if let Some(last) = previous {
            if last != current {
                switches += 1;
            }
        }
        previous = Some(current);
    }
    switches
}

fn derive_workflow_signals(workflow: &WorkflowRuntime) -> WorkflowSignals {
    let focus_kinds: HashSet<&str> = workflow
        .recent_samples
        .iter()
        .map(|sample| sample.focus_kind.as_str())
        .collect();
    let ai_samples = workflow
        .recent_samples
        .iter()
        .filter(|sample| sample.category == classifier::ActivityCategory::AiAssisted)
        .count() as i64;
    let manual_samples = workflow
        .recent_samples
        .iter()
        .filter(|sample| sample.category == classifier::ActivityCategory::ManualCoding)
        .count() as i64;

    let prompt_loop_score = (workflow.context_switches_10m * 6)
        + (workflow.ai_streak_secs / 60)
        + (ai_samples * 2)
        + if focus_kinds.contains("browser_ai") || focus_kinds.contains("terminal_ai") {
            8
        } else {
            0
        };

    let workflow_state = if workflow.context_switches_10m >= 12 {
        "context_switch_storm"
    } else if workflow.ai_streak_secs >= 3600
        && (focus_kinds.contains("terminal_ai") || focus_kinds.contains("agent_background"))
    {
        "agent_shepherding"
    } else if workflow.ai_streak_secs >= 2400
        && manual_samples > 0
        && ai_samples > 0
        && workflow.context_switches_10m >= 5
    {
        "prompt_edit_loop"
    } else if workflow.coding_streak_secs >= 5400 && workflow.context_switches_10m <= 3 {
        "deep_flow"
    } else if focus_kinds.contains("browser_ai")
        && (focus_kinds.contains("editor") || focus_kinds.contains("editor_ai"))
        && (focus_kinds.contains("terminal_ai") || focus_kinds.contains("agent_background"))
    {
        "research_spiral"
    } else if focus_kinds.contains("terminal_ai") || focus_kinds.contains("agent_background") {
        "agent_guided"
    } else if focus_kinds.contains("editor") || focus_kinds.contains("ide") {
        "implementation"
    } else {
        "settled"
    }
    .to_string();

    WorkflowSignals {
        workflow_state,
        context_switches_10m: workflow.context_switches_10m,
        ai_streak_secs: workflow.ai_streak_secs,
        coding_streak_secs: workflow.coding_streak_secs,
        prompt_loop_score: prompt_loop_score.min(100),
    }
}

fn should_auto_start(handle: &AppHandle, category: classifier::ActivityCategory) -> bool {
    if category == classifier::ActivityCategory::NonCoding {
        return false;
    }
    handle
        .try_state::<Arc<SettingsState>>()
        .and_then(|settings| settings.get_bool("auto_start_on_coding"))
        .unwrap_or(false)
}

fn maybe_auto_pause(handle: &AppHandle, idle_secs: Option<u64>) -> bool {
    let Some(idle_secs) = idle_secs else {
        return false;
    };
    let Some(settings) = handle.try_state::<Arc<SettingsState>>() else {
        return false;
    };

    let threshold = settings.get_i64("idle_threshold_mins").unwrap_or(5) * 60;
    if (idle_secs as i64) < threshold {
        return false;
    }

    let db = handle.state::<DbState>();
    if let Ok(conn) = db.conn.lock() {
        if let Ok(Some(session)) = queries::get_active_session(&conn) {
            if session.status == "active" {
                info!(
                    "Auto-pausing: {}s idle exceeds {}s threshold",
                    idle_secs, threshold
                );
                let state = handle.state::<Arc<MonitorState>>();
                state.is_paused.store(true, Ordering::SeqCst);
                state.auto_paused.store(true, Ordering::SeqCst);
                let _ = queries::pause_session(&conn, session.id);
                let _ = handle.emit("session-auto-paused", ());
                return true;
            }
        }
    }
    false
}

fn maybe_auto_resume(handle: &AppHandle, idle_secs: Option<u64>) -> bool {
    let Some(idle_secs) = idle_secs else {
        return false;
    };
    let Some(settings) = handle.try_state::<Arc<SettingsState>>() else {
        return false;
    };

    let threshold = settings.get_i64("idle_threshold_mins").unwrap_or(5) * 60;
    if (idle_secs as i64) >= threshold {
        return false;
    }

    let db = handle.state::<DbState>();
    if let Ok(conn) = db.conn.lock() {
        if let Ok(Some(session)) = queries::get_active_session(&conn) {
            let state = handle.state::<Arc<MonitorState>>();
            state.is_paused.store(false, Ordering::SeqCst);
            state.auto_paused.store(false, Ordering::SeqCst);
            let _ = queries::resume_session(&conn, session.id);
            info!("Auto-resumed after idle");
            let _ = handle.emit("session-auto-resumed", ());
            return true;
        }
    }
    false
}

fn maybe_auto_stop(handle: &AppHandle, idle_secs: Option<u64>) -> bool {
    let Some(idle_secs) = idle_secs else {
        return false;
    };
    let Some(settings) = handle.try_state::<Arc<SettingsState>>() else {
        return false;
    };
    if !settings.get_bool("auto_stop_on_idle").unwrap_or(false) {
        return false;
    }

    let threshold = settings.get_i64("idle_threshold_mins").unwrap_or(5) * 60;
    let stop_after = threshold * 2;
    if (idle_secs as i64) < stop_after {
        return false;
    }

    let db = handle.state::<DbState>();
    if let Ok(conn) = db.conn.lock() {
        if let Ok(Some(session)) = queries::get_active_session(&conn) {
            let _ = queries::end_session(&conn, session.id);
            clear_runtime_state(handle);
            info!("Auto-stopped session after extended idle");
            let _ = handle.emit("session-stopped", session.id);
            return true;
        }
    }
    false
}

fn update_break_state(
    handle: &AppHandle,
    category: classifier::ActivityCategory,
    tick_secs: i64,
) {
    let monitor = handle.state::<Arc<MonitorState>>();
    if category == classifier::ActivityCategory::AiAssisted {
        let prev = monitor
            .continuous_ai_secs
            .fetch_add(tick_secs, Ordering::SeqCst);
        let current = prev + tick_secs;

        if let Some(settings) = handle.try_state::<Arc<SettingsState>>() {
            let enabled = settings.get_bool("break_enforcer_enabled").unwrap_or(true);
            let interval_mins = settings.get_i64("break_interval_mins").unwrap_or(25);
            let threshold = interval_mins * 60;
            if enabled && threshold > 0 && current >= threshold {
                let break_duration = settings.get_i64("break_duration_mins").unwrap_or(5);
                let _ = handle.emit(
                    "break-reminder",
                    serde_json::json!({
                        "continuousSecs": current,
                        "breakDurationMins": break_duration,
                    }),
                );
                monitor.continuous_ai_secs.store(0, Ordering::SeqCst);
            }
        }
    } else {
        monitor.continuous_ai_secs.store(0, Ordering::SeqCst);
    }
}

fn update_tray_tooltip(handle: &AppHandle, total: i64) {
    let tooltip = if total <= 0 {
        "VibeCheck".to_string()
    } else {
        let h = total / 3600;
        let m = (total % 3600) / 60;
        if h > 0 {
            format!("VibeCheck  {}h {}m", h, m)
        } else {
            format!("VibeCheck  {}m", m)
        }
    };
    if let Some(tray) = handle.tray_by_id(&TrayIconId::new("main-tray")) {
        let _ = tray.set_tooltip(Some(&tooltip));
    }
}

#[allow(dead_code)]
pub fn stop_monitoring(app_handle: &AppHandle) {
    let monitor_state = app_handle.state::<Arc<MonitorState>>();
    monitor_state.is_running.store(false, Ordering::SeqCst);
    clear_runtime_state(app_handle);
    info!("Background monitoring stopped");
}

#[allow(dead_code)]
pub fn clear_runtime_state(handle: &AppHandle) {
    let state = handle.state::<Arc<MonitorState>>();
    state.is_paused.store(false, Ordering::SeqCst);
    state.auto_paused.store(false, Ordering::SeqCst);
    state.continuous_ai_secs.store(0, Ordering::SeqCst);
    if let Ok(mut workflow) = state.workflow.lock() {
        *workflow = WorkflowRuntime::default();
    }
    update_tray_tooltip(handle, 0);
}
