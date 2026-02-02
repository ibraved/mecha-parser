import * as React from "react";

import type { RosterSnapshot, TrackerStatus } from "@/types";
import { getStatus, onRosterSnapshot, onTrackerStatus, startTracking, stopTracking } from "@/lib/tracker";
import { isTauri } from "@/lib/tauri";

export function useTracker() {
  const [status, setStatus] = React.useState<TrackerStatus>(() => ({
    tracking: false,
  }));
  const [snapshot, setSnapshot] = React.useState<RosterSnapshot | null>(null);

  React.useEffect(() => {
    let unlistenRoster: (() => void) | null = null;
    let unlistenStatus: (() => void) | null = null;

    (async () => {
      const s = await getStatus();
      if (s) setStatus(s);

      unlistenRoster = (await onRosterSnapshot(setSnapshot)) ?? null;
      unlistenStatus = (await onTrackerStatus(setStatus)) ?? null;
    })();

    return () => {
      unlistenRoster?.();
      unlistenStatus?.();
    };
  }, []);

  return {
    isTauri: isTauri(),
    status,
    snapshot,
    startTracking: async () => startTracking(),
    stopTracking: async () => stopTracking(),
  };
}

