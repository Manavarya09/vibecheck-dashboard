use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use tauri::{AppHandle, Emitter, Manager};
use tokio::time::{interval, Duration};

use super::classifier;
use super::detector;
use crate::db::models::SessionUpdate;
use crate::db::queries;
use crate::db::DbState;

pub struct MonitorState {
    pub is_running: AtomicBool,
    pub is_paused: AtomicBool,
}

impl Default for MonitorState {
    fn default() -> Self {
        Self {
            is_running: AtomicBool::new(false),
            is_paused: AtomicBool::new(false),
        }
    }
}

pub fn start_monitoring(app_handle: AppHandle) {
    let monitor_state = app_handle.state::<Arc<MonitorState>>();
    monitor_state.is_running.store(true, Ordering::SeqCst);
    monitor_state.is_paused.store(false, Ordering::SeqCst);

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
            if state.is_paused.load(Ordering::SeqCst) {
                continue;
            }

            let db = handle.state::<DbState>();
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
                Err(_) => continue,
            };

            let category = classifier::classify(&window);

            let _ = queries::insert_activity(
                &conn,
                session.id,
                &window.app_name,
                &window.window_title,
                category.as_str(),
            );

            if let Ok(stats) = queries::get_session_stats(&conn, session.id) {
                let update = SessionUpdate {
                    session_id: session.id,
                    duration_secs: stats.total_duration_secs,
                    current_activity: stats
                        .current_activity
                        .unwrap_or_else(|| "non_coding".to_string()),
                    current_app: stats.current_app.unwrap_or_default(),
                    ai_assisted_secs: stats.ai_assisted_secs,
                    manual_coding_secs: stats.manual_coding_secs,
                    non_coding_secs: stats.non_coding_secs,
                };
                let _ = handle.emit("session-update", &update);
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
}
