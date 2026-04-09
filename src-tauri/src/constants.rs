/// Polling interval for active window detection in seconds.
pub const POLLING_INTERVAL_SECS: u64 = 5;

/// Duration of each activity entry in seconds (matches polling interval).
pub const ACTIVITY_ENTRY_DURATION_SECS: i64 = 5;

/// Number of polling ticks before updating daily summary (5s * 60 = 5 minutes).
pub const SUMMARY_UPDATE_INTERVAL: u32 = 60;

/// Idle detection threshold: consecutive non-coding seconds before auto-pause.
pub const IDLE_THRESHOLD_SECS: u64 = 300;

/// Maximum number of recent sessions returned by default.
pub const DEFAULT_RECENT_SESSIONS_LIMIT: i64 = 10;

/// Maximum number of recent activity entries returned by default.
pub const DEFAULT_RECENT_ACTIVITY_LIMIT: i64 = 20;
