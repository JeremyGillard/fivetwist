//! The exported API, run inside a JavaScript engine: `wasm-pack test --node crates/pentago-wasm`.
//!
//! The rules and the conversions are covered by the native unit tests; these check what only
//! exists across the boundary: thrown errors and the values JavaScript receives.

#![cfg(target_arch = "wasm32")]

use pentago_wasm::{Color, Direction, Outcome, Phase, Quadrant, WasmGame};
use wasm_bindgen::{JsCast, JsError, JsValue};
use wasm_bindgen_test::wasm_bindgen_test;

/// The `message` of the JavaScript `Error` that would be thrown.
fn message(error: JsError) -> String {
    let error: js_sys::Error = JsValue::from(error).unchecked_into();
    error.message().into()
}

#[wasm_bindgen_test]
fn a_turn_can_be_played() {
    let mut game = WasmGame::new();
    assert!(game.place(1, 1).is_ok());
    assert!(
        game.rotate(Quadrant::TopLeft as u32, Direction::Clockwise as u32)
            .is_ok()
    );

    assert_eq!(game.cells()[7], Color::White as u8);
    assert_eq!(game.current(), Color::Black);
    assert_eq!(game.phase(), Phase::Placement);
    assert_eq!(game.outcome(), None);
}

#[wasm_bindgen_test]
fn a_position_off_the_board_throws() {
    let mut game = WasmGame::new();
    let error = game.place(6, 0).unwrap_err();
    assert_eq!(message(error), "this position is off the board");
}

#[wasm_bindgen_test]
fn a_move_against_the_rules_throws_the_core_message() {
    let mut game = WasmGame::new();
    let error = game
        .rotate(Quadrant::TopLeft as u32, Direction::Clockwise as u32)
        .unwrap_err();
    assert_eq!(
        message(error),
        "this action is not allowed in the current phase"
    );
}

#[wasm_bindgen_test]
fn a_won_game_reports_its_outcome() {
    let mut game = WasmGame::new();
    let white = [(0, 0), (1, 0), (2, 0), (3, 0)];
    let black = [(0, 5), (1, 5), (2, 5), (0, 4)];
    for ((white_col, white_row), (black_col, black_row)) in white.into_iter().zip(black) {
        for (col, row) in [(white_col, white_row), (black_col, black_row)] {
            game.place(col, row).unwrap();
            game.rotate(Quadrant::BottomRight as u32, Direction::Clockwise as u32)
                .unwrap();
        }
    }
    game.place(4, 0).unwrap();

    assert_eq!(game.phase(), Phase::Over);
    assert_eq!(game.outcome(), Some(Outcome::WhiteWins));
}

#[wasm_bindgen_test]
fn a_bad_quadrant_throws_and_leaves_the_game_usable() {
    let mut game = WasmGame::new();
    game.place(0, 0).unwrap();

    let error = game.rotate(9, Direction::Clockwise as u32).unwrap_err();
    assert_eq!(message(error), "this is not a quadrant");

    assert_eq!(game.phase(), Phase::Rotation);
    assert!(
        game.rotate(Quadrant::TopLeft as u32, Direction::Clockwise as u32)
            .is_ok()
    );
}
