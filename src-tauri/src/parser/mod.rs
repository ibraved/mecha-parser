use once_cell::sync::Lazy;
use regex::Regex;
use serde_json::Value;

#[derive(Debug, Clone)]
pub enum Update {
  ResetPlayers,
  PlayerUpdate(PlayerUpdate),
}

#[derive(Debug, Clone, Default)]
pub struct PlayerUpdate {
  pub player_id: String,
  pub display_name: Option<String>,
  pub ready: Option<bool>,
  pub mecha_id: Option<u32>,
  pub is_ai: Option<bool>,
  pub camp_id: Option<u8>,
}

static PLAYER_LINE: Lazy<Regex> = Lazy::new(|| {
  Regex::new(
    r"playerId\s*:\s*(?P<playerId>\d+),\s*displayName\s*:\s*(?P<displayName>[^,]+),\s*mechaId\s*:\s*(?P<mechaId>\d+),\s*pilotId\s*:\s*\d+,\s*ready\s*:\s*(?P<ready>\w+)",
  )
  .expect("invalid regex")
});

static READY_NOTICE_LINE: Lazy<Regex> = Lazy::new(|| {
  Regex::new(r"(?i)OnEmBattleReadyStateNotice\s+playerId:\s*(?P<playerId>\d+)\s+ready:\s*(?P<ready>\w+)")
    .expect("invalid regex")
});

static MECHA_SELECT_LINE: Lazy<Regex> = Lazy::new(|| {
  Regex::new(
    r"playerId\s*:\s*(?P<playerId>\d+),\s*displayName\s*:\s*(?P<displayName>.+?)\s+选择机甲\[(?P<mechaId>\d+)\]",
  )
  .expect("invalid regex")
});

fn parse_boolish(v: &Value) -> Option<bool> {
  match v {
    Value::Bool(b) => Some(*b),
    Value::String(s) => {
      let sl = s.trim().to_ascii_lowercase();
      if sl == "true" {
        Some(true)
      } else if sl == "false" {
        Some(false)
      } else {
        None
      }
    }
    _ => None,
  }
}

fn parse_boolish_str(s: &str) -> Option<bool> {
  let sl = s.trim().to_ascii_lowercase();
  if sl == "true" {
    Some(true)
  } else if sl == "false" {
    Some(false)
  } else {
    None
  }
}

fn normalize_mecha_id(v: &Value) -> Option<u32> {
  let mid = match v {
    Value::Number(n) => n.as_u64().and_then(|u| u32::try_from(u).ok()),
    Value::String(s) => s.trim().parse::<u32>().ok(),
    _ => None,
  }?;
  if mid == 0 { None } else { Some(mid) }
}

fn extract_json_object_from_line(line: &str) -> Option<String> {
  let msg_idx = line.find("msg=");
  let search_from = msg_idx.map(|i| i + 4).unwrap_or(0);

  let mut start: Option<usize> = None;
  let bytes = line.as_bytes();

  let mut scan_from = search_from;
  for pass in 0..2 {
    for i in scan_from..bytes.len() {
      let ch = bytes[i] as char;
      if ch != '{' && ch != '[' {
        continue;
      }
      if ch == '[' {
        // ensure this '[' looks like a JSON array start, not "[frame=...]"
        let mut j = i + 1;
        while j < bytes.len() && (bytes[j] as char).is_whitespace() {
          j += 1;
        }
        if j < bytes.len() {
          let next = bytes[j] as char;
          if !matches!(next, '{' | '[' | ']' | '"' | '-' | '0'..='9') {
            continue;
          }
        }
      }
      start = Some(i);
      break;
    }

    if start.is_some() {
      break;
    }

    // fallback: scan whole line
    if pass == 0 && scan_from != 0 {
      scan_from = 0;
      continue;
    }
  }

  let Some(start) = start else { return None };

  let mut stack: Vec<char> = vec![];
  let mut in_string = false;
  let mut escape = false;

  for (idx, c) in line.char_indices().skip_while(|(i, _)| *i < start) {
    if in_string {
      if escape {
        escape = false;
      } else if c == '\\' {
        escape = true;
      } else if c == '"' {
        in_string = false;
      }
      continue;
    }

    if c == '"' {
      in_string = true;
      continue;
    }

    if c == '{' || c == '[' {
      stack.push(c);
      continue;
    }
    if c == '}' {
      if stack.last() != Some(&'{') {
        return None;
      }
      stack.pop();
      if stack.is_empty() {
        return Some(line[start..idx + 1].to_string());
      }
      continue;
    }
    if c == ']' {
      if stack.last() != Some(&'[') {
        return None;
      }
      stack.pop();
      if stack.is_empty() {
        return Some(line[start..idx + 1].to_string());
      }
      continue;
    }
  }

  None
}

