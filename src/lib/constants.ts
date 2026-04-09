export const POLLING_INTERVAL_MS = 5000;
export const MAX_FEED_ENTRIES = 20;
export const MAX_RECENT_SESSIONS = 10;
export const IDLE_THRESHOLD_SECS = 300;

export const CATEGORIES = {
  AI_ASSISTED: "ai_assisted",
  MANUAL_CODING: "manual_coding",
  NON_CODING: "non_coding",
} as const;

export const CATEGORY_LABELS: Record<string, string> = {
  ai_assisted: "AI-Assisted",
  manual_coding: "Manual Coding",
  non_coding: "Non-Coding",
};

export const CATEGORY_COLORS: Record<string, string> = {
  ai_assisted: "var(--primary)",
  manual_coding: "var(--success)",
  non_coding: "var(--text-tertiary)",
};
