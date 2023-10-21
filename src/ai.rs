use crate::state;
use crate::state::{GameState,Pos};

fn apply(game_state: GameState, pos: Option<Pos>
) -> Option<GameState> {
  let mut gs = game_state;
  if let Some(_) = state::apply(&mut gs, pos) {
    Some(gs)
  } else {
    None
  }
}

pub fn minimax(game_state: &GameState,
               depth: usize,
               maximizing_player: bool,
               maximizing_color: state::Color) -> (Option<Pos>, f64) {
  if depth == 0 || game_state.is_game_over() {
    return (None, score(game_state, maximizing_color));
  }
  if maximizing_player {
    let mut value = f64::NEG_INFINITY;
    let mut pos = None;
    if game_state.all_lines.is_empty() {
      if let Some(new_state) = apply(game_state.clone(), None) {
        let (_, v) = minimax(&new_state, depth-1, false, maximizing_color);
        if v > value {
          value = v;
          pos = None;
        }
      }
    } else {
      for p in game_state.all_lines.keys() {
        if let Some(new_state) = apply(game_state.clone(), Some(p.clone())) {
          let (_, v) = minimax(&new_state, depth-1, false, maximizing_color);
          if v > value {
            value = v;
            pos = Some(p.clone());
          }
        }
      }
    }
    (pos, value)
  } else {
    let mut value = f64::INFINITY;
    let mut pos = None;
    if game_state.all_lines.is_empty() {
      if let Some(new_state) = apply(game_state.clone(), None) {
        let (_, v) = minimax(&new_state, depth-1, true, maximizing_color);
        if v < value {
          value = v;
          pos = None;
        }
      }
    } else {
      for p in game_state.all_lines.keys() {
        if let Some(new_state) = apply(game_state.clone(), Some(p.clone())) {
          let (_, v) = minimax(&new_state, depth-1, true, maximizing_color);
          if v < value {
            value = v;
            pos = Some(p.clone());
          }
        }
      }
    }
    (pos, value)
  }
}

fn score(game_state: &GameState, maximizing_color: state::Color) -> f64 {
  let mut result = 0;
  for row in 0..7 {
    for col in 0..7 {
      if game_state.get_position(Pos { row, col }) == maximizing_color {
        result = result + 1;
      } else {
        result = result - 1;
      }
    }
  }
  result as f64
}

