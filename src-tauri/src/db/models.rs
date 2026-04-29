use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Session {
    pub id: i64,
    pub started_at: String,
    pub ended_at: Option<String>,
    pub duration_secs: i64,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityEntry {
    pub id: i64,
    pub session_id: i64,
    pub timestamp: String,
    pub app_name: String,
    pub window_title: String,
    pub category: String,
    pub duration_secs: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DailySummary {
    pub date: String,
    pub total_secs: i64,
    pub ai_assisted_secs: i64,
    pub manual_coding_secs: i64,
    pub non_coding_secs: i64,
    pub session_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionStats {
    pub total_duration_secs: i64,
    pub ai_assisted_secs: i64,
    pub manual_coding_secs: i64,
    pub non_coding_secs: i64,
    pub current_activity: Option<String>,
    pub current_app: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionUpdate {
    pub session_id: i64,
    pub duration_secs: i64,
    pub current_activity: String,
    pub current_app: String,
    pub ai_assisted_secs: i64,
    pub manual_coding_secs: i64,
    pub non_coding_secs: i64,
    pub workflow_state: String,
    pub context_switches_10m: i64,
    pub ai_streak_secs: i64,
    pub coding_streak_secs: i64,
    pub prompt_loop_score: i64,
}
