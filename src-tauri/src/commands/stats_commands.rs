use tauri::State;

use crate::db::models::{ActivityEntry, DailySummary, Session, SessionStats};
use crate::db::queries;
use crate::db::DbState;
use crate::error::AppError;

#[tauri::command]
pub fn get_session_stats(db: State<DbState>, session_id: i64) -> Result<SessionStats, AppError> {
    let conn = db.conn.lock().map_err(|e| AppError::Session(e.to_string()))?;
    queries::get_session_stats(&conn, session_id)
}

#[tauri::command]
pub fn get_today_summary(db: State<DbState>) -> Result<DailySummary, AppError> {
    let conn = db.conn.lock().map_err(|e| AppError::Session(e.to_string()))?;
    queries::get_today_summary(&conn)
}

#[tauri::command]
pub fn get_recent_sessions(
    db: State<DbState>,
    limit: Option<i64>,
) -> Result<Vec<Session>, AppError> {
    let conn = db.conn.lock().map_err(|e| AppError::Session(e.to_string()))?;
    queries::get_recent_sessions(&conn, limit.unwrap_or(10))
}

#[tauri::command]
pub fn get_recent_activity(
    db: State<DbState>,
    session_id: i64,
    limit: Option<i64>,
) -> Result<Vec<ActivityEntry>, AppError> {
    let conn = db.conn.lock().map_err(|e| AppError::Session(e.to_string()))?;
    queries::get_recent_activity(&conn, session_id, limit.unwrap_or(20))
}

#[tauri::command]
pub fn get_app_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}
