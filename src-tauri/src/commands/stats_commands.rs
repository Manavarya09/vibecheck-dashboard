use tauri::State;

use crate::db::models::{DailySummary, Session, SessionStats};
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
