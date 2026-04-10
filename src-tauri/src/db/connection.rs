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

    CREATE TABLE IF NOT EXISTS settings (
        key        TEXT PRIMARY KEY,
        value      TEXT NOT NULL,
        updated_at TEXT NOT NULL
    );
"#;

const DEFAULT_SETTINGS: &[(&str, &str)] = &[
    ("polling_interval_secs", "5"),
    ("break_interval_mins", "25"),
    ("break_duration_mins", "5"),
    ("idle_threshold_mins", "5"),
    ("auto_start_on_coding", "false"),
    ("auto_stop_on_idle", "false"),
    ("startup_at_login", "false"),
    ("break_enforcer_enabled", "true"),
];

pub fn init_db(data_dir: PathBuf) -> Result<DbState, rusqlite::Error> {
    std::fs::create_dir_all(&data_dir).ok();
    let db_path = data_dir.join("vibecheck.db");
    let conn = Connection::open(db_path)?;
    conn.execute_batch(SCHEMA)?;
    seed_default_settings(&conn)?;
    Ok(DbState {
        conn: Mutex::new(conn),
    })
}

fn seed_default_settings(conn: &Connection) -> Result<(), rusqlite::Error> {
    let now = chrono::Utc::now().to_rfc3339();
    for (key, value) in DEFAULT_SETTINGS {
        conn.execute(
            "INSERT OR IGNORE INTO settings (key, value, updated_at) VALUES (?1, ?2, ?3)",
            rusqlite::params![key, value, now],
        )?;
    }
    Ok(())
}