fn apply_voice_room_snapshot(payload: &Value) -> Vec<Update> {
  let mut out = vec![];
  
  let camps: Vec<&Value> = if payload.is_array() {
    payload.as_array().unwrap().iter().collect()
  } else {
    vec![payload]
  };

  for camp in camps {
    let camp_id = camp.get("campId").and_then(|v| v.as_u64()).and_then(|u| u8::try_from(u).ok());
    let players = camp.get("campPlayers").and_then(|v| v.as_array()).cloned().unwrap_or_default();
    for p in players {
      let player_id = match p.get("playerId") {
        Some(Value::String(s)) => s.trim().to_string(),
        Some(Value::Number(n)) => n.to_string(),
        _ => continue,
      };
      if player_id.is_empty() {
        continue;
      }
      let display_name = p
        .get("playerName")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());
      let is_ai = p.get("isAi").and_then(parse_boolish);
      let mecha_id = p
        .get("mechaIdSelected")
        .and_then(|v| normalize_mecha_id(v))
        .or_else(|| p.get("mechaId").and_then(|v| normalize_mecha_id(v)));
      let p_camp_id = p.get("campId").and_then(|v| v.as_u64()).and_then(|u| u8::try_from(u).ok());

      out.push(Update::PlayerUpdate(PlayerUpdate {
        player_id,
        display_name,
        is_ai,
        mecha_id,
        camp_id: p_camp_id.or(camp_id),
        ready: None,
      }));
    }
  }
  out
}

