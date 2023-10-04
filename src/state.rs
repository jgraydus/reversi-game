use std::collections::HashMap;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Color { Empty, Black, White }

#[derive(Copy, Clone)]
pub struct GameState {
    positions: [Color; 64],
    current_player: Color,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Pos { pub row: usize, pub col: usize }

impl GameState {
    pub fn new() -> GameState {
        let mut game_state = GameState {
            positions: [Color::Empty; 64],
            current_player: Color::Black,
        };
        game_state.set_position(Pos { row: 3, col: 3 }, Color::White);
        game_state.set_position(Pos { row: 3, col: 4 }, Color::Black);
        game_state.set_position(Pos { row: 4, col: 3 }, Color::Black);
        game_state.set_position(Pos { row: 4, col: 4 }, Color::White);
        game_state
    }
  
    pub fn get_position(&self, pos: Pos) -> Color {
        let Pos { row, col } = pos;
        self.positions[col + row*8] 
    }
  
    fn set_position(&mut self, pos: Pos, value: Color) {
        let Pos { row, col } = pos;
        self.positions[col + row*8] = value;
    }

    pub fn get_current_player(&self) -> Color {
        self.current_player
    }

    pub fn set_current_player(&mut self, value: Color) {
        self.current_player = value;
    }
}

pub fn apply(game_state: &mut GameState,
         pos: Pos,
         color: Color,
         all_lines: &HashMap<Pos, Vec<Vec<Pos>>>
) -> Option<()> {
    if game_state.get_position(pos) != Color::Empty { return None; }

    if let Some(lines) = all_lines.get(&pos) {
        game_state.set_position(pos, color);
        for line in lines {
           for Pos { row: r, col: c } in line {
               game_state.set_position(Pos { row: *r, col: *c }, color);
           }
        }
        return Some(())
    }

    None
}

pub fn compute_all_lines(
  game_state: &GameState,
) -> HashMap<Pos,Vec<Vec<Pos>>> {
  let mut result = HashMap::new();

  for row in 0..8 {
      for col in 0..8 {
         let lines = compute_lines(game_state, Pos { row, col });
         if lines.len() > 0 { result.insert(Pos { row, col }, lines); }
      }
  }

  result
}

const DELTAS: [(i8,i8); 8] = [
  (-1,-1),(-1,0),(-1,1),(0,-1),(0,1),(1,-1),(1,0),(1,1)
];

fn compute_lines(
    game_state: &GameState, pos: Pos
) -> Vec<Vec<Pos>> {
  DELTAS
    .iter()
    .filter_map(|(row_delta, col_delta)| {
       find_line(game_state, pos, *row_delta, *col_delta)
     })
    .collect()
}

#[allow(unreachable_code)]
fn find_line(game_state: &GameState,
             pos: Pos,
             row_delta: i8,
             col_delta: i8
) -> Option<Vec<Pos>> {
  let tmp = game_state.get_position(pos);
  if tmp != Color::Empty { return None; }

  let Pos { row, col } = pos;
  let mut r = row as i8 + row_delta;
  let mut c = col as i8 + col_delta;
  if r < 0 || r > 7 || c < 0 || c > 7 { return None; }
  let tmp = game_state.get_position(Pos { row: r as usize, col: c as usize});
  if tmp == Color::Empty || tmp == game_state.get_current_player() {
      return None;
  }

  let mut result = vec![Pos { row: r as usize, col: c as usize }];

  loop {
    r = r + row_delta;
    c = c + col_delta;
    if r < 0 || r > 7 || c < 0 || c > 7 { return None; }
    let tmp = game_state.get_position(Pos { row: r as usize, col: c as usize });
    if tmp == Color::Empty { return None; }
    if tmp == game_state.get_current_player() { return Some(result); }
    result.push(Pos { row: r as usize, col: c as usize });
  }

  unreachable!()
}

