use chrono::{Local, Utc};
use rusqlite::{params, Connection};

use super::models::{ActivityEntry, DailySummary, Session, SessionStats};
use crate::error::AppError;

pub fn create_session(conn: &Connection) -> Result<Session, AppError> {
    let now = Utc::now().to_rfc3339();
    conn.execute(
        "INSERT INTO sessions (started_at, status) VALUES (?1, 'active')",
        params![now],
    )?;
    let id = conn.last_insert_rowid();
    Ok(Session {
        id,
        started_at: now,
        ended_at: None,
        duration_secs: 0,
        status: "active".to_string(),
    })
}

pub fn end_session(conn: &Connection, session_id: i64) -> Result<(), AppError> {
    let now = Utc::now().to_rfc3339();
    conn.execute(
        "UPDATE sessions SET ended_at = ?1, status = 'completed' WHERE id = ?2",
        params![now, session_id],
    )?;
    update_session_duration(conn, session_id)?;
    upsert_daily_summary(conn)?;
    Ok(())
}

pub fn pause_session(conn: &Connection, session_id: i64) -> Result<(), AppError> {
    conn.execute(
        "UPDATE sessions SET status = 'paused' WHERE id = ?1",
        params![session_id],
    )?;
    update_session_duration(conn, session_id)?;
    Ok(())
}

pub fn resume_session(conn: &Connection, session_id: i64) -> Result<(), AppError> {
    conn.execute(
        "UPDATE sessions SET status = 'active' WHERE id = ?1",
        params![session_id],
    )?;
    Ok(())
}

pub fn get_active_session(conn: &Connection) -> Result<Option<Session>, AppError> {
    let mut stmt = conn.prepare(
        "SELECT id, started_at, ended_at, duration_secs, status
         FROM sessions WHERE status IN ('active', 'paused')
         ORDER BY id DESC LIMIT 1",
    )?;
    let mut rows = stmt.query_map([], |row| {
        Ok(Session {
            id: row.get(0)?,
            started_at: row.get(1)?,
            ended_at: row.get(2)?,
            duration_secs: row.get(3)?,
            status: row.get(4)?,
        })
    })?;
    match rows.next() {
        Some(Ok(session)) => Ok(Some(session)),
        Some(Err(e)) => Err(AppError::Database(e)),
        None => Ok(None),
    }
}

pub fn insert_activity(
    conn: &Connection,
    session_id: i64,
    app_name: &str,
    window_title: &str,
    category: &str,
) -> Result<(), AppError> {
    let now = Utc::now().to_rfc3339();
    conn.execute(
        "INSERT INTO activity_entries (session_id, timestamp, app_name, window_title, category, duration_secs)
         VALUES (?1, ?2, ?3, ?4, ?5, 5)",
        params![session_id, now, app_name, window_title, category],
    )?;
    Ok(())
}

pub fn get_session_stats(conn: &Connection, session_id: i64) -> Result<SessionStats, AppError> {
    let total: i64 = conn.query_row(
        "SELECT COALESCE(SUM(duration_secs), 0) FROM activity_entries WHERE session_id = ?1",
        params![session_id],
        |row| row.get(0),
    )?;

    let ai: i64 = conn.query_row(
        "SELECT COALESCE(SUM(duration_secs), 0) FROM activity_entries
         WHERE session_id = ?1 AND category = 'ai_assisted'",
        params![session_id],
        |row| row.get(0),
    )?;

    let manual: i64 = conn.query_row(
        "SELECT COALESCE(SUM(duration_secs), 0) FROM activity_entries
         WHERE session_id = ?1 AND category = 'manual_coding'",
        params![session_id],
        |row| row.get(0),
    )?;

    let non_coding: i64 = conn.query_row(
        "SELECT COALESCE(SUM(duration_secs), 0) FROM activity_entries
         WHERE session_id = ?1 AND category = 'non_coding'",
        params![session_id],
        |row| row.get(0),
    )?;

    let latest: Option<(String, String)> = conn
        .query_row(
            "SELECT category, app_name FROM activity_entries
             WHERE session_id = ?1 ORDER BY id DESC LIMIT 1",
            params![session_id],
            |row| Ok((row.get(0)?, row.get(1)?)),
        )
        .ok();

    Ok(SessionStats {
        total_duration_secs: total,
        ai_assisted_secs: ai,
        manual_coding_secs: manual,
        non_coding_secs: non_coding,
        current_activity: latest.as_ref().map(|(cat, _)| cat.clone()),
        current_app: latest.map(|(_, app)| app),
    })
}

