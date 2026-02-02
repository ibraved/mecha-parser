use indexmap::IndexMap;

use crate::{
  parser::PlayerUpdate,
  types::{Player, TeamSplit},
};

pub const MAX_PLAYERS: usize = 50;

#[derive(Default)]
pub struct PlayersState {
  // Keep insertion order for LRU-like eviction (like Python OrderedDict).
  players: IndexMap<String, Player>,
}

impl PlayersState {
  pub fn clear(&mut self) {
    self.players.clear();
  }

  pub fn apply_update(&mut self, up: PlayerUpdate) -> bool {
    if up.player_id.trim().is_empty() {
      return false;
    }
    let player_id = up.player_id.trim().to_string();

    if let Some(existing) = self.players.get_mut(&player_id) {
      let mut changed = false;
      if let Some(v) = up.display_name {
        if Some(&v) != existing.display_name.as_ref() {
          existing.display_name = Some(v);
          changed = true;
        }
      }
      if let Some(v) = up.ready {
        if Some(v) != existing.ready {
          existing.ready = Some(v);
          changed = true;
        }
      }
      if let Some(v) = up.mecha_id {
        if Some(v) != existing.mecha_id {
          existing.mecha_id = Some(v);
          changed = true;
        }
      }
      if let Some(v) = up.is_ai {
        if Some(v) != existing.is_ai {
          existing.is_ai = Some(v);
          changed = true;
        }
      }
      if let Some(v) = up.camp_id {
        if Some(v) != existing.camp_id {
          existing.camp_id = Some(v);
          changed = true;
        }
      }
      return changed;
    }

    // New player: evict oldest if needed.
    if self.players.len() >= MAX_PLAYERS {
      self.players.shift_remove_index(0);
    }
    self.players.insert(
      player_id.clone(),
      Player {
        player_id,
        display_name: up.display_name,
        ready: up.ready,
        mecha_id: up.mecha_id,
        is_ai: up.is_ai,
        camp_id: up.camp_id,
      },
    );
    true
  }

  pub fn snapshot_teams(&self) -> TeamSplit {
    let mut team1: Vec<Player> = vec![];
    let mut team2: Vec<Player> = vec![];
    let mut unassigned: Vec<Player> = vec![];

    for p in self.players.values() {
      match p.camp_id {
        Some(1) => team1.push(p.clone()),
        Some(2) => team2.push(p.clone()),
        _ => unassigned.push(p.clone()),
      }
    }

    // Sort to roughly match terminal view: non-AI first, unknown, then AI; then name.
    fn sort_key(p: &Player) -> (u8, String, String) {
      let bucket = match p.is_ai {
        Some(true) => 2,
        Some(false) => 0,
        None => 1,
      };
      let name = p.display_name.clone().unwrap_or_default();
      (bucket, name, p.player_id.clone())
    }

    team1.sort_by_key(sort_key);
    team2.sort_by_key(sort_key);
    unassigned.sort_by_key(sort_key);

    TeamSplit {
      team1,
      team2,
      unassigned,
    }
  }
}

