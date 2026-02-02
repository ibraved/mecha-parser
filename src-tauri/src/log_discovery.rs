use std::{
  fs,
  path::{Path, PathBuf},
  time::SystemTime,
};

use sysinfo::{ProcessRefreshKind, RefreshKind, System};

fn newest_time(meta: &fs::Metadata) -> SystemTime {
  meta
    .created()
    .or_else(|_| meta.modified())
    .unwrap_or(SystemTime::UNIX_EPOCH)
}

fn iter_log_base_candidates(exe_path: &Path) -> Vec<PathBuf> {
  let mut bases: Vec<PathBuf> = vec![];
  let mut p = exe_path.parent();
  for _ in 0..3 {
    if let Some(parent) = p {
      bases.push(parent.to_path_buf());
      p = parent.parent();
    }
  }

  let mut out: Vec<PathBuf> = vec![];
  for base in bases {
    out.push(base.join("logs").join("MechaBREAK"));
    out.push(base.join("MechaBreak").join("logs").join("MechaBREAK"));
    out.push(base.join("Game").join("MechaBreak").join("logs").join("MechaBREAK"));
  }

  out.sort();
  out.dedup();
  out
}

fn pick_latest_file_in_latest_folder(log_base: &Path) -> Option<PathBuf> {
  let subfolders = fs::read_dir(log_base)
    .ok()?
    .filter_map(|e| e.ok())
    .filter(|e| e.file_type().map(|t| t.is_dir()).unwrap_or(false))
    .collect::<Vec<_>>();

  let latest_folder = subfolders
    .into_iter()
    .max_by_key(|e| e.metadata().map(|m| newest_time(&m)).unwrap_or(SystemTime::UNIX_EPOCH))?;

  let files = fs::read_dir(latest_folder.path())
    .ok()?
    .filter_map(|e| e.ok())
    .filter(|e| e.file_type().map(|t| t.is_file()).unwrap_or(false))
    // Filter for _up.log files only - these contain the player roster information
    // Exclude SkillLog.log and other specialized log types
    .filter(|e| {
      let name = e.file_name().to_string_lossy().to_lowercase();
      name.ends_with("_up.log")
    })
    .collect::<Vec<_>>();

  files
    .into_iter()
    .max_by_key(|e| e.metadata().map(|m| newest_time(&m)).unwrap_or(SystemTime::UNIX_EPOCH))
    .map(|e| e.path())
}

/// Try to locate the newest MechaBREAK log file by inspecting running processes.
///
/// Ported from the Python logic in `Mecha Break Tracker/coding/main.py`.
pub fn find_latest_mechabreak_log_file() -> Option<PathBuf> {
  let refresh = RefreshKind::new().with_processes(ProcessRefreshKind::everything());
  let mut sys = System::new_with_specifics(refresh);
  sys.refresh_processes();

  for (_pid, proc_) in sys.processes() {
    // sysinfo 0.30's name() returns &str directly
    let name = proc_.name().to_lowercase();
    let exe_path = proc_.exe().map(|p| p.to_path_buf());
    let Some(exe_path) = exe_path else { continue };

    // Process names changed across builds; match a few common variants.
    let name_match = name.contains("mechabreak") || name.contains("starmechabreak") || name.contains("seasungame");
    let path_match = exe_path.to_string_lossy().to_ascii_lowercase().contains("mechabreak");
    
    if !name_match && !path_match {
      continue;
    }

    for log_base in iter_log_base_candidates(&exe_path) {
      if !log_base.exists() {
        continue;
      }
      if let Some(file) = pick_latest_file_in_latest_folder(&log_base) {
        return Some(file);
      }
    }
  }
  None
}

