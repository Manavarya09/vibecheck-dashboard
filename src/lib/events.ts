import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import type { SessionUpdate } from "./types";

export async function onSessionUpdate(
  callback: (update: SessionUpdate) => void
): Promise<UnlistenFn> {
  return listen<SessionUpdate>("session-update", (event) => {
    callback(event.payload);
  });
}

export async function onTrayPauseToggle(
  callback: () => void
): Promise<UnlistenFn> {
  return listen("tray-pause-toggle", () => {
    callback();
  });
}
