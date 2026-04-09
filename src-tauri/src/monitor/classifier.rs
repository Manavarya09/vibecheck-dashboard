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

    // Terminal emulators
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

#[cfg(test)]
mod tests {
    use super::*;

    fn window(app: &str, title: &str) -> DetectedWindow {
        DetectedWindow {
            app_name: app.to_string(),
            window_title: title.to_string(),
        }
    }

    #[test]
    fn cursor_is_always_ai_assisted() {
        assert_eq!(classify(&window("Cursor", "main.rs")), ActivityCategory::AiAssisted);
        assert_eq!(classify(&window("cursor", "")), ActivityCategory::AiAssisted);
    }

    #[test]
    fn browser_with_claude_is_ai_assisted() {
        assert_eq!(
            classify(&window("Google Chrome", "Claude - Anthropic")),
            ActivityCategory::AiAssisted
        );
        assert_eq!(
            classify(&window("Arc", "Claude")),
            ActivityCategory::AiAssisted
        );
    }

    #[test]
    fn browser_with_chatgpt_is_ai_assisted() {
        assert_eq!(
            classify(&window("Safari", "ChatGPT")),
            ActivityCategory::AiAssisted
        );
    }

    #[test]
    fn browser_without_ai_is_non_coding() {
        assert_eq!(
            classify(&window("Google Chrome", "Reddit - Pair Programming")),
            ActivityCategory::NonCoding
        );
    }

    #[test]
    fn vscode_is_manual_coding() {
        assert_eq!(
            classify(&window("Code", "main.rs - my-project")),
            ActivityCategory::ManualCoding
        );
    }

    #[test]
    fn vscode_with_copilot_is_ai_assisted() {
        assert_eq!(
            classify(&window("Code", "Copilot Chat - main.rs")),
            ActivityCategory::AiAssisted
        );
    }

    #[test]
    fn terminal_is_manual_coding() {
        assert_eq!(classify(&window("Terminal", "zsh")), ActivityCategory::ManualCoding);
        assert_eq!(classify(&window("iTerm2", "~")), ActivityCategory::ManualCoding);
        assert_eq!(classify(&window("Warp", "cargo build")), ActivityCategory::ManualCoding);
        assert_eq!(classify(&window("Ghostty", "~")), ActivityCategory::ManualCoding);
    }

    #[test]
    fn xcode_is_manual_coding() {
        assert_eq!(
            classify(&window("Xcode", "MyApp.swift")),
            ActivityCategory::ManualCoding
        );
    }

    #[test]
    fn slack_is_non_coding() {
        assert_eq!(
            classify(&window("Slack", "#general")),
            ActivityCategory::NonCoding
        );
    }

    #[test]
    fn finder_is_non_coding() {
        assert_eq!(
            classify(&window("Finder", "Downloads")),
            ActivityCategory::NonCoding
        );
    }

    #[test]
    fn empty_strings_are_non_coding() {
        assert_eq!(classify(&window("", "")), ActivityCategory::NonCoding);
    }

    #[test]
    fn classification_is_case_insensitive() {
        assert_eq!(
            classify(&window("GOOGLE CHROME", "CLAUDE")),
            ActivityCategory::AiAssisted
        );
        assert_eq!(
            classify(&window("TERMINAL", "zsh")),
            ActivityCategory::ManualCoding
        );
    }

    #[test]
    fn zed_is_manual_coding() {
        assert_eq!(
            classify(&window("Zed", "main.rs")),
            ActivityCategory::ManualCoding
        );
    }

    #[test]
    fn brave_with_perplexity_is_ai_assisted() {
        assert_eq!(
            classify(&window("Brave Browser", "Perplexity AI")),
            ActivityCategory::AiAssisted
        );
    }

    #[test]
    fn browser_with_v0_is_ai_assisted() {
        assert_eq!(
            classify(&window("Google Chrome", "v0 by Vercel")),
            ActivityCategory::AiAssisted
        );
    }

    #[test]
    fn browser_with_bolt_is_ai_assisted() {
        assert_eq!(
            classify(&window("Arc", "bolt.new")),
            ActivityCategory::AiAssisted
        );
    }
}
