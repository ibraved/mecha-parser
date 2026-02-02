use std::{fs, path::PathBuf};
use tauri::Manager;
use tauri::AppHandle;

use crate::types::AppConfig;

fn config_path(app: &AppHandle) -> Result<PathBuf, String> {
  let dir = app
    .path()
    .app_data_dir()
    .map_err(|e| format!("app_data_dir failed: {e}"))?;
  Ok(dir.join("config.json"))
}

pub fn load_config(app: &AppHandle) -> AppConfig {
  let path = match config_path(app) {
    Ok(p) => p,
    Err(_) => return AppConfig::default(),
  };
  let Ok(bytes) = fs::read(&path) else {
    return AppConfig::default();
  };
  serde_json::from_slice::<AppConfig>(&bytes).unwrap_or_default()
}

pub fn save_config(app: &AppHandle, cfg: &AppConfig) -> Result<(), String> {
  let path = config_path(app)?;
  if let Some(parent) = path.parent() {
    fs::create_dir_all(parent).map_err(|e| format!("create_dir_all failed: {e}"))?;
  }
  let bytes = serde_json::to_vec_pretty(cfg).map_err(|e| format!("serialize config failed: {e}"))?;
  fs::write(&path, bytes).map_err(|e| format!("write config failed: {e}"))?;
  Ok(())
}

