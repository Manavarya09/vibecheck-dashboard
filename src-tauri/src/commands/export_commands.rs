use rusqlite::params;
use serde::Serialize;
use tauri::State;

use crate::db::DbState;
use crate::error::AppError;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ExportSession {
    id: i64,
    started_at: String,
    ended_at: Option<String>,
    duration_secs: i64,
    status: String,
    activities: Vec<ExportActivity>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ExportActivity {
    timestamp: String,
    app_name: String,
    window_title: String,
    category: String,
    duration_secs: i64,
}

#[tauri::command]
pub fn export_data(db: State<DbState>, format: String) -> Result<String, AppError> {
    let conn = db
        .conn
        .lock()
        .map_err(|e| AppError::Session(e.to_string()))?;

    let mut session_stmt = conn.prepare(
        "SELECT id, started_at, ended_at, duration_secs, status FROM sessions ORDER BY id",
    )?;

    let sessions: Vec<ExportSession> = session_stmt
        .query_map([], |row| {
            Ok((
                row.get::<_, i64>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, Option<String>>(2)?,
                row.get::<_, i64>(3)?,
                row.get::<_, String>(4)?,
            ))
        })?
        .filter_map(|r| r.ok())
        .map(|(id, started_at, ended_at, duration_secs, status)| {
            let mut act_stmt = conn
                .prepare(
                    "SELECT timestamp, app_name, window_title, category, duration_secs
                 FROM activity_entries WHERE session_id = ?1 ORDER BY id",
                )
                .unwrap();

            let activities: Vec<ExportActivity> = act_stmt
                .query_map(params![id], |row| {
                    Ok(ExportActivity {
                        timestamp: row.get(0)?,
                        app_name: row.get(1)?,
                        window_title: row.get(2)?,
                        category: row.get(3)?,
                        duration_secs: row.get(4)?,
                    })
                })
                .unwrap()
                .filter_map(|r| r.ok())
                .collect();

            ExportSession {
                id,
                started_at,
                ended_at,
                duration_secs,
                status,
                activities,
            }
        })
        .collect();

    match format.as_str() {
        "csv" => Ok(to_csv(&sessions)),
        _ => serde_json::to_string_pretty(&sessions).map_err(|e| AppError::Session(e.to_string())),
    }
}

fn to_csv(sessions: &[ExportSession]) -> String {
    let mut out = String::from("session_id,started_at,ended_at,duration_secs,status,timestamp,app_name,window_title,category,activity_duration_secs\n");
    for s in sessions {
        if s.activities.is_empty() {
            out.push_str(&format!(
                "{},{},{},{},{},,,,,\n",
                s.id,
                s.started_at,
                s.ended_at.as_deref().unwrap_or(""),
                s.duration_secs,
                s.status,
            ));
        } else {
            for a in &s.activities {
                out.push_str(&format!(
                    "{},{},{},{},{},{},\"{}\",\"{}\",{},{}\n",
                    s.id,
                    s.started_at,
                    s.ended_at.as_deref().unwrap_or(""),
                    s.duration_secs,
                    s.status,
                    a.timestamp,
                    a.app_name.replace('"', "\"\""),
                    a.window_title.replace('"', "\"\""),
                    a.category,
                    a.duration_secs,
                ));
            }
        }
    }
    out
}
