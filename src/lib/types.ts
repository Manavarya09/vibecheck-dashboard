export interface Session {
  id: number;
  startedAt: string;
  endedAt: string | null;
  durationSecs: number;
  status: "active" | "paused" | "completed";
}

export interface DailySummary {
  date: string;
  totalSecs: number;
  aiAssistedSecs: number;
  manualCodingSecs: number;
  nonCodingSecs: number;
  sessionCount: number;
}

export interface SessionStats {
  totalDurationSecs: number;
  aiAssistedSecs: number;
  manualCodingSecs: number;
  nonCodingSecs: number;
  currentActivity: string | null;
  currentApp: string | null;
}

export interface ActivityEntry {
  id: number;
  sessionId: number;
  timestamp: string;
  appName: string;
  windowTitle: string;
  category: string;
  durationSecs: number;
}

export interface SessionUpdate {
  sessionId: number;
  durationSecs: number;
  currentActivity: string;
  currentApp: string;
  aiAssistedSecs: number;
  manualCodingSecs: number;
  nonCodingSecs: number;
  workflowState: string;
  contextSwitches10m: number;
  aiStreakSecs: number;
  codingStreakSecs: number;
  promptLoopScore: number;
}
