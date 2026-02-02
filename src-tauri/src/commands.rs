use std::{
  path::PathBuf,
  sync::{
    atomic::{AtomicBool, Ordering},
    Mutex,
  },
  time::{Duration, Instant},
};
use tauri::Emitter;
use tauri::{AppHandle, Manager, State};

use crate::{
  config::{load_config, save_config},
  log_discovery::find_latest_mechabreak_log_file,
  parser::{self, Update},
  state::PlayersState,
  tailer::{spawn_tailer, TailerHandle},
  types::{AppConfig, LogMode, RosterSnapshot, TrackerStatus},
};
const EVT_ROSTER_SNAPSHOT: &str = "roster:snapshot";
const EVT_TRACKER_STATUS: &str = "tracker:status";

pub struct TrackerController {
  stop: AtomicBool,
  tailer: Mutex<Option<TailerHandle>>,
  tracking: AtomicBool,
  config: Mutex<Option<AppConfig>>,
  status: Mutex<TrackerStatus>,
}

impl Default for TrackerController {
  fn default() -> Self {
    Self {
      stop: AtomicBool::new(false),
      tailer: Mutex::new(None),
      tracking: AtomicBool::new(false),
      config: Mutex::new(None),
      status: Mutex::new(TrackerStatus {
        tracking: false,
        log_path: None,
        last_error: None,
      }),
    }
  }
}

fn set_and_emit_status(ctrl: &TrackerController, app: &AppHandle, status: TrackerStatus) {
  if let Ok(mut guard) = ctrl.status.lock() {
    *guard = status.clone();
  }
  eprintln!("[emit] Emitting {} with tracking={}", EVT_TRACKER_STATUS, status.tracking);
  match app.emit(EVT_TRACKER_STATUS, status) {
    Ok(_) => eprintln!("[emit] Event sent successfully"),
    Err(e) => eprintln!("[emit] Event failed: {:?}", e),
  }
}

fn emit_snapshot(app: &AppHandle, snapshot: RosterSnapshot) {
  eprintln!("[emit] Emitting {} v{} with {} team1, {} team2", 
    EVT_ROSTER_SNAPSHOT, snapshot.version, 
    snapshot.teams.team1.len(), snapshot.teams.team2.len());
  let _ = app.emit(EVT_ROSTER_SNAPSHOT, snapshot);
}

fn get_or_load_config(ctrl: &TrackerController, app: &AppHandle) -> AppConfig {
  if let Ok(mut guard) = ctrl.config.lock() {
    if let Some(cfg) = guard.as_ref() {
      return cfg.clone();
    }
    let cfg = load_config(app);
    *guard = Some(cfg.clone());
    return cfg;
  }
  AppConfig::default()
}

#[tauri::command]
pub fn start_tracking(app: AppHandle, ctrl: State<TrackerController>) -> Result<(), String> {
  if ctrl.tracking.swap(true, Ordering::SeqCst) {
    return Ok(());
  }
  ctrl.stop.store(false, Ordering::SeqCst);

  let cfg = get_or_load_config(&ctrl, &app);
  
  let log_path: PathBuf = match cfg.mode {
    LogMode::Auto => find_latest_mechabreak_log_file()
      .ok_or_else(|| "Could not locate MechaBREAK log file".to_string())?,
    LogMode::Manual => {
      let p = cfg
        .manual_log_path
        .clone()
        .ok_or_else(|| "Manual mode selected but no manualLogPath configured".to_string())?;
      PathBuf::from(p)
    }
  };

  set_and_emit_status(
    &ctrl,
    &app,
    TrackerStatus {
      tracking: true,
      log_path: Some(log_path.to_string_lossy().to_string()),
      last_error: None,
    },
  );

  let (tailer_handle, rx) = spawn_tailer(log_path.clone(), true);
  {
    let mut guard = ctrl.tailer.lock().map_err(|_| "tracker lock poisoned".to_string())?;
    *guard = Some(tailer_handle.clone());
  }

  // Clone app handle for the thread
  let app_clone = app.clone();
  
  // Processing loop in background task (non-async; keeps it simple).
  std::thread::spawn(move || {
    let mut state = PlayersState::default();
    let mut version: u64 = 0;
    let mut dirty = true;
    let mut last_emit = Instant::now() - Duration::from_secs(10);

    // Get controller from app state inside the thread
    let ctrl = app_clone.state::<TrackerController>();

    // Emit an initial empty snapshot so UI can render placeholders.
    version += 1;
    emit_snapshot(
      &app_clone,
      RosterSnapshot {
        version,
        teams: state.snapshot_teams(),
      },
    );

    while !ctrl.stop.load(Ordering::Relaxed) {
      match rx.recv_timeout(Duration::from_millis(100)) {
        Ok(line) => {
          for up in parser::parse_line(&line) {
            match up {
              Update::ResetPlayers => {
                state.clear();
                dirty = true;
              }
              Update::PlayerUpdate(pu) => {
                dirty |= state.apply_update(pu);
              }
            }
          }
        }
        Err(std::sync::mpsc::RecvTimeoutError::Timeout) => {}
        Err(std::sync::mpsc::RecvTimeoutError::Disconnected) => break,
      }

      if dirty && last_emit.elapsed() >= Duration::from_millis(120) {
        version += 1;
        emit_snapshot(
          &app_clone,
          RosterSnapshot {
            version,
            teams: state.snapshot_teams(),
          },
        );
        dirty = false;
        last_emit = Instant::now();
      }
    }

    tailer_handle.stop();
    set_and_emit_status(
      &ctrl,
      &app_clone,
      TrackerStatus {
        tracking: false,
        log_path: Some(log_path.to_string_lossy().to_string()),
        last_error: None,
      },
    );
    ctrl.tracking.store(false, Ordering::SeqCst);
  });

  Ok(())
}

#[tauri::command]
pub fn stop_tracking(ctrl: State<TrackerController>) -> Result<(), String> {
  ctrl.stop.store(true, Ordering::SeqCst);
  if let Ok(mut guard) = ctrl.tailer.lock() {
    if let Some(t) = guard.take() {
      t.stop();
    }
  }
  Ok(())
}

#[tauri::command]
pub fn get_status(ctrl: State<TrackerController>) -> TrackerStatus {
  if let Ok(guard) = ctrl.status.lock() {
    return guard.clone();
  }
  TrackerStatus {
    tracking: ctrl.tracking.load(Ordering::Relaxed),
    log_path: None,
    last_error: None,
  }
}

#[tauri::command]
pub fn get_config(app: AppHandle, ctrl: State<TrackerController>) -> AppConfig {
  get_or_load_config(&ctrl, &app)
}

#[tauri::command]
pub fn set_config(app: AppHandle, ctrl: State<TrackerController>, cfg: AppConfig) -> Result<(), String> {
  save_config(&app, &cfg)?;
  if let Ok(mut guard) = ctrl.config.lock() {
    *guard = Some(cfg.clone());
  }
  Ok(())
}