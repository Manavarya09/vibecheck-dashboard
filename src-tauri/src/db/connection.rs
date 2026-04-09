use rusqlite::Connection;
use std::path::PathBuf;
use std::sync::Mutex;

pub struct DbState {
    pub conn: Mutex<Connection>,
}

const SCHEMA: &str = r#"
    PRAGMA journal_mode = WAL;

    CREATE TABLE IF NOT EXISTS sessions (
        id            INTEGER PRIMARY KEY AUTOINCREMENT,
        started_at    TEXT NOT NULL,
        ended_at      TEXT,
        duration_secs INTEGER NOT NULL DEFAULT 0,
        status        TEXT NOT NULL DEFAULT 'active'
                      CHECK(status IN ('active', 'paused', 'completed'))
    );

    CREATE TABLE IF NOT EXISTS activity_entries (
        id            INTEGER PRIMARY KEY AUTOINCREMENT,
        session_id    INTEGER NOT NULL REFERENCES sessions(id),
        timestamp     TEXT NOT NULL,
        app_name      TEXT NOT NULL,
        window_title  TEXT NOT NULL DEFAULT '',
        category      TEXT NOT NULL
                      CHECK(category IN ('ai_assisted', 'manual_coding', 'non_coding')),
        duration_secs INTEGER NOT NULL DEFAULT 5
    );

    CREATE TABLE IF NOT EXISTS daily_summaries (
        id                 INTEGER PRIMARY KEY AUTOINCREMENT,
        date               TEXT NOT NULL UNIQUE,
        total_secs         INTEGER NOT NULL DEFAULT 0,
        ai_assisted_secs   INTEGER NOT NULL DEFAULT 0,
        manual_coding_secs INTEGER NOT NULL DEFAULT 0,
        non_coding_secs    INTEGER NOT NULL DEFAULT 0,
        session_count      INTEGER NOT NULL DEFAULT 0
    );

    CREATE INDEX IF NOT EXISTS idx_activity_session
        ON activity_entries(session_id);
    CREATE INDEX IF NOT EXISTS idx_activity_timestamp
        ON activity_entries(timestamp);
    CREATE INDEX IF NOT EXISTS idx_sessions_started
        ON sessions(started_at);
"#;

pub fn init_db(data_dir: PathBuf) -> Result<DbState, rusqlite::Error> {
    std::fs::create_dir_all(&data_dir).ok();
    let db_path = data_dir.join("vibecheck.db");
    let conn = Connection::open(db_path)?;
    conn.execute_batch(SCHEMA)?;
    Ok(DbState {
        conn: Mutex::new(conn),
    })
}
