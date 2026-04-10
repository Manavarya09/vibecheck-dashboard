mod commands;
mod constants;
mod db;
mod error;
mod monitor;
mod permissions;
mod tray;

use std::sync::Arc;

use tauri::{Emitter, Manager};

use commands::export_commands;
use commands::spending_commands;
use commands::session_commands;
use commands::settings_commands;
use commands::stats_commands;
use db::connection;
use monitor::session::MonitorState;

#[tauri::command]
fn check_screen_recording_permission() -> bool {
    permissions::check_screen_recording()
}

#[tauri::command]
fn set_autostart(app_handle: tauri::AppHandle, enabled: bool) -> Result<(), String> {
    use tauri_plugin_autostart::ManagerExt;
    let manager = app_handle.autolaunch();
    if enabled {
        manager.enable().map_err(|e| e.to_string())?;
    } else {
        manager.disable().map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
fn get_autostart_enabled(app_handle: tauri::AppHandle) -> Result<bool, String> {
    use tauri_plugin_autostart::ManagerExt;
    app_handle.autolaunch().is_enabled().map_err(|e| e.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_autostart::init(
            tauri_plugin_autostart::MacosLauncher::LaunchAgent,
            None,
        ))
        .plugin(
            tauri_plugin_log::Builder::default()
                .level(log::LevelFilter::Info)
                .build(),
        )
        .invoke_handler(tauri::generate_handler![
            session_commands::start_session,
            session_commands::stop_session,
            session_commands::pause_session,
            session_commands::resume_session,
            session_commands::get_current_session,
            stats_commands::get_session_stats,
            stats_commands::get_today_summary,
            stats_commands::get_recent_sessions,
            stats_commands::get_recent_activity,
            stats_commands::get_app_version,
            settings_commands::get_settings,
            settings_commands::update_setting,
            settings_commands::reset_settings,
            session_commands::get_auto_start_enabled,
            check_screen_recording_permission,
            set_autostart,
            get_autostart_enabled,
            export_commands::export_data,
            stats_commands::get_heatmap_data,
            stats_commands::get_historical_stats,
            stats_commands::add_session_note,
            stats_commands::get_session_notes,
            stats_commands::add_session_tag,
            stats_commands::get_session_tags,
            stats_commands::remove_session_tag,
            stats_commands::get_db_path,
            spending_commands::get_spending_rates,
            spending_commands::upsert_spending_rate,
            spending_commands::delete_spending_rate,
            spending_commands::get_budget_configs,
            spending_commands::set_budget,
        ])
        .setup(|app| {
            let data_dir = app
                .path()
                .app_data_dir()
                .expect("failed to resolve app data directory");

            let db_state =
                connection::init_db(data_dir).expect("failed to initialize database");

            // Load settings into cache
            let initial_settings = {
                let conn = db_state.conn.lock().expect("db lock");
                crate::db::queries::get_all_settings(&conn).unwrap_or_default()
            };
            let settings_state = Arc::new(settings_commands::SettingsState::new(initial_settings));

            app.manage(db_state);
            app.manage(Arc::new(MonitorState::default()));
            app.manage(settings_state);

            // Check screen recording permission on startup
            if !permissions::check_screen_recording() {
                log::warn!("Screen Recording permission not granted -- window titles will be empty");
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.emit("permission-missing", "screen_recording");
                }
            }

            tray::setup_tray(app)?;

            // Close window to tray instead of quitting
            let main_window = app.get_webview_window("main").unwrap();
            let window_clone = main_window.clone();
            main_window.on_window_event(move |event| {
                if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                    api.prevent_close();
                    let _ = window_clone.hide();
                }
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
