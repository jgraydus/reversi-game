use wasm_bindgen::prelude::*;
use web_sys;

use crate::state::*;

// draw the game to the canvas
pub fn draw(
  context: &web_sys::CanvasRenderingContext2d,
  size: f64,
  game_state: &GameState
) {
   clear(context, size);
   draw_game_board(context, size);
   draw_placed_pieces(context, size, game_state);
   draw_side_panel(context, size, game_state);
}

// clear the current content by drawing a white rectangle
// over the entire canvas
fn clear(context: &web_sys::CanvasRenderingContext2d, size: f64) {
    context.begin_path();
    context.set_fill_style(&JsValue::from_str("white"));
    context.move_to(0.0, 0.0);
    context.line_to(size + 200.0, 0.0);
    context.line_to(size + 200.0, size);
    context.line_to(0.0, size);
    context.line_to(0.0, 0.0);
    context.fill();
}

fn draw_game_board(context: &web_sys::CanvasRenderingContext2d, size: f64) {
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
}

fn draw_placed_pieces(context: &web_sys::CanvasRenderingContext2d,
                      size: f64,
                      game_state: &GameState) {
    for row in 0..8 {
        for col in 0..8 {
            let color = game_state.get_position(Pos { row, col });
            if color == Color::Black || color == Color::White {
                draw_piece(context, size, Pos { row, col }, color);
            }
        }
    }
}

fn draw_piece(context: &web_sys::CanvasRenderingContext2d,
              size: f64,
              pos: Pos,
              color: Color) {
    context.begin_path();
    let s = if color == Color::Black { "black" } else { "white" };
    context.set_fill_style(&JsValue::from_str(s));
    let Pos { row, col } = pos;
    let x = size / 16.0 + col as f64 * size / 8.0;
    let y = size / 16.0 + row as f64 * size / 8.0;
    let r = size / 16.0 - 5.0;
    context.arc(x, y, r, 0.0, 2.0 * 3.141592).unwrap();
    context.fill();
}

fn draw_side_panel(context: &web_sys::CanvasRenderingContext2d,
                   size: f64,
                   game_state: &GameState) {
    if game_state.is_game_over() {
        draw_game_over(context, size, game_state);
    } else {
        // draw a piece to show who's turn it is
        context.begin_path();
        context.set_line_width(1.0);
        let color = game_state.get_current_player();
        let s = if color == Color::Black { "black" } else { "white" };
        context.set_fill_style(&JsValue::from_str(s));
        let x = size + 100.0;
        let y = 50.0;
        let r = size / 16.0 - 5.0;
        context.arc(x, y, r, 0.0, 2.0 * 3.141592).unwrap();
        context.fill();
        context.stroke();

        // write text to say whose turn it is
        context.set_font("24pt sans-serif");
        context.set_fill_style(&JsValue::from_str("black"));
        if game_state.get_current_player() == Color::Black {
            context.fill_text("BLACK's turn", size+10.0, 125.0).unwrap();
        } else {
            context.fill_text("WHITE's turn", size+10.0, 125.0).unwrap();
        }

        if game_state.get_all_lines().is_empty() {
            render_pass_button(&context, size as f64);
        }
    }
}

fn draw_game_over(context: &web_sys::CanvasRenderingContext2d,
                  size: f64,
                  game_state: &GameState) {
    context.set_fill_style(&JsValue::from_str("black"));
    context.set_font("36pt sans-serif");
    context.fill_text("GAME", size+20.0, 50.0).unwrap();
    context.fill_text("OVER", size+20.0, 100.0).unwrap();

    let mut black = 0;
    let mut white = 0;
    for row in 0..8 {
        for col in 0..8 {
            let color = game_state.get_position(Pos { row, col });
            if color == Color::Black { black = black + 1; }
            if color == Color::White { white = white + 1; }
        }
    }

    context.set_font("24pt sans-serif");
    context.fill_text(&format!("black: {}", black),
                      size + 20.0, 150.0).unwrap();
    context.fill_text(&format!("white: {}", white),
                      size + 20.0, 180.0).unwrap();

    // draw reset button
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
    context.fill_text("RESET", x+5.0, y+30.0).unwrap();
}

fn render_pass_button(
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

