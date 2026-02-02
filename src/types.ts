export type Player = {
  playerId: string;
  displayName?: string;
  ready?: boolean;
  mechaId?: number;
  isAi?: boolean;
  campId?: number;
};

export type TeamSplit = {
  team1: Player[];
  team2: Player[];
  unassigned: Player[];
};

export type RosterSnapshot = {
  version: number;
  teams: TeamSplit;
};

export type TrackerStatus = {
  tracking: boolean;
  logPath?: string;
  lastError?: string;
};

export type LogMode = "Auto" | "Manual";

export type AppConfig = {
  mode: LogMode;
  manualLogPath?: string;
  assetPackDir?: string;
};