/// Parse one log line into 0..N state updates.
pub fn parse_line(line: &str) -> Vec<Update> {
  if line.contains("GAME_S2C_QUERY_COMBAT_RECORD_RESULT") {
    return vec![Update::ResetPlayers];
  }

  // UI prepare screen lines
  if line.contains("UIWarPreparePlayer.cs") && line.contains("playerId") {
    if let Some(caps) = PLAYER_LINE.captures(line) {
      let player_id = caps.name("playerId").map(|m| m.as_str()).unwrap_or("").to_string();
      if !player_id.is_empty() {
        return vec![Update::PlayerUpdate(PlayerUpdate {
          player_id,
          display_name: caps.name("displayName").map(|m| m.as_str().trim().to_string()),
          mecha_id: caps
            .name("mechaId")
            .and_then(|m| m.as_str().trim().parse::<u32>().ok()),
          ready: caps.name("ready").and_then(|m| parse_boolish_str(m.as_str())),
          is_ai: None,
          camp_id: None,
        })];
      }
    }

    if let Some(caps) = MECHA_SELECT_LINE.captures(line) {
      let player_id = caps.name("playerId").map(|m| m.as_str()).unwrap_or("").to_string();
      if !player_id.is_empty() {
        return vec![Update::PlayerUpdate(PlayerUpdate {
          player_id,
          display_name: caps.name("displayName").map(|m| m.as_str().trim().to_string()),
          mecha_id: caps
            .name("mechaId")
            .and_then(|m| m.as_str().trim().parse::<u32>().ok()),
          ready: None,
          is_ai: None,
          camp_id: None,
        })];
      }
    }
  }

  // Ready notice line
  if line.contains("OnEmBattleReadyStateNotice") && line.contains("playerId") {
    if let Some(caps) = READY_NOTICE_LINE.captures(line) {
      let player_id = caps.name("playerId").map(|m| m.as_str()).unwrap_or("").to_string();
      if !player_id.is_empty() {
        return vec![Update::PlayerUpdate(PlayerUpdate {
          player_id,
          ready: caps.name("ready").and_then(|m| parse_boolish_str(m.as_str())),
          display_name: None,
          mecha_id: None,
          is_ai: None,
          camp_id: None,
        })];
      }
    }
  }

  // JSON-carrying lines (UIWarPrepareModule, VoiceModule, etc.)
  let Some(json_blob) = extract_json_object_from_line(line) else { return vec![] };
  let Ok(payload) = serde_json::from_str::<Value>(&json_blob) else { return vec![] };

  // Voice room snapshots (EnterWarVoiceRoom msg=[ ... ])
  if line.contains("EnterWarVoiceRoom") {
    if payload.is_object() && payload.get("campPlayers").is_some() {
      return apply_voice_room_snapshot(&payload);
    }
    if payload.is_array() {
      return apply_voice_room_snapshot(&payload);
    }
  }

  // Mecha selection changes:
  // UIWarPrepareModule: 收到玩家选择机甲协议, { "playerId": "...", "mechaId": 100021, ... }
  if payload.is_object()
    && line.contains("UIWarPrepareModule")
    && line.contains("收到玩家选择机甲协议")
    && payload.get("playerId").is_some()
    && payload.get("mechaId").is_some()
  {
    let player_id = payload
      .get("playerId")
      .and_then(|v| v.as_str().map(|s| s.to_string()).or_else(|| v.as_u64().map(|u| u.to_string())));
    if let Some(player_id) = player_id {
      return vec![Update::PlayerUpdate(PlayerUpdate {
        player_id,
        mecha_id: payload.get("mechaId").and_then(normalize_mecha_id),
        display_name: None,
        ready: None,
        is_ai: None,
        camp_id: None,
      })];
    }
  }

  // Ready updates: { "playerId": "...", "ready": true, "aiMechaDiy": { "mechaId": ... } }
  if payload.is_object() && payload.get("playerId").is_some() && payload.get("ready").is_some() {
    let player_id = payload
      .get("playerId")
      .and_then(|v| v.as_str().map(|s| s.to_string()).or_else(|| v.as_u64().map(|u| u.to_string())));
    if let Some(player_id) = player_id {
      let mut mecha_id = None;
      if let Some(ai) = payload.get("aiMechaDiy") {
        if let Some(mid) = ai.get("mechaId") {
          mecha_id = normalize_mecha_id(mid);
        }
      }
      return vec![Update::PlayerUpdate(PlayerUpdate {
        player_id,
        ready: payload.get("ready").and_then(parse_boolish),
        mecha_id,
        display_name: None,
        is_ai: None,
        camp_id: None,
      })];
    }
  }

  vec![]
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn json_extractor_skips_frame_brackets() {
    let line = r#"foo [frame=123] msg=[{"a":1,"b":[2,3]}] trailing"#;
    let json = extract_json_object_from_line(line).expect("should extract json");
    assert_eq!(json, r#"[{"a":1,"b":[2,3]}]"#);
  }

  #[test]
  fn parses_player_line_and_ready() {
    let line = "UIWarPreparePlayer.cs playerId : 111, displayName : Alice, mechaId : 100020, pilotId : 1, ready : true";
    let ups = parse_line(line);
    assert_eq!(ups.len(), 1);
    match &ups[0] {
      Update::PlayerUpdate(p) => {
        assert_eq!(p.player_id, "111");
        assert_eq!(p.display_name.as_deref(), Some("Alice"));
        assert_eq!(p.mecha_id, Some(100020));
        assert_eq!(p.ready, Some(true));
      }
      _ => panic!("expected player update"),
    }
  }

  #[test]
  fn parses_ready_notice() {
    let line = "OnEmBattleReadyStateNotice playerId: 111 ready: false";
    let ups = parse_line(line);
    assert_eq!(ups.len(), 1);
    match &ups[0] {
      Update::PlayerUpdate(p) => {
        assert_eq!(p.player_id, "111");
        assert_eq!(p.ready, Some(false));
      }
      _ => panic!("expected player update"),
    }
  }
}

