use std::sync::atomic::Ordering;
use std::sync::Arc;

use tauri::{AppHandle, Manager, State};

use crate::db::models::Session;
use crate::db::queries;
use crate::db::DbState;
use crate::error::AppError;
use crate::monitor::session::MonitorState;

#[tauri::command]
pub fn start_session(db: State<DbState>, app_handle: AppHandle) -> Result<Session, AppError> {
    let conn = db
        .conn
        .lock()
        .map_err(|e| AppError::Session(e.to_string()))?;

    if let Some(existing) = queries::get_active_session(&conn)? {
        return Ok(existing);
    }

    let session = queries::create_session(&conn)?;
    drop(conn);

    let _ = app_handle;
    Ok(session)
}

#[tauri::command]
pub fn stop_session(db: State<DbState>, app_handle: AppHandle) -> Result<(), AppError> {
    let conn = db
        .conn
        .lock()
        .map_err(|e| AppError::Session(e.to_string()))?;

    if let Some(session) = queries::get_active_session(&conn)? {
        queries::end_session(&conn, session.id)?;
    }
    drop(conn);

    let monitor = app_handle.state::<Arc<MonitorState>>();
    monitor.is_paused.store(false, Ordering::SeqCst);
    monitor.auto_paused.store(false, Ordering::SeqCst);
    crate::monitor::session::clear_runtime_state(&app_handle);
    Ok(())
}

#[tauri::command]
pub fn pause_session(db: State<DbState>, app_handle: AppHandle) -> Result<(), AppError> {
    let conn = db
        .conn
        .lock()
        .map_err(|e| AppError::Session(e.to_string()))?;

    if let Some(session) = queries::get_active_session(&conn)? {
        queries::pause_session(&conn, session.id)?;
    }

    let monitor = app_handle.state::<Arc<MonitorState>>();
    monitor.is_paused.store(true, Ordering::SeqCst);
    Ok(())
}

#[tauri::command]
pub fn resume_session(db: State<DbState>, app_handle: AppHandle) -> Result<(), AppError> {
    let conn = db
        .conn
        .lock()
        .map_err(|e| AppError::Session(e.to_string()))?;

    if let Some(session) = queries::get_active_session(&conn)? {
        queries::resume_session(&conn, session.id)?;
    }

    let monitor = app_handle.state::<Arc<MonitorState>>();
    monitor.is_paused.store(false, Ordering::SeqCst);
    Ok(())
}

#[tauri::command]
pub fn get_current_session(db: State<DbState>) -> Result<Option<Session>, AppError> {
    let conn = db
        .conn
        .lock()
        .map_err(|e| AppError::Session(e.to_string()))?;
    queries::get_active_session(&conn)
}

#[tauri::command]
pub fn get_auto_start_enabled(
    settings: State<Arc<crate::commands::settings_commands::SettingsState>>,
) -> bool {
    settings.get_bool("auto_start_on_coding").unwrap_or(false)
}