pub fn get_today_summary(conn: &Connection) -> Result<DailySummary, AppError> {
    let today = Local::now().format("%Y-%m-%d").to_string();

    let result = conn.query_row(
        "SELECT date, total_secs, ai_assisted_secs, manual_coding_secs, non_coding_secs, session_count
         FROM daily_summaries WHERE date = ?1",
        params![today],
        |row| {
            Ok(DailySummary {
                date: row.get(0)?,
                total_secs: row.get(1)?,
                ai_assisted_secs: row.get(2)?,
                manual_coding_secs: row.get(3)?,
                non_coding_secs: row.get(4)?,
                session_count: row.get(5)?,
            })
        },
    );

    match result {
        Ok(summary) => Ok(summary),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(DailySummary {
            date: today,
            total_secs: 0,
            ai_assisted_secs: 0,
            manual_coding_secs: 0,
            non_coding_secs: 0,
            session_count: 0,
        }),
        Err(e) => Err(AppError::Database(e)),
    }
}

pub fn get_recent_sessions(conn: &Connection, limit: i64) -> Result<Vec<Session>, AppError> {
    let mut stmt = conn.prepare(
        "SELECT id, started_at, ended_at, duration_secs, status
         FROM sessions ORDER BY id DESC LIMIT ?1",
    )?;
    let sessions = stmt
        .query_map(params![limit], |row| {
            Ok(Session {
                id: row.get(0)?,
                started_at: row.get(1)?,
                ended_at: row.get(2)?,
                duration_secs: row.get(3)?,
                status: row.get(4)?,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;
    Ok(sessions)
}

pub fn get_recent_activity(
    conn: &Connection,
    session_id: i64,
    limit: i64,
) -> Result<Vec<ActivityEntry>, AppError> {
    let mut stmt = conn.prepare(
        "SELECT id, session_id, timestamp, app_name, window_title, category, duration_secs
         FROM activity_entries
         WHERE session_id = ?1
         ORDER BY id DESC
         LIMIT ?2",
    )?;
    let entries = stmt
        .query_map(params![session_id, limit], |row| {
            Ok(ActivityEntry {
                id: row.get(0)?,
                session_id: row.get(1)?,
                timestamp: row.get(2)?,
                app_name: row.get(3)?,
                window_title: row.get(4)?,
                category: row.get(5)?,
                duration_secs: row.get(6)?,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;
    Ok(entries)
}

fn update_session_duration(conn: &Connection, session_id: i64) -> Result<(), AppError> {
    conn.execute(
        "UPDATE sessions SET duration_secs = (
            SELECT COALESCE(SUM(duration_secs), 0) FROM activity_entries WHERE session_id = ?1
         ) WHERE id = ?1",
        params![session_id],
    )?;
    Ok(())
}

pub fn upsert_daily_summary(conn: &Connection) -> Result<(), AppError> {
    let today = Local::now().format("%Y-%m-%d").to_string();

    conn.execute(
        "INSERT INTO daily_summaries (date, total_secs, ai_assisted_secs, manual_coding_secs, non_coding_secs, session_count)
         SELECT
            ?1,
            COALESCE(SUM(ae.duration_secs), 0),
            COALESCE(SUM(CASE WHEN ae.category = 'ai_assisted' THEN ae.duration_secs ELSE 0 END), 0),
            COALESCE(SUM(CASE WHEN ae.category = 'manual_coding' THEN ae.duration_secs ELSE 0 END), 0),
            COALESCE(SUM(CASE WHEN ae.category = 'non_coding' THEN ae.duration_secs ELSE 0 END), 0),
            (SELECT COUNT(*) FROM sessions WHERE date(started_at) = ?1)
         FROM activity_entries ae
         JOIN sessions s ON ae.session_id = s.id
         WHERE date(s.started_at) = ?1
         ON CONFLICT(date) DO UPDATE SET
            total_secs = excluded.total_secs,
            ai_assisted_secs = excluded.ai_assisted_secs,
            manual_coding_secs = excluded.manual_coding_secs,
            non_coding_secs = excluded.non_coding_secs,
            session_count = excluded.session_count",
        params![today],
    )?;
    Ok(())
}
