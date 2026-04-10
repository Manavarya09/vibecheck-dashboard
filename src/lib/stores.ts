import { writable } from "svelte/store";
import type { Session, DailySummary, SessionUpdate } from "./types";

export const currentSession = writable<Session | null>(null);
export const todaySummary = writable<DailySummary | null>(null);
export const recentSessions = writable<Session[]>([]);
export const liveUpdate = writable<SessionUpdate | null>(null);
export const autoPaused = writable(false);
