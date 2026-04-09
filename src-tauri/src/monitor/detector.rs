use crate::error::AppError;

pub struct DetectedWindow {
    pub app_name: String,
    pub window_title: String,
    pub process_id: u64,
}

pub fn get_active_window_info() -> Result<DetectedWindow, AppError> {
    match active_win_pos_rs::get_active_window() {
        Ok(window) => Ok(DetectedWindow {
            app_name: window.app_name,
            window_title: window.title,
            process_id: window.process_id,
        }),
        Err(()) => Err(AppError::WindowDetection(
            "Failed to get active window".to_string(),
        )),
    }
}

/// Check if any AI coding tool is running as a child process of the given PID.
/// This is the fallback for when Screen Recording permission isn't granted
/// and window titles come back empty.
#[cfg(target_os = "macos")]
pub fn has_ai_child_process(parent_pid: u64) -> Option<String> {
    let output = std::process::Command::new("pgrep")
        .args(["-P", &parent_pid.to_string()])
        .output()
        .ok()?;

    let child_pids = String::from_utf8_lossy(&output.stdout);
    for pid in child_pids.lines() {
        let pid = pid.trim();
        if pid.is_empty() {
            continue;
        }

        // Get the command name for each child process
        if let Ok(ps_output) = std::process::Command::new("ps")
            .args(["-p", pid, "-o", "comm="])
            .output()
        {
            let comm = String::from_utf8_lossy(&ps_output.stdout)
                .trim()
                .to_lowercase();

            if comm.contains("claude") {
                return Some("Claude Code".to_string());
            }
            if comm.contains("aider") {
                return Some("aider".to_string());
            }
            if comm.contains("codex") {
                return Some("Codex".to_string());
            }
            if comm.contains("copilot") {
                return Some("Copilot".to_string());
            }
        }

        // Also check grandchildren (claude might be wrapped in node/python)
        if let Ok(grandchild_output) = std::process::Command::new("pgrep")
            .args(["-P", pid])
            .output()
        {
            let grandchild_pids = String::from_utf8_lossy(&grandchild_output.stdout);
            for gpid in grandchild_pids.lines() {
                let gpid = gpid.trim();
                if gpid.is_empty() {
                    continue;
                }
                if let Ok(ps_out) = std::process::Command::new("ps")
                    .args(["-p", gpid, "-o", "comm="])
                    .output()
                {
                    let gcomm = String::from_utf8_lossy(&ps_out.stdout)
                        .trim()
                        .to_lowercase();

                    if gcomm.contains("claude") {
                        return Some("Claude Code".to_string());
                    }
                    if gcomm.contains("aider") {
                        return Some("aider".to_string());
                    }
                    if gcomm.contains("codex") {
                        return Some("Codex".to_string());
                    }
                }
            }
        }
    }

    None
}

#[cfg(not(target_os = "macos"))]
pub fn has_ai_child_process(_parent_pid: u64) -> Option<String> {
    None
}
