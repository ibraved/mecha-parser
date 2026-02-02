import * as React from "react";

import type { AppConfig, LogMode } from "@/types";
import { getConfig, setConfig } from "@/lib/tracker";
import { isTauri } from "@/lib/tauri";

const DEFAULT: AppConfig = {
  mode: "Auto",
};

export function useConfig() {
  const [config, setLocalConfig] = React.useState<AppConfig>(DEFAULT);
  const [loaded, setLoaded] = React.useState(false);

  React.useEffect(() => {
    (async () => {
      const cfg = await getConfig();
      if (cfg) setLocalConfig(cfg);
      setLoaded(true);
    })();
  }, []);

  const setMode = (mode: LogMode) => setLocalConfig((c) => ({ ...c, mode }));
  const setManualLogPath = (manualLogPath: string) => setLocalConfig((c) => ({ ...c, manualLogPath }));

  const save = async () => {
    if (!isTauri()) return;
    await setConfig(config);
  };

  return { config, setLocalConfig, loaded, setMode, setManualLogPath, save, isTauri: isTauri() };
}

