import { invoke } from "@tauri-apps/api/core";
import type { Session, DailySummary, SessionStats } from "./types";

export async function startSession(): Promise<Session> {
  return invoke("start_session");
}

export async function stopSession(): Promise<void> {
  return invoke("stop_session");
}

export async function pauseSession(): Promise<void> {
  return invoke("pause_session");
}

export async function resumeSession(): Promise<void> {
  return invoke("resume_session");
}

export async function getCurrentSession(): Promise<Session | null> {
  return invoke("get_current_session");
}

export async function getSessionStats(sessionId: number): Promise<SessionStats> {
  return invoke("get_session_stats", { sessionId });
}

export async function getTodaySummary(): Promise<DailySummary> {
  return invoke("get_today_summary");
}

export async function getRecentSessions(limit?: number): Promise<Session[]> {
  return invoke("get_recent_sessions", { limit: limit ?? 10 });
}
