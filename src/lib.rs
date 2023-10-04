use futures::channel::mpsc::channel;
use futures::stream::StreamExt;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures;
use web_sys;

mod draw;
use draw::*;

mod state;
use state::*;

const SIZE: u32 = 600;

fn get_canvas() -> web_sys::HtmlCanvasElement {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let canvas = document
        .get_element_by_id("canvas")
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .unwrap();
    canvas
}

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    wasm_bindgen_futures::spawn_local(async move {
        // get the canvas and ensure it's the correct size
        let canvas = get_canvas(); 
        canvas.set_height(SIZE);
        canvas.set_width(SIZE + 200); 
   
        // get the rendering context 
        let context = canvas
            .get_context("2d").unwrap().unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>().unwrap();
   
        // set up the initial game state 
        let mut game_state = GameState::new();
   
        // do the initial render 
        render_board(&context, SIZE as f64, &game_state);
    
        // channel to pass click events
        let (mut s, mut r) = channel::<(i32,i32)>(10);
   
        // when the user clicks the canvas, send the location into the channel
        let click_handler: Closure<dyn FnMut(web_sys::PointerEvent)>
          = Closure::new(move |evt: web_sys::PointerEvent| {
                s.try_send((evt.offset_x(), evt.offset_y())).unwrap();
            });
        canvas.set_onclick(Some(click_handler.as_ref().unchecked_ref()));
        click_handler.forget();

        // the currently available moves
        let mut lines: HashMap<Pos, Vec<Vec<Pos>>>
           = compute_all_lines(&game_state);

        // process each received click
        while let Some((x, y)) = r.next().await {
          /*
          let msg = format!("lines: {:?}", lines);
          web_sys::console::log_1(&msg.into());
          let msg = format!("len: {}", lines.len());
          web_sys::console::log_1(&msg.into());
          let msg = format!("x: {}, y: {}", x, y);
          web_sys::console::log_1(&msg.into());
          */

          // if the user clicks the 'pass' button
          if lines.is_empty()
             && x > SIZE as i32 + 50
             && x < SIZE as i32 + 150
             && y > 400
             && y < 440 {
              web_sys::console::log_1(&"HERE".into());
              game_state.set_current_player(
                  if game_state.get_current_player() == Color::Black { Color::White }
                  else { Color::Black });
        
              render_board(&context,
                           SIZE as f64,
                           &game_state);
              lines = compute_all_lines(&game_state);
              if lines.is_empty() { render_pass_button(&context, SIZE as f64); }
              continue;
          }

          // if the click is not on the board
          if x > SIZE as i32 {
              continue;
          }

          let col = (x as f64 / SIZE as f64 * 8.0).floor() as usize;
          let row = (y as f64 / SIZE as f64 * 8.0).floor() as usize;

          let color = game_state.get_current_player();

          if let Some(_) = apply(&mut game_state, Pos {row, col}, color, &lines) {
              game_state.set_current_player(
                  if color == Color::Black { Color::White }
                  else { Color::Black });
              render_board(&context,
                           SIZE as f64,
                           &game_state);
              lines = compute_all_lines(&game_state);
          }

          if lines.is_empty() { render_pass_button(&context, SIZE as f64); }
        }
    });

    Ok(())
}

