use std::collections::HashMap;

pub const EMPTY: u8 = 0;
pub const BLACK: u8 = 1;
pub const WHITE: u8 = 2;

pub struct GameState {
    positions: [u8; 64]
}

impl GameState {
    pub fn new() -> GameState {
        let mut game_state = GameState { positions: [EMPTY; 64] };
        game_state.set_position(3, 3, WHITE);
        game_state.set_position(3, 4, BLACK);
        game_state.set_position(4, 3, BLACK);
        game_state.set_position(4, 4, WHITE);
        game_state
    }
  
    pub fn get_position(&self, row: usize, col: usize) -> u8 {
       self.positions[col + row*8] 
    }
  
    pub fn set_position(&mut self, row: usize, col: usize, value: u8) {
       self.positions[col + row*8] = value;
    }
}

pub fn compute_all_lines(
  game_state: &GameState,
  color: u8
) -> HashMap<(usize,usize),Vec<Vec<(usize,usize)>>> {
  let mut result = HashMap::new();

  for row in 0..8 {
      for col in 0..8 {
         let lines = compute_lines(game_state, row, col, color);
         if lines.len() > 0 { result.insert((row, col), lines); }
      }
  }

  result
}

pub fn apply(game_state: &mut GameState,
         row: usize,
         col: usize,
         color: u8,
         all_lines: &HashMap<(usize,usize), Vec<Vec<(usize,usize)>>>
) -> Option<()> {
    if game_state.get_position(row, col) != 0 { return None; }

    if let Some(lines) = all_lines.get(&(row, col)) {
        game_state.set_position(row, col, color);
        for line in lines {
           for (r, c) in line {
               game_state.set_position(*r, *c, color);
           }
        }
        return Some(())
    }

    None
}

const DELTAS: [(i8,i8); 8] = [
  (-1,-1),(-1,0),(-1,1),(0,-1),(0,1),(1,-1),(1,0),(1,1)
];

fn compute_lines(game_state: &GameState,
                 row: usize,
                 col: usize,
                 color: u8
) -> Vec<Vec<(usize,usize)>> {
  DELTAS
    .iter()
    .filter_map(|(row_delta, col_delta)| {
       find_line(game_state, row, col, *row_delta, *col_delta, color)
     })
    .collect()
}

#[allow(unreachable_code)]
fn find_line(game_state: &GameState,
             row: usize,
             col: usize,
             row_delta: i8,
             col_delta: i8,
             color: u8
) -> Option<Vec<(usize,usize)>> {
  let tmp = game_state.get_position(row, col);
  if tmp != EMPTY { return None; }

  let mut r = row as i8 + row_delta;
  let mut c = col as i8 + col_delta;
  if r < 0 || r > 7 || c < 0 || c > 7 { return None; }
  let tmp = game_state.get_position(r as usize, c as usize);
  if tmp == EMPTY || tmp == color { return None; }

  let mut result = vec![(r as usize, c as usize)];

  loop {
    r = r + row_delta;
    c = c + col_delta;
    if r < 0 || r > 7 || c < 0 || c > 7 { return None; }
    let tmp = game_state.get_position(r as usize, c as usize);
    if tmp == EMPTY { return None; }
    if tmp == color { return Some(result); }
    result.push((r as usize, c as usize));
  }

  unreachable!()
}

