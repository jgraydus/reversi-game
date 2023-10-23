use futures::channel::mpsc::{channel, Receiver};
use futures::stream::StreamExt;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures;
use web_sys;

// make printing a console log move convenient
macro_rules! log {
  ( $( $t:tt )* ) => {
    web_sys::console::log_1(&format!( $( $t )* ).into());
  }
}

mod draw;
use draw::*;

mod state;
use state::*;

mod ai;

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

#[wasm_bindgen]
pub fn run() -> Result<(), JsValue> {
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
        let mut human_player = state::Color::Black;

        // channel to pass click events
        let (mut s, mut r) = channel::<(i32,i32)>(10);

        // when the user clicks the canvas, send the location into the channel
        let click_handler: Closure<dyn FnMut(web_sys::PointerEvent)>
          = Closure::new(move |evt: web_sys::PointerEvent| {
                s.try_send((evt.offset_x(), evt.offset_y())).unwrap();
            });
        canvas.set_onclick(Some(click_handler.as_ref().unchecked_ref()));
        click_handler.forget();

        // run the game
        loop {
          // choose color
          draw_game_board(&context, SIZE as f64);
          draw_color_chooser(&context, SIZE as f64);

          let x1 = (SIZE / 2 - 100) as f64;
          let y1 = (SIZE / 2 - 100) as f64;

          while let Some((x, y)) = r.next().await {
            let (x, y) = (x as f64, y as f64);
            if x > x1+20.0 && x < x1+180.0 && y > y1+80.0 && y < y1+120.0 {
              human_player = state::Color::Black;
              break;
            }
            if x > x1+20.0 && x < x1+180.0 && y > y1+140.0 && y < y1+180.0 {
              human_player = state::Color::White;
              break;
            }
          }

          // play!
          draw(&context, SIZE as f64, &game_state);
          while !do_next_turn(&mut game_state, human_player, &mut r).await {
            draw(&context, SIZE as f64, &game_state);
          }
          draw(&context, SIZE as f64, &game_state);

          // wait for user to click 'reset'
          while let Some((x, y)) = r.next().await {
            if    x > SIZE as i32 + 50
               && x < SIZE as i32 + 150
               && y > 400
               && y < 440 {
               game_state.reset();
               break;
            }
          }
        }
    });

    Ok(())
}

async fn do_next_turn(
  game_state: &mut GameState,
  human_player: state::Color,
  r: &mut Receiver<(i32, i32)>
) -> bool {
  if game_state.get_current_player() != human_player {
    let (p, _) = ai::minimax(&game_state, 3, true, state::Color::White);
    apply(game_state, p);
  } else {
    // process each received click
    while let Some((x, y)) = r.next().await {
      // if the user clicks the 'reset' button
      if game_state.is_game_over()
         && x > SIZE as i32 + 50
         && x < SIZE as i32 + 150
         && y > 400
         && y < 440 {
         game_state.reset();
         break;
      }
      // if the user clicks the 'pass' button
      if game_state.get_all_lines().is_empty()
         && x > SIZE as i32 + 50
         && x < SIZE as i32 + 150
         && y > 400
         && y < 440 {
          apply(game_state, None);
          break;
      }
      // if the click is not on the board
      if x > SIZE as i32 {
          continue;
      }
      // convert the clicked position into the board square coord
      let pos = Pos {
          row: (y as f64 / SIZE as f64 * 8.0).floor() as usize,
          col: (x as f64 / SIZE as f64 * 8.0).floor() as usize,
      };
      apply(game_state, Some(pos));
      break;
    }
  }
  game_state.is_game_over()
}
