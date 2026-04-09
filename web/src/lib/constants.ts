export const HERO_HEADLINE = "Screen Time for the Age of Vibe Coding";

export const HERO_SUBTEXT =
  "The first developer wellness platform purpose-built for AI-assisted programming. Track your habits, detect unhealthy patterns, and build a better relationship with your AI tools.";

export const STATS = [
  {
    value: 71.1,
    suffix: "%",
    label: "of developers report regular time distortion during AI coding sessions",
    citation: "VibeCheck Survey, n=142",
  },
  {
    value: 3.68,
    suffix: "h",
    label: "mean uninterrupted session length, with only 8.9% including voluntary breaks",
    citation: "Session tracking study, n=38",
  },
  {
    value: 67,
    suffix: "%",
    label: "longer than self-estimated: actual session durations exceed what developers think",
    citation: "Objective telemetry comparison",
  },
];

export const AIFL_PHASES = [
  {
    name: "Intent Articulation",
    description: "Developer describes goal in natural language. Low cognitive cost.",
    intervention: "Track prompt submission events",
  },
  {
    name: "Instant Generation",
    description: "AI produces code in seconds. Immediate reward signal.",
    intervention: "Measure inter-prompt intervals",
  },
  {
    name: "Near-Miss Evaluation",
    description: 'Code is "almost right" -- drives next iteration like a slot near-miss.',
    intervention: "Detect rapid retry patterns",
  },
  {
    name: "Goal Escalation",
    description: '"Just one more feature" -- task scope expands organically.',
    intervention: "Flag scope-creep indicators",
  },
  {
    name: "Friction Elimination",
    description: "No compiler errors, no docs lookup -- no natural stopping points.",
    intervention: "Inject artificial pause points",
  },
  {
    name: "Sunk-Cost Lock-In",
    description: "Time + money invested = can't walk away. Rate-limit upgrades at peak.",
    intervention: "Spending alerts and cooldowns",
  },
];

export const FEATURES = [
  {
    title: "Active Window Monitoring",
    description:
      "Detects VS Code, Cursor, Claude Code, JetBrains, terminal AI tools, and 20+ browser-based AI platforms. Classifies every 5 seconds.",
  },
  {
    title: "Background AI Detection",
    description:
      "Tracks Claude Code even when its window isn't focused. The entire session where AI is coding for you counts as AI-assisted.",
  },
  {
    title: "Break Enforcement",
    description:
      "Configurable Pomodoro-style interventions. After N minutes of continuous AI coding, a full-screen overlay reminds you to step away.",
  },
  {
    title: "AIFL Loop Detection",
    description:
      "Machine learning classifier trained on session telemetry detects when you've entered an AI-Induced Flow Loop. Escalating interventions.",
  },
];

export const STEPS = [
  {
    number: "01",
    title: "Install",
    description:
      "Download the desktop app for macOS. It sits in your system tray, quietly monitoring which applications you use.",
  },
  {
    number: "02",
    title: "Track",
    description:
      "Start a session and code normally. VibeCheck classifies your activity in real-time: AI-assisted, manual coding, or non-coding.",
  },
  {
    number: "03",
    title: "Understand",
    description:
      "Review your dashboard. See where your time actually goes. Spot the patterns you couldn't see yourself.",
  },
];

export const TECH_STACK = ["Rust", "Tauri v2", "Svelte 5", "SQLite", "TypeScript"];

export const GITHUB_REPO = "Manavarya09/vibecheck-dashboard";
