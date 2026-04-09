use super::detector::{has_ai_child_process, DetectedWindow};

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

    // Standalone AI coding tools (app-level detection)
    if is_ai_coding_app(&app) {
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

    // Terminal detection (multi-layered)
    if is_terminal(&app) {
        // Layer 1: Check window title for AI tools (requires Screen Recording)
        if has_ai_terminal_tool_in_title(&title) {
            return ActivityCategory::AiAssisted;
        }

        // Layer 2: Check child processes for AI tools (no permissions needed)
        if has_ai_child_process(window.process_id).is_some() {
            return ActivityCategory::AiAssisted;
        }

        // Layer 3: Check window title for terminal editors
        if has_terminal_editor_in_title(&title) {
            return ActivityCategory::ManualCoding;
        }

        // Default: terminal is manual coding
        return ActivityCategory::ManualCoding;
    }

    // JetBrains IDEs
    if is_jetbrains(&app) {
        if title.contains("ai assistant") {
            return ActivityCategory::AiAssisted;
        }
        return ActivityCategory::ManualCoding;
    }

    // Xcode
    if app.contains("xcode") {
        return ActivityCategory::ManualCoding;
    }

    ActivityCategory::NonCoding
}

fn is_ai_coding_app(app: &str) -> bool {
    app.contains("cursor")
        || app.contains("claude")
        || app.contains("windsurf")
        || app.contains("codeium")
        || app.contains("amazon q")
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
        || title.contains("github copilot")
        || title.contains("copilot chat")
        || title.contains("amazon q")
        || title.contains("codewhisperer")
        || title.contains("devin")
        || title.contains("lovable")
        || title.contains("cursor composer")
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

fn has_terminal_editor_in_title(title: &str) -> bool {
    title.contains("nvim")
        || title.contains("vim")
        || title.contains("helix")
        || title.contains("kakoune")
        || title.contains("nano")
        || title.contains("emacs")
}

fn is_code_editor(app: &str) -> bool {
    app.contains("code") || app.contains("visual studio") || app.contains("zed")
}

fn is_jetbrains(app: &str) -> bool {
    app.contains("idea")
        || app.contains("intellij")
        || app.contains("pycharm")
        || app.contains("webstorm")
        || app.contains("goland")
        || app.contains("phpstorm")
        || app.contains("rustrover")
        || app.contains("clion")
        || app.contains("rider")
        || app.contains("datagrip")
        || app.contains("dataspell")
        || app.contains("fleet")
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
            process_id: 0, // tests don't use process-based detection
        }
    }

    #[test]
    fn cursor_is_always_ai_assisted() {
        assert_eq!(classify(&window("Cursor", "main.rs")), ActivityCategory::AiAssisted);
        assert_eq!(classify(&window("cursor", "")), ActivityCategory::AiAssisted);
    }

    #[test]
    fn claude_code_app_is_ai_assisted() {
        assert_eq!(classify(&window("Claude", "")), ActivityCategory::AiAssisted);
        assert_eq!(classify(&window("claude", "~/vibe-track")), ActivityCategory::AiAssisted);
    }

    #[test]
    fn windsurf_is_ai_assisted() {
        assert_eq!(classify(&window("Windsurf", "main.rs")), ActivityCategory::AiAssisted);
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

    #[test]
    fn zen_browser_with_claude_is_ai_assisted() {
        assert_eq!(
            classify(&window("Zen Browser", "Claude - new chat")),
            ActivityCategory::AiAssisted
        );
    }

    #[test]
    fn arc_browser_without_ai_is_non_coding() {
        assert_eq!(
            classify(&window("Arc", "Twitter")),
            ActivityCategory::NonCoding
        );
    }

    #[test]
    fn codeium_is_ai_assisted() {
        assert_eq!(
            classify(&window("Codeium", "project/main.rs")),
            ActivityCategory::AiAssisted
        );
    }

    #[test]
    fn terminal_with_claude_code_title_is_ai_assisted() {
        assert_eq!(
            classify(&window("Terminal", "claude ~/project")),
            ActivityCategory::AiAssisted
        );
        assert_eq!(
            classify(&window("iTerm2", "aider main.py")),
            ActivityCategory::AiAssisted
        );
    }

    #[test]
    fn terminal_with_nvim_is_manual() {
        assert_eq!(
            classify(&window("iTerm2", "nvim src/main.rs")),
            ActivityCategory::ManualCoding
        );
    }

    #[test]
    fn jetbrains_is_manual_coding() {
        assert_eq!(
            classify(&window("IntelliJ IDEA", "Main.java")),
            ActivityCategory::ManualCoding
        );
        assert_eq!(
            classify(&window("PyCharm", "app.py")),
            ActivityCategory::ManualCoding
        );
        assert_eq!(
            classify(&window("RustRover", "lib.rs")),
            ActivityCategory::ManualCoding
        );
    }

    #[test]
    fn jetbrains_ai_assistant_is_ai_assisted() {
        assert_eq!(
            classify(&window("IntelliJ IDEA", "AI Assistant - Main.java")),
            ActivityCategory::AiAssisted
        );
    }

    #[test]
    fn ghostty_terminal_is_manual() {
        assert_eq!(
            classify(&window("Ghostty", "~/code")),
            ActivityCategory::ManualCoding
        );
    }

    #[test]
    fn browser_with_replit_is_ai_assisted() {
        assert_eq!(
            classify(&window("Google Chrome", "Replit - main.py")),
            ActivityCategory::AiAssisted
        );
    }
}
