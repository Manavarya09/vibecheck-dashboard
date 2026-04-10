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

#[cfg(test)]
mod tests {
    use super::*;

    fn test_db() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute_batch(
            "PRAGMA journal_mode = WAL;
            CREATE TABLE IF NOT EXISTS sessions (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                started_at TEXT NOT NULL,
                ended_at TEXT,
                duration_secs INTEGER NOT NULL DEFAULT 0,
                status TEXT NOT NULL DEFAULT 'active'
                CHECK(status IN ('active', 'paused', 'completed'))
            );
            CREATE TABLE IF NOT EXISTS activity_entries (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                session_id INTEGER NOT NULL REFERENCES sessions(id),
                timestamp TEXT NOT NULL,
                app_name TEXT NOT NULL,
                window_title TEXT NOT NULL DEFAULT '',
                category TEXT NOT NULL
                CHECK(category IN ('ai_assisted', 'manual_coding', 'non_coding')),
                duration_secs INTEGER NOT NULL DEFAULT 5
            );
            CREATE TABLE IF NOT EXISTS daily_summaries (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                date TEXT NOT NULL UNIQUE,
                total_secs INTEGER NOT NULL DEFAULT 0,
                ai_assisted_secs INTEGER NOT NULL DEFAULT 0,
                manual_coding_secs INTEGER NOT NULL DEFAULT 0,
                non_coding_secs INTEGER NOT NULL DEFAULT 0,
                session_count INTEGER NOT NULL DEFAULT 0
            );
            CREATE TABLE IF NOT EXISTS settings (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL,
                updated_at TEXT NOT NULL
            );",
        )
        .unwrap();
        conn
    }

    #[test]
    fn create_session_returns_active_session() {
        let conn = test_db();
        let session = create_session(&conn).unwrap();
        assert_eq!(session.status, "active");
        assert!(session.ended_at.is_none());
        assert!(session.id > 0);
    }

    #[test]
    fn get_active_session_finds_active() {
        let conn = test_db();
        let created = create_session(&conn).unwrap();
        let found = get_active_session(&conn).unwrap();
        assert!(found.is_some());
        assert_eq!(found.unwrap().id, created.id);
    }

    #[test]
    fn end_session_sets_completed() {
        let conn = test_db();
        let session = create_session(&conn).unwrap();
        end_session(&conn, session.id).unwrap();
        let found = get_active_session(&conn).unwrap();
        assert!(found.is_none());
    }

    #[test]
    fn pause_and_resume_session() {
        let conn = test_db();
        let session = create_session(&conn).unwrap();
        pause_session(&conn, session.id).unwrap();
        let paused = get_active_session(&conn).unwrap().unwrap();
        assert_eq!(paused.status, "paused");
        resume_session(&conn, session.id).unwrap();
        let resumed = get_active_session(&conn).unwrap().unwrap();
        assert_eq!(resumed.status, "active");
    }

    #[test]
    fn insert_and_query_activity() {
        let conn = test_db();
        let session = create_session(&conn).unwrap();
        insert_activity(&conn, session.id, "Claude", "", "ai_assisted").unwrap();
        insert_activity(&conn, session.id, "Code", "main.rs", "manual_coding").unwrap();
        insert_activity(&conn, session.id, "Slack", "#general", "non_coding").unwrap();
        let entries = get_recent_activity(&conn, session.id, 10).unwrap();
        assert_eq!(entries.len(), 3);
    }

    #[test]
    fn session_stats_aggregation() {
        let conn = test_db();
        let session = create_session(&conn).unwrap();
        for _ in 0..5 {
            insert_activity(&conn, session.id, "Claude", "", "ai_assisted").unwrap();
        }
        for _ in 0..3 {
            insert_activity(&conn, session.id, "Code", "", "manual_coding").unwrap();
        }
        let stats = get_session_stats(&conn, session.id).unwrap();
        assert_eq!(stats.ai_assisted_secs, 25); // 5 * 5s
        assert_eq!(stats.manual_coding_secs, 15); // 3 * 5s
        assert_eq!(stats.total_duration_secs, 40);
    }

    #[test]
    fn recent_sessions_ordering() {
        let conn = test_db();
        let s1 = create_session(&conn).unwrap();
        end_session(&conn, s1.id).unwrap();
        let s2 = create_session(&conn).unwrap();
        end_session(&conn, s2.id).unwrap();
        let recent = get_recent_sessions(&conn, 10).unwrap();
        assert_eq!(recent.len(), 2);
        assert!(recent[0].id > recent[1].id); // newest first
    }

    #[test]
    fn settings_crud() {
        let conn = test_db();
        // Seed settings
        let now = chrono::Utc::now().to_rfc3339();
        conn.execute(
            "INSERT INTO settings (key, value, updated_at) VALUES ('test_key', '42', ?1)",
            rusqlite::params![now],
        )
        .unwrap();

        let all = get_all_settings(&conn).unwrap();
        assert_eq!(all.get("test_key").unwrap(), "42");

        let val = get_setting(&conn, "test_key").unwrap();
        assert_eq!(val, "42");

        update_setting(&conn, "test_key", "99").unwrap();
        let val2 = get_setting(&conn, "test_key").unwrap();
        assert_eq!(val2, "99");
    }

    #[test]
    fn update_nonexistent_setting_fails() {
        let conn = test_db();
        let result = update_setting(&conn, "no_such_key", "value");
        assert!(result.is_err());
    }

    #[test]
    fn today_summary_empty() {
        let conn = test_db();
        let summary = get_today_summary(&conn).unwrap();
        assert_eq!(summary.total_secs, 0);
        assert_eq!(summary.session_count, 0);
    }
}

