use super::detector::DetectedWindow;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ActivityCategory {
    AiAssisted,
    ManualCoding,
    NonCoding,
}

impl ActivityCategory {
    pub fn as_str(&self) -> &'static str {
        match self {
            ActivityCategory::AiAssisted => "ai_assisted",
            ActivityCategory::ManualCoding => "manual_coding",
            ActivityCategory::NonCoding => "non_coding",
        }
    }
}

pub fn classify(window: &DetectedWindow) -> ActivityCategory {
    let app = window.app_name.to_lowercase();
    let title = window.window_title.to_lowercase();

    // Cursor is always AI-assisted
    if app.contains("cursor") {
        return ActivityCategory::AiAssisted;
    }

    // Browser with AI tool tabs
    if is_browser(&app) {
        if has_ai_tool_in_title(&title) {
            return ActivityCategory::AiAssisted;
        }
        return ActivityCategory::NonCoding;
    }

    // VS Code / editors with AI context
    if is_code_editor(&app) {
        if title.contains("copilot") || title.contains("cody") {
            return ActivityCategory::AiAssisted;
        }
        return ActivityCategory::ManualCoding;
    }

    // Terminal with AI coding tools (Claude Code, aider, etc.)
    if is_terminal(&app) && has_ai_terminal_tool_in_title(&title) {
        return ActivityCategory::AiAssisted;
    }

    // Terminal emulators (plain terminal usage)
    if is_terminal(&app) {
        return ActivityCategory::ManualCoding;
    }

    // Xcode
    if app.contains("xcode") {
        return ActivityCategory::ManualCoding;
    }

    ActivityCategory::NonCoding
}

fn is_browser(app: &str) -> bool {
    app.contains("chrome")
        || app.contains("safari")
        || app.contains("firefox")
        || app.contains("arc")
        || app.contains("brave")
        || app.contains("edge")
        || app.contains("zen")
        || app.contains("orion")
}

fn has_ai_tool_in_title(title: &str) -> bool {
    title.contains("claude")
        || title.contains("chatgpt")
        || title.contains("copilot")
        || title.contains("gemini")
        || title.contains("perplexity")
        || title.contains("v0")
        || title.contains("bolt")
        || title.contains("replit")
}

fn has_ai_terminal_tool_in_title(title: &str) -> bool {
    title.contains("claude")
        || title.contains("aider")
        || title.contains("copilot")
        || title.contains("cody")
        || title.contains("continue")
        || title.contains("mentat")
        || title.contains("sweep")
        || title.contains("codex")
}

fn is_code_editor(app: &str) -> bool {
    app.contains("code") || app.contains("visual studio") || app.contains("zed")
}

fn is_terminal(app: &str) -> bool {
    app.contains("terminal")
        || app.contains("iterm")
        || app.contains("warp")
        || app.contains("alacritty")
        || app.contains("kitty")
        || app.contains("ghostty")
        || app.contains("hyper")
}
