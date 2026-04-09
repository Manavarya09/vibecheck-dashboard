mod commands;
mod db;
mod error;
mod monitor;
mod tray;

use std::sync::Arc;

use tauri::Manager;

use commands::session_commands;
use commands::stats_commands;
use db::connection;
use monitor::session::MonitorState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            session_commands::start_session,
            session_commands::stop_session,
            session_commands::pause_session,
            session_commands::resume_session,
            session_commands::get_current_session,
            stats_commands::get_session_stats,
            stats_commands::get_today_summary,
            stats_commands::get_recent_sessions,
        ])
        .setup(|app| {
            let data_dir = app
                .path()
                .app_data_dir()
                .expect("failed to resolve app data directory");

            let db_state =
                connection::init_db(data_dir).expect("failed to initialize database");
            app.manage(db_state);
            app.manage(Arc::new(MonitorState::default()));

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