// --- Historical ---

pub fn get_daily_summaries(
    conn: &Connection,
    days: i64,
) -> Result<Vec<super::models::DailySummary>, AppError> {
    let mut stmt = conn.prepare(
        "SELECT date, total_secs, ai_assisted_secs, manual_coding_secs, non_coding_secs, session_count
         FROM daily_summaries
         WHERE date >= date('now', ?1)
         ORDER BY date ASC"
    )?;
    let offset = format!("-{} days", days);
    let summaries = stmt
        .query_map(params![offset], |row| {
            Ok(super::models::DailySummary {
                date: row.get(0)?,
                total_secs: row.get(1)?,
                ai_assisted_secs: row.get(2)?,
                manual_coding_secs: row.get(3)?,
                non_coding_secs: row.get(4)?,
                session_count: row.get(5)?,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;
    Ok(summaries)
}

// --- Settings ---

pub fn get_all_settings(
    conn: &Connection,
) -> Result<std::collections::HashMap<String, String>, AppError> {
    let mut stmt = conn.prepare("SELECT key, value FROM settings")?;
    let mut map = std::collections::HashMap::new();
    let rows = stmt.query_map([], |row| {
        Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
    })?;
    for row in rows {
        let (k, v) = row?;
        map.insert(k, v);
    }
    Ok(map)
}

pub fn update_setting(conn: &Connection, key: &str, value: &str) -> Result<(), AppError> {
    let now = Utc::now().to_rfc3339();
    let rows = conn.execute(
        "UPDATE settings SET value = ?1, updated_at = ?2 WHERE key = ?3",
        params![value, now, key],
    )?;
    if rows == 0 {
        return Err(AppError::Session(format!("Unknown setting: {}", key)));
    }
    Ok(())
}

#[allow(dead_code)]
pub fn get_setting(conn: &Connection, key: &str) -> Result<String, AppError> {
    conn.query_row(
        "SELECT value FROM settings WHERE key = ?1",
        params![key],
        |row| row.get(0),
    )
    .map_err(AppError::Database)
}

#[allow(dead_code)]
pub fn get_database_stats(conn: &Connection) -> Result<(i64, i64, i64), AppError> {
    let sessions: i64 = conn.query_row("SELECT COUNT(*) FROM sessions", [], |row| row.get(0))?;
    let activities: i64 = conn.query_row("SELECT COUNT(*) FROM activity_entries", [], |row| {
        row.get(0)
    })?;
    let days: i64 = conn.query_row("SELECT COUNT(*) FROM daily_summaries", [], |row| row.get(0))?;
    Ok((sessions, activities, days))
}

// --- Spending ---

#[allow(clippy::type_complexity)]
pub fn get_spending_rates(
    conn: &Connection,
) -> Result<Vec<(i64, String, String, f64, Option<String>)>, AppError> {
    let mut stmt = conn.prepare(
        "SELECT id, tool_name, rate_type, rate_value, billing_period FROM spending_rates ORDER BY tool_name"
    )?;
    let rows = stmt
        .query_map([], |row| {
            Ok((
                row.get::<_, i64>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, f64>(3)?,
                row.get::<_, Option<String>>(4)?,
            ))
        })?
        .collect::<Result<Vec<_>, _>>()?;
    Ok(rows)
}

pub fn upsert_spending_rate(
    conn: &Connection,
    tool_name: &str,
    rate_type: &str,
    rate_value: f64,
    billing_period: &str,
) -> Result<(), AppError> {
    let now = chrono::Utc::now().to_rfc3339();
    conn.execute(
        "INSERT INTO spending_rates (tool_name, rate_type, rate_value, billing_period, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5)
         ON CONFLICT(tool_name) DO UPDATE SET rate_type = ?2, rate_value = ?3, billing_period = ?4, updated_at = ?5",
        params![tool_name, rate_type, rate_value, billing_period, now],
    )?;
    Ok(())
}

pub fn delete_spending_rate(conn: &Connection, id: i64) -> Result<(), AppError> {
    conn.execute("DELETE FROM spending_rates WHERE id = ?1", params![id])?;
    Ok(())
}

// --- Budget ---

pub fn get_budget_configs(conn: &Connection) -> Result<Vec<(i64, String, f64, bool)>, AppError> {
    let mut stmt =
        conn.prepare("SELECT id, period, limit_amount, enabled FROM budget_config ORDER BY id")?;
    let rows = stmt
        .query_map([], |row| {
            Ok((
                row.get::<_, i64>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, f64>(2)?,
                row.get::<_, bool>(3)?,
            ))
        })?
        .collect::<Result<Vec<_>, _>>()?;
    Ok(rows)
}

pub fn upsert_budget(conn: &Connection, period: &str, limit_amount: f64) -> Result<(), AppError> {
    conn.execute(
        "INSERT INTO budget_config (period, limit_amount, enabled)
         VALUES (?1, ?2, 1)
         ON CONFLICT(id) DO UPDATE SET limit_amount = ?2",
        params![period, limit_amount],
    )?;
    Ok(())
}

// --- Notes & Tags ---

pub fn add_session_note(conn: &Connection, session_id: i64, note: &str) -> Result<(), AppError> {
    let now = Utc::now().to_rfc3339();
    conn.execute(
        "INSERT INTO session_notes (session_id, note, created_at) VALUES (?1, ?2, ?3)",
        params![session_id, note, now],
    )?;
    Ok(())
}

pub fn get_session_notes(
    conn: &Connection,
    session_id: i64,
) -> Result<Vec<(i64, String, String)>, AppError> {
    let mut stmt = conn.prepare(
        "SELECT id, note, created_at FROM session_notes WHERE session_id = ?1 ORDER BY id DESC",
    )?;
    let rows = stmt
        .query_map(params![session_id], |row| {
            Ok((
                row.get::<_, i64>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
            ))
        })?
        .collect::<Result<Vec<_>, _>>()?;
    Ok(rows)
}

pub fn add_session_tag(conn: &Connection, session_id: i64, tag: &str) -> Result<(), AppError> {
    conn.execute(
        "INSERT OR IGNORE INTO session_tags (session_id, tag) VALUES (?1, ?2)",
        params![session_id, tag],
    )?;
    Ok(())
}

pub fn get_session_tags(conn: &Connection, session_id: i64) -> Result<Vec<String>, AppError> {
    let mut stmt =
        conn.prepare("SELECT tag FROM session_tags WHERE session_id = ?1 ORDER BY tag")?;
    let rows = stmt
        .query_map(params![session_id], |row| row.get::<_, String>(0))?
        .collect::<Result<Vec<_>, _>>()?;
    Ok(rows)
}

pub fn remove_session_tag(conn: &Connection, session_id: i64, tag: &str) -> Result<(), AppError> {
    conn.execute(
        "DELETE FROM session_tags WHERE session_id = ?1 AND tag = ?2",
        params![session_id, tag],
    )?;
    Ok(())
}
