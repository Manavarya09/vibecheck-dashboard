use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use tauri::tray::TrayIconId;
use tauri::{AppHandle, Emitter, Manager};
use tokio::time::{interval, Duration};

use log::{debug, info, warn};
use super::classifier;
use super::detector;
use crate::commands::settings_commands::SettingsState;
use crate::db::models::SessionUpdate;
use crate::db::queries;
use crate::db::DbState;

use std::sync::atomic::AtomicI64;

pub struct MonitorState {
    pub is_running: AtomicBool,
    pub is_paused: AtomicBool,
    pub auto_paused: AtomicBool,
    pub continuous_ai_secs: AtomicI64,
}

impl Default for MonitorState {
    fn default() -> Self {
        Self {
            is_running: AtomicBool::new(false),
            is_paused: AtomicBool::new(false),
            auto_paused: AtomicBool::new(false),
            continuous_ai_secs: AtomicI64::new(0),
        }
    }
}

pub fn start_monitoring(app_handle: AppHandle) {
    let monitor_state = app_handle.state::<Arc<MonitorState>>();
    monitor_state.is_running.store(true, Ordering::SeqCst);
    monitor_state.is_paused.store(false, Ordering::SeqCst);
    info!("Session monitoring started");

    let handle = app_handle.clone();
    tauri::async_runtime::spawn(async move {
        let mut tick = interval(Duration::from_secs(5));
        let mut summary_tick = 0u32;

        loop {
            tick.tick().await;

            let state = handle.state::<Arc<MonitorState>>();
            if !state.is_running.load(Ordering::SeqCst) {
                break;
            }
            let db = handle.state::<DbState>();

            if state.is_paused.load(Ordering::SeqCst) {
                // Check if auto-paused and user has returned
                if state.auto_paused.load(Ordering::SeqCst) {
                    if let Some(idle_secs) = detector::get_idle_seconds() {
                        if let Some(settings) = handle.try_state::<Arc<SettingsState>>() {
                            let threshold = settings.get_i64("idle_threshold_mins").unwrap_or(5) * 60;
                            if (idle_secs as i64) < threshold {
                                state.is_paused.store(false, Ordering::SeqCst);
                                state.auto_paused.store(false, Ordering::SeqCst);
                                info!("Auto-resumed: user activity detected after idle");
                                let _ = handle.emit("session-auto-resumed", ());

                                if let Ok(conn) = db.conn.lock() {
                                    if let Ok(Some(session)) = queries::get_active_session(&conn) {
                                        let _ = queries::resume_session(&conn, session.id);
                                    }
                                }
                            }
                        }
                    }
                }
                continue;
            }

            // Check for idle -- auto-pause if threshold exceeded
            if let Some(idle_secs) = detector::get_idle_seconds() {
                if let Some(settings) = handle.try_state::<Arc<SettingsState>>() {
                    let threshold = settings.get_i64("idle_threshold_mins").unwrap_or(5) * 60;
                    if (idle_secs as i64) >= threshold {
                        info!("Auto-pausing: {}s idle exceeds {}s threshold", idle_secs, threshold);
                        state.is_paused.store(true, Ordering::SeqCst);
                        state.auto_paused.store(true, Ordering::SeqCst);
                        let _ = handle.emit("session-auto-paused", ());

                        if let Ok(conn) = db.conn.lock() {
                            if let Ok(Some(session)) = queries::get_active_session(&conn) {
                                let _ = queries::pause_session(&conn, session.id);
                            }
                        }
                        continue;
                    }
                }
            }
            let conn = match db.conn.lock() {
                Ok(c) => c,
                Err(_) => continue,
            };

            let session = match queries::get_active_session(&conn) {
                Ok(Some(s)) if s.status == "active" => s,
                _ => continue,
            };

            let window = match detector::get_active_window_info() {
                Ok(w) => w,
                Err(e) => {
                    warn!("Window detection failed: {}", e);
                    continue;
                }
            };

            let mut category = classifier::classify(&window);
            let mut app_name = window.app_name.clone();

            // If the focused window isn't already AI-assisted, check if an
            // AI coding tool is running in the background. The entire period
            // where Claude Code (or aider, codex) is active counts as
            // AI-assisted work -- not just the moments its window is focused.
            if category != classifier::ActivityCategory::AiAssisted {
                if let Some(tool_name) = detector::find_running_ai_tool() {
                    category = classifier::ActivityCategory::AiAssisted;
                    app_name = format!("{} (bg)", tool_name);
                }
            }

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
                };
                debug!("Session update: {}s total, {} active", update.duration_secs, update.current_app);
                let _ = handle.emit("session-update", &update);

                // Update tray tooltip with session duration
                let total = stats.total_duration_secs;
                let h = total / 3600;
                let m = (total % 3600) / 60;
                let tooltip = if h > 0 {
                    format!("VibeCheck  {}h {}m", h, m)
                } else {
                    format!("VibeCheck  {}m", m)
                };
                if let Some(tray) =
                    handle.tray_by_id(&TrayIconId::new("main-tray"))
                {
                    let _ = tray.set_tooltip(Some(&tooltip));
                }
            }

            // Break enforcer: track continuous AI coding time
            let monitor = handle.state::<Arc<MonitorState>>();
            if category == classifier::ActivityCategory::AiAssisted {
                let prev = monitor.continuous_ai_secs.fetch_add(5, Ordering::SeqCst);
                let current = prev + 5;

                // Check if we should trigger a break
                if let Some(settings) = handle.try_state::<Arc<SettingsState>>() {
                    let enabled = settings.get_bool("break_enforcer_enabled").unwrap_or(true);
                    let interval_mins = settings.get_i64("break_interval_mins").unwrap_or(25);
                    let threshold = interval_mins * 60;

                    if enabled && threshold > 0 && current >= threshold {
                        info!("Break reminder triggered after {}s of continuous AI coding", current);
                        let break_duration = settings.get_i64("break_duration_mins").unwrap_or(5);
                        let _ = handle.emit("break-reminder", serde_json::json!({
                            "continuousSecs": current,
                            "breakDurationMins": break_duration,
                        }));
                        monitor.continuous_ai_secs.store(0, Ordering::SeqCst);
                    }
                }
            } else {
                monitor.continuous_ai_secs.store(0, Ordering::SeqCst);
            }

            summary_tick += 1;
            if summary_tick >= 60 {
                let _ = queries::upsert_daily_summary(&conn);
                summary_tick = 0;
            }
        }
    });
}

pub fn stop_monitoring(app_handle: &AppHandle) {
    let monitor_state = app_handle.state::<Arc<MonitorState>>();
    monitor_state.is_running.store(false, Ordering::SeqCst);
    monitor_state.is_paused.store(false, Ordering::SeqCst);
    info!("Session monitoring stopped");

    // Reset tray tooltip
    if let Some(tray) = app_handle.tray_by_id(&TrayIconId::new("main-tray")) {
        let _ = tray.set_tooltip(Some("VibeCheck"));
    }
}
