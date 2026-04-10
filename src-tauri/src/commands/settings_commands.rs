use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use tauri::State;

use crate::db::queries;
use crate::db::DbState;
use crate::error::AppError;

pub struct SettingsState {
    pub cache: RwLock<HashMap<String, String>>,
}

impl SettingsState {
    pub fn new(initial: HashMap<String, String>) -> Self {
        Self {
            cache: RwLock::new(initial),
        }
    }

    pub fn get(&self, key: &str) -> Option<String> {
        self.cache.read().ok()?.get(key).cloned()
    }

    pub fn get_i64(&self, key: &str) -> Option<i64> {
        self.get(key)?.parse().ok()
    }

    pub fn get_bool(&self, key: &str) -> Option<bool> {
        match self.get(key)?.as_str() {
            "true" => Some(true),
            "false" => Some(false),
            _ => None,
        }
    }
}

#[tauri::command]
pub fn get_settings(
    settings: State<Arc<SettingsState>>,
) -> Result<HashMap<String, String>, AppError> {
    settings
        .cache
        .read()
        .map(|c| c.clone())
        .map_err(|e| AppError::Session(e.to_string()))
}

#[tauri::command]
pub fn update_setting(
    db: State<DbState>,
    settings: State<Arc<SettingsState>>,
    key: String,
    value: String,
) -> Result<(), AppError> {
    let conn = db
        .conn
        .lock()
        .map_err(|e| AppError::Session(e.to_string()))?;
    queries::update_setting(&conn, &key, &value)?;

    if let Ok(mut cache) = settings.cache.write() {
        cache.insert(key, value);
    }
    Ok(())
}

#[tauri::command]
pub fn reset_settings(
    db: State<DbState>,
    settings: State<Arc<SettingsState>>,
) -> Result<HashMap<String, String>, AppError> {
    let conn = db
        .conn
        .lock()
        .map_err(|e| AppError::Session(e.to_string()))?;

    conn.execute("DELETE FROM settings", [])
        .map_err(AppError::Database)?;

    drop(conn);

    // Re-seed defaults by re-initializing
    let conn = db
        .conn
        .lock()
        .map_err(|e| AppError::Session(e.to_string()))?;
    let now = chrono::Utc::now().to_rfc3339();
    let defaults = [
        ("polling_interval_secs", "5"),
        ("break_interval_mins", "25"),
        ("break_duration_mins", "5"),
        ("idle_threshold_mins", "5"),
        ("auto_start_on_coding", "false"),
        ("auto_stop_on_idle", "false"),
        ("startup_at_login", "false"),
        ("break_enforcer_enabled", "true"),
    ];
    for (key, value) in defaults {
        conn.execute(
            "INSERT OR IGNORE INTO settings (key, value, updated_at) VALUES (?1, ?2, ?3)",
            rusqlite::params![key, value, now],
        )
        .map_err(AppError::Database)?;
    }

    let all = queries::get_all_settings(&conn)?;
    if let Ok(mut cache) = settings.cache.write() {
        *cache = all.clone();
    }
    Ok(all)
}
