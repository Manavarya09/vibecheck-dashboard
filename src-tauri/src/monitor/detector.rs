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

/// Check if any AI coding tool process is running anywhere on the system.
/// If Claude Code is running, the entire session is AI-assisted -- not just
/// the moments when its window is focused. The user prompts, then the AI
/// codes for them. Switching to another window doesn't change that.
#[cfg(target_os = "macos")]
pub fn find_running_ai_tool() -> Option<String> {
    let ai_tools: &[(&str, &str)] = &[
        ("claude", "Claude Code"),
        ("aider", "aider"),
        ("codex", "Codex"),
    ];

    for (process_name, display_name) in ai_tools {
        if let Ok(output) = std::process::Command::new("pgrep")
            .args(["-x", process_name])
            .output()
        {
            let stdout = String::from_utf8_lossy(&output.stdout);
            if !stdout.trim().is_empty() {
                return Some(display_name.to_string());
            }
        }
    }

    None
}

#[cfg(not(target_os = "macos"))]
pub fn find_running_ai_tool() -> Option<String> {
    None
}

#[cfg(target_os = "macos")]
pub fn get_idle_seconds() -> Option<u64> {
    let output = std::process::Command::new("ioreg")
        .args(["-c", "IOHIDSystem", "-d", "4"])
        .output()
        .ok()?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    for line in stdout.lines() {
        if line.contains("HIDIdleTime") {
            // Extract the numeric value
            let parts: Vec<&str> = line.split('=').collect();
            if let Some(val_str) = parts.last() {
                let trimmed = val_str.trim().trim_end_matches('}').trim();
                if let Ok(nanos) = trimmed.parse::<u64>() {
                    return Some(nanos / 1_000_000_000);
                }
            }
        }
    }
    None
}

#[cfg(not(target_os = "macos"))]
pub fn get_idle_seconds() -> Option<u64> {
    None
}
