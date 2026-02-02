import type { UnlistenFn } from "@tauri-apps/api/event";
import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/core";

import type { AppConfig, RosterSnapshot, TrackerStatus } from "@/types";
import { isTauri } from "@/lib/tauri";

export const EVENTS = {
  rosterSnapshot: "roster:snapshot",
  trackerStatus: "tracker:status",
} as const;

export async function startTracking(): Promise<void> {
  if (!isTauri()) return;
  await invoke("start_tracking");
}

export async function stopTracking(): Promise<void> {
  if (!isTauri()) return;
  await invoke("stop_tracking");
}

export async function getStatus(): Promise<TrackerStatus | null> {
  if (!isTauri()) return null;
  return await invoke<TrackerStatus>("get_status");
}

export async function getConfig(): Promise<AppConfig | null> {
  if (!isTauri()) return null;
  return await invoke<AppConfig>("get_config");
}

export async function setConfig(cfg: AppConfig): Promise<void> {
  if (!isTauri()) return;
  await invoke("set_config", { cfg });
}

export async function onRosterSnapshot(cb: (snap: RosterSnapshot) => void): Promise<UnlistenFn | null> {
  if (!isTauri()) return null;
  return await listen<RosterSnapshot>(EVENTS.rosterSnapshot, (e) => cb(e.payload));
}

export async function onTrackerStatus(cb: (status: TrackerStatus) => void): Promise<UnlistenFn | null> {
  if (!isTauri()) return null;
  return await listen<TrackerStatus>(EVENTS.trackerStatus, (e) => cb(e.payload));
}

