use chrono::Timelike;
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

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ResearchSessionReport {
    session_id: i64,
    started_at: String,
    ended_at: Option<String>,
    duration_secs: i64,
    ai_assisted_secs: i64,
    manual_coding_secs: i64,
    non_coding_secs: i64,
    ai_share_pct: i64,
    app_switches: i64,
    distinct_apps: i64,
    browser_ai_ticks: i64,
    terminal_ai_ticks: i64,
    long_session_flag: bool,
    started_hour_local: i64,
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

#[tauri::command]
pub fn export_research_report(db: State<DbState>, format: String) -> Result<String, AppError> {
    let conn = db
        .conn
        .lock()
        .map_err(|e| AppError::Session(e.to_string()))?;

    let mut stmt = conn.prepare(
        "SELECT id, started_at, ended_at, duration_secs
         FROM sessions
         WHERE status = 'completed'
         ORDER BY id",
    )?;

    let mut reports = Vec::new();
    let rows = stmt.query_map([], |row| {
        Ok((
            row.get::<_, i64>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, Option<String>>(2)?,
            row.get::<_, i64>(3)?,
        ))
    })?;

    for row in rows {
        let (session_id, started_at, ended_at, duration_secs) = row?;

        let mut activity_stmt = conn.prepare(
            "SELECT app_name, category FROM activity_entries
             WHERE session_id = ?1 ORDER BY id",
        )?;
        let activities = activity_stmt
            .query_map(params![session_id], |activity_row| {
                Ok((
                    activity_row.get::<_, String>(0)?,
                    activity_row.get::<_, String>(1)?,
                ))
            })?
            .collect::<Result<Vec<_>, _>>()?;

        let mut ai_assisted_secs = 0;
        let mut manual_coding_secs = 0;
        let mut non_coding_secs = 0;
        let mut app_switches = 0;
        let mut last_app: Option<String> = None;
        let mut distinct_apps = std::collections::HashSet::new();
        let mut browser_ai_ticks = 0;
        let mut terminal_ai_ticks = 0;

        for (app_name, category) in &activities {
            distinct_apps.insert(app_name.clone());

            if let Some(last) = &last_app {
                if last != app_name {
                    app_switches += 1;
                }
            }
            last_app = Some(app_name.clone());

            match category.as_str() {
                "ai_assisted" => {
                    ai_assisted_secs += 5;
                    let lower = app_name.to_lowercase();
                    if lower.contains("chrome")
                        || lower.contains("arc")
                        || lower.contains("brave")
                        || lower.contains("firefox")
                        || lower.contains("safari")
                        || lower.contains("edge")
                    {
                        browser_ai_ticks += 1;
                    }
                    if lower.contains("terminal")
                        || lower.contains("iterm")
                        || lower.contains("warp")
                        || lower.contains("ghostty")
                        || lower.contains("kitty")
                        || lower.contains("codex")
                        || lower.contains("claude")
                        || lower.contains("aider")
                    {
                        terminal_ai_ticks += 1;
                    }
                }
                "manual_coding" => manual_coding_secs += 5,
                _ => non_coding_secs += 5,
            }
        }

        let started_hour_local = chrono::DateTime::parse_from_rfc3339(&started_at)
            .map(|dt| dt.with_timezone(&chrono::Local).hour() as i64)
            .unwrap_or_default();

        let ai_share_pct = if duration_secs > 0 {
            ((ai_assisted_secs as f64 / duration_secs as f64) * 100.0).round() as i64
        } else {
            0
        };

        reports.push(ResearchSessionReport {
            session_id,
            started_at,
            ended_at,
            duration_secs,
            ai_assisted_secs,
            manual_coding_secs,
            non_coding_secs,
            ai_share_pct,
            app_switches,
            distinct_apps: distinct_apps.len() as i64,
            browser_ai_ticks,
            terminal_ai_ticks,
            long_session_flag: duration_secs >= 7200,
            started_hour_local,
        });
    }

    match format.as_str() {
        "csv" => Ok(research_to_csv(&reports)),
        _ => serde_json::to_string_pretty(&reports).map_err(|e| AppError::Session(e.to_string())),
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

fn research_to_csv(reports: &[ResearchSessionReport]) -> String {
    let mut out = String::from(
        "session_id,started_at,ended_at,duration_secs,ai_assisted_secs,manual_coding_secs,non_coding_secs,ai_share_pct,app_switches,distinct_apps,browser_ai_ticks,terminal_ai_ticks,long_session_flag,started_hour_local\n",
    );
    for report in reports {
        out.push_str(&format!(
            "{},{},{},{},{},{},{},{},{},{},{},{},{},{}\n",
            report.session_id,
            report.started_at,
            report.ended_at.as_deref().unwrap_or(""),
            report.duration_secs,
            report.ai_assisted_secs,
            report.manual_coding_secs,
            report.non_coding_secs,
            report.ai_share_pct,
            report.app_switches,
            report.distinct_apps,
            report.browser_ai_ticks,
            report.terminal_ai_ticks,
            report.long_session_flag,
            report.started_hour_local
        ));
    }
    out
}
