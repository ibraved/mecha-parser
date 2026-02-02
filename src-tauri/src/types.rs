use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Player {
  pub player_id: String,
  pub display_name: Option<String>,
  pub ready: Option<bool>,
  pub mecha_id: Option<u32>,
  pub is_ai: Option<bool>,
  pub camp_id: Option<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TeamSplit {
  pub team1: Vec<Player>,
  pub team2: Vec<Player>,
  pub unassigned: Vec<Player>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RosterSnapshot {
  /// Monotonic version counter incremented on each emitted snapshot.
  pub version: u64,
  pub teams: TeamSplit,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrackerStatus {
  pub tracking: bool,
  pub log_path: Option<String>,
  pub last_error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppConfig {
  pub mode: LogMode,
  pub manual_log_path: Option<String>,
  pub asset_pack_dir: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum LogMode {
  Auto,
  Manual,
}

impl Default for AppConfig {
  fn default() -> Self {
    Self {
      mode: LogMode::Auto,
      manual_log_path: None,
      asset_pack_dir: None,
    }
  }
}

