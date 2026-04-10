use tauri::{Manager, State};

use crate::db::models::{ActivityEntry, DailySummary, Session, SessionStats};
use crate::db::queries;
use crate::db::DbState;
use crate::error::AppError;

#[tauri::command]
pub fn get_session_stats(db: State<DbState>, session_id: i64) -> Result<SessionStats, AppError> {
    let conn = db
        .conn
        .lock()
        .map_err(|e| AppError::Session(e.to_string()))?;
    queries::get_session_stats(&conn, session_id)
}

#[tauri::command]
pub fn get_today_summary(db: State<DbState>) -> Result<DailySummary, AppError> {
    let conn = db
        .conn
        .lock()
        .map_err(|e| AppError::Session(e.to_string()))?;
    queries::get_today_summary(&conn)
}

#[tauri::command]
pub fn get_recent_sessions(
    db: State<DbState>,
    limit: Option<i64>,
) -> Result<Vec<Session>, AppError> {
    let conn = db
        .conn
        .lock()
        .map_err(|e| AppError::Session(e.to_string()))?;
    queries::get_recent_sessions(&conn, limit.unwrap_or(10))
}

#[tauri::command]
pub fn get_recent_activity(
    db: State<DbState>,
    session_id: i64,
    limit: Option<i64>,
) -> Result<Vec<ActivityEntry>, AppError> {
    let conn = db
        .conn
        .lock()
        .map_err(|e| AppError::Session(e.to_string()))?;
    queries::get_recent_activity(&conn, session_id, limit.unwrap_or(20))
}

#[tauri::command]
pub fn get_app_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

#[tauri::command]
pub fn get_heatmap_data(
    db: State<DbState>,
    days: Option<i64>,
) -> Result<Vec<DailySummary>, AppError> {
    let conn = db
        .conn
        .lock()
        .map_err(|e| AppError::Session(e.to_string()))?;
    queries::get_daily_summaries(&conn, days.unwrap_or(365))
}

#[tauri::command]
pub fn get_historical_stats(db: State<DbState>, days: i64) -> Result<Vec<DailySummary>, AppError> {
    let conn = db
        .conn
        .lock()
        .map_err(|e| AppError::Session(e.to_string()))?;
    queries::get_daily_summaries(&conn, days)
}

#[tauri::command]
pub fn add_session_note(db: State<DbState>, session_id: i64, note: String) -> Result<(), AppError> {
    let conn = db
        .conn
        .lock()
        .map_err(|e| AppError::Session(e.to_string()))?;
    queries::add_session_note(&conn, session_id, &note)
}

#[tauri::command]
pub fn get_session_notes(
    db: State<DbState>,
    session_id: i64,
) -> Result<Vec<(i64, String, String)>, AppError> {
    let conn = db
        .conn
        .lock()
        .map_err(|e| AppError::Session(e.to_string()))?;
    queries::get_session_notes(&conn, session_id)
}

#[tauri::command]
pub fn add_session_tag(db: State<DbState>, session_id: i64, tag: String) -> Result<(), AppError> {
    let conn = db
        .conn
        .lock()
        .map_err(|e| AppError::Session(e.to_string()))?;
    queries::add_session_tag(&conn, session_id, &tag)
}

#[tauri::command]
pub fn get_session_tags(db: State<DbState>, session_id: i64) -> Result<Vec<String>, AppError> {
    let conn = db
        .conn
        .lock()
        .map_err(|e| AppError::Session(e.to_string()))?;
    queries::get_session_tags(&conn, session_id)
}

#[tauri::command]
pub fn remove_session_tag(
    db: State<DbState>,
    session_id: i64,
    tag: String,
) -> Result<(), AppError> {
    let conn = db
        .conn
        .lock()
        .map_err(|e| AppError::Session(e.to_string()))?;
    queries::remove_session_tag(&conn, session_id, &tag)
}

#[tauri::command]
pub fn get_db_path(app_handle: tauri::AppHandle) -> Result<String, AppError> {
    let data_dir = app_handle
        .path()
        .app_data_dir()
        .map_err(|e| AppError::Session(e.to_string()))?;
    Ok(data_dir.join("vibecheck.db").to_string_lossy().to_string())
}
