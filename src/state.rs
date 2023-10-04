use std::collections::HashMap;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Color { Empty, Black, White }

pub struct GameState {
    positions: [Color; 64],
    current_player: Color,
}

impl GameState {
    pub fn new() -> GameState {
        let mut game_state = GameState {
            positions: [Color::Empty; 64],
            current_player: Color::Black,
        };
        game_state.set_position(3, 3, Color::White);
        game_state.set_position(3, 4, Color::Black);
        game_state.set_position(4, 3, Color::Black);
        game_state.set_position(4, 4, Color::White);
        game_state
    }
  
    pub fn get_position(&self, row: usize, col: usize) -> Color {
        self.positions[col + row*8] 
    }
  
    pub fn set_position(&mut self, row: usize, col: usize, value: Color) {
        self.positions[col + row*8] = value;
    }

    pub fn get_current_player(&self) -> Color {
        self.current_player
    }

    pub fn set_current_player(&mut self, value: Color) {
        self.current_player = value;
    }
}

pub fn compute_all_lines(
  game_state: &GameState,
) -> HashMap<(usize,usize),Vec<Vec<(usize,usize)>>> {
  let mut result = HashMap::new();

  for row in 0..8 {
      for col in 0..8 {
         let lines = compute_lines(game_state,
                                   row,
                                   col,
                                   game_state.get_current_player());
         if lines.len() > 0 { result.insert((row, col), lines); }
      }
  }

  result
}

pub fn apply(game_state: &mut GameState,
         row: usize,
         col: usize,
         color: Color,
         all_lines: &HashMap<(usize,usize), Vec<Vec<(usize,usize)>>>
) -> Option<()> {
    if game_state.get_position(row, col) != Color::Empty { return None; }

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
                 color: Color
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
             color: Color
) -> Option<Vec<(usize,usize)>> {
  let tmp = game_state.get_position(row, col);
  if tmp != Color::Empty { return None; }

  let mut r = row as i8 + row_delta;
  let mut c = col as i8 + col_delta;
  if r < 0 || r > 7 || c < 0 || c > 7 { return None; }
  let tmp = game_state.get_position(r as usize, c as usize);
  if tmp == Color::Empty || tmp == game_state.get_current_player() {
      return None;
  }

  let mut result = vec![(r as usize, c as usize)];

  loop {
    r = r + row_delta;
    c = c + col_delta;
    if r < 0 || r > 7 || c < 0 || c > 7 { return None; }
    let tmp = game_state.get_position(r as usize, c as usize);
    if tmp == Color::Empty { return None; }
    if tmp == color { return Some(result); }
    result.push((r as usize, c as usize));
  }

  unreachable!()
}

