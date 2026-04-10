import { invoke } from "@tauri-apps/api/core";
import type { Session, DailySummary, SessionStats, ActivityEntry } from "./types";

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

export async function getRecentActivity(
  sessionId: number,
  limit?: number
): Promise<ActivityEntry[]> {
  return invoke("get_recent_activity", { sessionId, limit: limit ?? 20 });
}

export async function getSettings(): Promise<Record<string, string>> {
  return invoke("get_settings");
}

export async function updateSetting(key: string, value: string): Promise<void> {
  return invoke("update_setting", { key, value });
}

export async function resetSettings(): Promise<Record<string, string>> {
  return invoke("reset_settings");
}


export async function getAutoStartEnabled(): Promise<boolean> {
  return invoke("get_auto_start_enabled");
}

export async function checkScreenRecordingPermission(): Promise<boolean> {
  return invoke("check_screen_recording_permission");
}

export async function setAutostart(enabled: boolean): Promise<void> {
  return invoke("set_autostart", { enabled });
}

export async function getAutostartEnabled(): Promise<boolean> {
  return invoke("get_autostart_enabled");
}

export async function exportData(format: "json" | "csv"): Promise<string> {
  return invoke("export_data", { format });
}

export async function getHeatmapData(days?: number): Promise<DailySummary[]> {
  return invoke("get_heatmap_data", { days: days ?? 365 });
}

export async function getHistoricalStats(days: number): Promise<DailySummary[]> {
  return invoke("get_historical_stats", { days });
}


export interface SpendingRate {
  id: number;
  toolName: string;
  rateType: string;
  rateValue: number;
  billingPeriod: string | null;
}

export async function getSpendingRates(): Promise<SpendingRate[]> {
  return invoke("get_spending_rates");
}

export async function upsertSpendingRate(toolName: string, rateType: string, rateValue: number, billingPeriod: string): Promise<void> {
  return invoke("upsert_spending_rate", { toolName, rateType, rateValue, billingPeriod });
}

export async function deleteSpendingRate(id: number): Promise<void> {
  return invoke("delete_spending_rate", { id });
}


export async function getDbPath(): Promise<string> {
  return invoke("get_db_path");
}
