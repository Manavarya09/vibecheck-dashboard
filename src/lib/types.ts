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

export interface SessionUpdate {
  sessionId: number;
  durationSecs: number;
  currentActivity: string;
  currentApp: string;
  aiAssistedSecs: number;
  manualCodingSecs: number;
  nonCodingSecs: number;
}
