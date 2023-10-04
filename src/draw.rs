use wasm_bindgen::prelude::*;
use web_sys;

use crate::state::*;

pub fn render_board(
  context: &web_sys::CanvasRenderingContext2d,
  size: f64,
  game_state: &GameState
) {
    // clear
    context.begin_path();
    context.set_fill_style(&JsValue::from_str("white"));
    context.move_to(0.0, 0.0);
    context.line_to(size + 200.0, 0.0);
    context.line_to(size + 200.0, size);
    context.line_to(0.0, size);
    context.line_to(0.0, 0.0);
    context.fill();

    // draw green background
    context.begin_path();
    context.set_line_width(3.0);
    context.set_stroke_style(&JsValue::from_str("black"));
    context.set_fill_style(&JsValue::from_str("green"));
    context.move_to(1.0, 1.0);
    context.line_to(size-1.0, 1.0);
    context.line_to(size+1.0, size-1.0);
    context.line_to(1.0, size-1.0);
    context.line_to(1.0, 1.0);
    context.fill();

    // draw the grid lines
    for i in 1..8 {
        let x = i as f64 * size / 8.0;
        context.move_to(x, 0.0);
        context.line_to(x, size);
    }
    for i in 1..8 {
        let y = i as f64 * size / 8.0;
        context.move_to(0.0, y);
        context.line_to(size, y);
    }
    context.stroke();

    // draw the placed pieces
    for row in 0..8 {
       for col in 0..8 {
           let value = game_state.get_position(Pos { row, col });
           if value == Color::Black || value == Color::White {
               context.begin_path();
               if value == Color::Black {
                   context.set_fill_style(&JsValue::from_str("black"));
               } else {
                   context.set_fill_style(&JsValue::from_str("white"));
               }
               let x = size / 16.0 + col as f64 * size / 8.0;
               let y = size / 16.0 + row as f64 * size / 8.0;
               let r = size / 16.0 - 5.0;
               context.arc(x, y, r, 0.0, 2.0 * 3.141592).unwrap();
               context.fill();
            }
        }
    }

    // draw a piece to show who's turn it is
    context.begin_path();
    context.set_line_width(1.0);
    if game_state.get_current_player() == Color::Black {
        context.set_fill_style(&JsValue::from_str("black"));
    } else {
        context.set_fill_style(&JsValue::from_str("white"));
    }
    let x = size + 100.0;
    let y = 100.0;
    let r = size / 16.0 - 5.0;
    context.arc(x, y, r, 0.0, 2.0 * 3.141592).unwrap();
    context.fill();
    context.stroke();

    // write text to say whose turn it is
    context.set_font("24pt sans-serif");
    context.set_fill_style(&JsValue::from_str("black"));
    if game_state.get_current_player() == Color::Black {
        context.fill_text("BLACK's turn", size+10.0, 175.0).unwrap();
    } else {
        context.fill_text("WHITE's turn", size+10.0, 175.0).unwrap();
    }
}

pub fn render_pass_button(
  context: &web_sys::CanvasRenderingContext2d,
  size: f64) {
    // draw the pass button if there are no moves available
    context.begin_path();
    let x = size+50.0;
    let y = 400.0;
    context.move_to(x,y);
    context.line_to(x,y+40.0);
    context.line_to(x+100.0,y+40.0);
    context.line_to(x+100.0,y);
    context.line_to(x,y);
    context.set_fill_style(&JsValue::from_str("red"));
    context.fill();
    context.stroke();
    context.set_font("20pt sans-serif");
    context.set_fill_style(&JsValue::from_str("black"));
    context.fill_text("PASS", x+15.0, y+30.0).unwrap();
}

