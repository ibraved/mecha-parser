use std::{
  fs::File,
  io::{BufRead, BufReader, Seek, SeekFrom},
  path::{Path, PathBuf},
  sync::{
    atomic::{AtomicBool, Ordering},
    mpsc::{self, Receiver},
    Arc,
  },
  thread,
  time::Duration,
};

#[derive(Clone)]
pub struct TailerHandle {
  stop: Arc<AtomicBool>,
}

impl TailerHandle {
  pub fn stop(&self) {
    self.stop.store(true, Ordering::Relaxed);
  }
}

fn open_reader(path: &Path, start_at_end: bool) -> std::io::Result<BufReader<File>> {
  let mut file = File::open(path)?;
  if start_at_end {
    let _ = file.seek(SeekFrom::End(0));
  } else {
    let _ = file.seek(SeekFrom::Start(0));
  }
  Ok(BufReader::new(file))
}

/// Tail a file and produce appended lines.
///
/// - Starts at EOF by default (like `tail -f`).
/// - Detects truncation/rotation by noticing file length shrink and re-opens.
pub fn spawn_tailer(path: PathBuf, start_at_end: bool) -> (TailerHandle, Receiver<String>) {
  let stop = Arc::new(AtomicBool::new(false));
  let (tx, rx) = mpsc::channel::<String>();

  let stop2 = stop.clone();
  thread::spawn(move || {
    let mut reader = match open_reader(&path, start_at_end) {
      Ok(r) => r,
      Err(_) => {
        // Best-effort retry loop if file isn't available yet.
        loop {
          if stop2.load(Ordering::Relaxed) {
            return;
          }
          thread::sleep(Duration::from_millis(500));
          if let Ok(r) = open_reader(&path, start_at_end) {
            break r;
          }
        }
      }
    };

    let mut line = String::new();
    let mut idle_backoff = Duration::from_millis(200);

    while !stop2.load(Ordering::Relaxed) {
      line.clear();
      match reader.read_line(&mut line) {
        Ok(0) => {
          // EOF: sleep a bit, but also check for truncation/rotation.
          let cur_pos = reader.get_mut().stream_position().unwrap_or(0);
          if let Ok(meta) = reader.get_ref().metadata() {
            if meta.len() < cur_pos {
              if let Ok(r) = open_reader(&path, false) {
                reader = r;
              }
            }
          }

          thread::sleep(idle_backoff);
          // gently ramp up to reduce CPU usage when idle
          idle_backoff = (idle_backoff * 2).min(Duration::from_millis(1000));
        }
        Ok(_) => {
          idle_backoff = Duration::from_millis(200);
          // Trim newline(s)
          while line.ends_with('\n') || line.ends_with('\r') {
            line.pop();
          }
          if !line.is_empty() {
            let _ = tx.send(line.clone());
          }
        }
        Err(_) => {
          thread::sleep(Duration::from_millis(500));
        }
      }
    }
  });

  (TailerHandle { stop }, rx)
}