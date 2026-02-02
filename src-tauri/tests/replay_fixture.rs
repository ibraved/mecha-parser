use mecha_tracker::{parser, state::PlayersState};

#[test]
fn replay_fixture_lines_produces_expected_state() {
  let text = include_str!("fixtures/sample_lines.txt");
  let mut state = PlayersState::default();

  for line in text.lines() {
    for up in parser::parse_line(line) {
      match up {
        parser::Update::ResetPlayers => state.clear(),
        parser::Update::PlayerUpdate(pu) => {
          state.apply_update(pu);
        }
      }
    }
  }

  // After the reset line at the end of the fixture, state should be empty.
  let teams = state.snapshot_teams();
  assert_eq!(teams.team1.len(), 0);
  assert_eq!(teams.team2.len(), 0);
  assert_eq!(teams.unassigned.len(), 0);
}

