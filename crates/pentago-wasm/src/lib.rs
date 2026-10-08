//! JavaScript bindings for [`pentago_core`].
//!
//! JavaScript holds a [`WasmGame`] and can only change it through [`WasmGame::place`] and
//! [`WasmGame::rotate`], which apply the rules. Everything else is a read-only copy of the state.
//!
//! Only numbers, flat enums and byte arrays cross the boundary, so this crate mirrors the core's
//! enums and converts between the two.
//!
//! Every value coming from JavaScript is checked here, even where TypeScript already restricts
//! it: an unchecked bad value would make the generated glue throw while the game is borrowed,
//! and leave the object unusable.

use std::error::Error;
use std::fmt;

use pentago_core::{Game, MoveError, Position};
use wasm_bindgen::prelude::*;

/// Value of an empty cell in [`WasmGame::cells`].
const EMPTY: u8 = 0;

/// A side. The numeric values are the ones used by `cells()`.
#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    White = 1,
    Black = 2,
}

#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Quadrant {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Clockwise,
    CounterClockwise,
}

/// What the game expects next. Once it is `Over`, `outcome()` says how it ended.
#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Phase {
    Placement,
    Rotation,
    Over,
}

#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Outcome {
    WhiteWins,
    BlackWins,
    Draw,
}

impl From<pentago_core::Color> for Color {
    fn from(color: pentago_core::Color) -> Self {
        match color {
            pentago_core::Color::White => Color::White,
            pentago_core::Color::Black => Color::Black,
        }
    }
}

impl From<Quadrant> for pentago_core::Quadrant {
    fn from(quadrant: Quadrant) -> Self {
        match quadrant {
            Quadrant::TopLeft => pentago_core::Quadrant::TopLeft,
            Quadrant::TopRight => pentago_core::Quadrant::TopRight,
            Quadrant::BottomLeft => pentago_core::Quadrant::BottomLeft,
            Quadrant::BottomRight => pentago_core::Quadrant::BottomRight,
        }
    }
}

impl From<Direction> for pentago_core::Direction {
    fn from(direction: Direction) -> Self {
        match direction {
            Direction::Clockwise => pentago_core::Direction::Clockwise,
            Direction::CounterClockwise => pentago_core::Direction::CounterClockwise,
        }
    }
}

impl From<pentago_core::Phase> for Phase {
    fn from(phase: pentago_core::Phase) -> Self {
        match phase {
            pentago_core::Phase::Placement => Phase::Placement,
            pentago_core::Phase::Rotation => Phase::Rotation,
            pentago_core::Phase::Over(_) => Phase::Over,
        }
    }
}

impl From<pentago_core::Outcome> for Outcome {
    fn from(outcome: pentago_core::Outcome) -> Self {
        match outcome {
            pentago_core::Outcome::Win(pentago_core::Color::White) => Outcome::WhiteWins,
            pentago_core::Outcome::Win(pentago_core::Color::Black) => Outcome::BlackWins,
            pentago_core::Outcome::Draw => Outcome::Draw,
        }
    }
}

impl Quadrant {
    const ALL: [Quadrant; 4] = [
        Quadrant::TopLeft,
        Quadrant::TopRight,
        Quadrant::BottomLeft,
        Quadrant::BottomRight,
    ];

    /// The quadrant JavaScript designates by this number, if there is one.
    fn from_js(value: u32) -> Option<Self> {
        Self::ALL
            .into_iter()
            .find(|&quadrant| quadrant as u32 == value)
    }
}

impl Direction {
    const ALL: [Direction; 2] = [Direction::Clockwise, Direction::CounterClockwise];

    /// The direction JavaScript designates by this number, if there is one.
    fn from_js(value: u32) -> Option<Self> {
        Self::ALL
            .into_iter()
            .find(|&direction| direction as u32 == value)
    }
}

fn encode_cell(cell: Option<pentago_core::Color>) -> u8 {
    cell.map_or(EMPTY, |color| Color::from(color) as u8)
}

/// Why an action coming from JavaScript was refused.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ActionError {
    /// The coordinates do not name a cell; the core cannot even represent them.
    OffBoard,
    UnknownQuadrant,
    UnknownDirection,
    Refused(MoveError),
}

impl fmt::Display for ActionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ActionError::OffBoard => write!(f, "this position is off the board"),
            ActionError::UnknownQuadrant => write!(f, "this is not a quadrant"),
            ActionError::UnknownDirection => write!(f, "this is not a rotation direction"),
            ActionError::Refused(error) => error.fmt(f),
        }
    }
}

impl Error for ActionError {}

impl From<MoveError> for ActionError {
    fn from(error: MoveError) -> Self {
        ActionError::Refused(error)
    }
}

/// A game of Pentago, owned by JavaScript.
#[wasm_bindgen]
#[derive(Debug, Default)]
pub struct WasmGame {
    game: Game,
}

#[wasm_bindgen]
impl WasmGame {
    /// A new game: empty board, White places first.
    #[wasm_bindgen(constructor)]
    pub fn new() -> WasmGame {
        WasmGame::default()
    }

    /// Places a marble of the current color. Throws if the move is not allowed.
    ///
    /// `col` and `row` go from 0 to 5, starting at the top-left corner.
    pub fn place(&mut self, col: u32, row: u32) -> Result<(), JsError> {
        Ok(self.try_place(col, row)?)
    }

    /// Rotates a quadrant, which ends the turn. Throws if the move is not allowed.
    pub fn rotate(
        &mut self,
        #[wasm_bindgen(unchecked_param_type = "Quadrant")] quadrant: u32,
        #[wasm_bindgen(unchecked_param_type = "Direction")] direction: u32,
    ) -> Result<(), JsError> {
        Ok(self.try_rotate(quadrant, direction)?)
    }

    /// The 36 cells, row by row from the top-left corner: the cell at (`col`, `row`) is at index
    /// `row * 6 + col`. Each value is 0 for an empty cell, or a `Color`.
    pub fn cells(&self) -> Vec<u8> {
        Position::all()
            .map(|position| encode_cell(self.game.board().get(position)))
            .collect()
    }

    /// The color whose turn it is (the last mover once the game is over).
    pub fn current(&self) -> Color {
        self.game.current().into()
    }

    pub fn phase(&self) -> Phase {
        self.game.phase().into()
    }

    /// How the game ended, or `undefined` while it is still being played.
    pub fn outcome(&self) -> Option<Outcome> {
        match self.game.phase() {
            pentago_core::Phase::Over(outcome) => Some(outcome.into()),
            pentago_core::Phase::Placement | pentago_core::Phase::Rotation => None,
        }
    }
}

// The logic behind the exported methods, kept free of JavaScript types so that it runs in
// ordinary `cargo test`.
impl WasmGame {
    fn try_place(&mut self, col: u32, row: u32) -> Result<(), ActionError> {
        // JavaScript can pass any number; an 8-bit parameter would silently wrap 256 to 0.
        let position = u8::try_from(col)
            .ok()
            .zip(u8::try_from(row).ok())
            .and_then(|(col, row)| Position::new(col, row))
            .ok_or(ActionError::OffBoard)?;
        self.game.place(position)?;
        Ok(())
    }

    fn try_rotate(&mut self, quadrant: u32, direction: u32) -> Result<(), ActionError> {
        let quadrant = Quadrant::from_js(quadrant).ok_or(ActionError::UnknownQuadrant)?;
        let direction = Direction::from_js(direction).ok_or(ActionError::UnknownDirection)?;
        self.game.rotate(quadrant.into(), direction.into())?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rotate(
        game: &mut WasmGame,
        quadrant: Quadrant,
        direction: Direction,
    ) -> Result<(), ActionError> {
        game.try_rotate(quadrant as u32, direction as u32)
    }

    /// Places at (`col`, `row`), then turns the bottom-right quadrant.
    fn play_turn(game: &mut WasmGame, col: u32, row: u32) {
        game.try_place(col, row).unwrap();
        rotate(game, Quadrant::BottomRight, Direction::Clockwise).unwrap();
    }

    /// White takes the top row while Black plays the bottom-left corner.
    fn game_won_by_white() -> WasmGame {
        let mut game = WasmGame::new();
        let white = [(0, 0), (1, 0), (2, 0), (3, 0)];
        let black = [(0, 5), (1, 5), (2, 5), (0, 4)];
        for ((white_col, white_row), (black_col, black_row)) in white.into_iter().zip(black) {
            play_turn(&mut game, white_col, white_row);
            play_turn(&mut game, black_col, black_row);
        }
        game.try_place(4, 0).unwrap();
        game
    }

    #[test]
    fn cells_are_encoded_as_empty_or_a_color() {
        assert_eq!(encode_cell(None), 0);
        assert_eq!(encode_cell(Some(pentago_core::Color::White)), 1);
        assert_eq!(encode_cell(Some(pentago_core::Color::Black)), 2);
        assert_eq!(Color::White as u8, 1);
        assert_eq!(Color::Black as u8, 2);
    }

    #[test]
    fn quadrants_convert_to_the_core_ones() {
        let pairs = [
            (Quadrant::TopLeft, pentago_core::Quadrant::TopLeft),
            (Quadrant::TopRight, pentago_core::Quadrant::TopRight),
            (Quadrant::BottomLeft, pentago_core::Quadrant::BottomLeft),
            (Quadrant::BottomRight, pentago_core::Quadrant::BottomRight),
        ];
        for (quadrant, expected) in pairs {
            assert_eq!(pentago_core::Quadrant::from(quadrant), expected);
        }
    }

    #[test]
    fn directions_convert_to_the_core_ones() {
        assert_eq!(
            pentago_core::Direction::from(Direction::Clockwise),
            pentago_core::Direction::Clockwise
        );
        assert_eq!(
            pentago_core::Direction::from(Direction::CounterClockwise),
            pentago_core::Direction::CounterClockwise
        );
    }

    #[test]
    fn outcomes_name_the_winner() {
        use pentago_core::Outcome::{Draw, Win};

        assert_eq!(
            Outcome::from(Win(pentago_core::Color::White)),
            Outcome::WhiteWins
        );
        assert_eq!(
            Outcome::from(Win(pentago_core::Color::Black)),
            Outcome::BlackWins
        );
        assert_eq!(Outcome::from(Draw), Outcome::Draw);
    }

    #[test]
    fn new_game_is_empty_with_white_to_place() {
        let game = WasmGame::new();
        assert_eq!(game.cells(), [0; 36]);
        assert_eq!(game.current(), Color::White);
        assert_eq!(game.phase(), Phase::Placement);
        assert_eq!(game.outcome(), None);
    }

    #[test]
    fn cells_are_listed_row_by_row() {
        let mut game = WasmGame::new();
        game.try_place(1, 2).unwrap();
        let cells = game.cells();
        assert_eq!(cells[2 * 6 + 1], Color::White as u8);
        assert_eq!(cells.iter().filter(|&&cell| cell != EMPTY).count(), 1);
    }

    #[test]
    fn place_then_rotate_passes_the_turn() {
        let mut game = WasmGame::new();
        game.try_place(1, 1).unwrap();
        assert_eq!(game.phase(), Phase::Rotation);
        assert_eq!(game.current(), Color::White);

        rotate(&mut game, Quadrant::TopLeft, Direction::Clockwise).unwrap();
        assert_eq!(game.phase(), Phase::Placement);
        assert_eq!(game.current(), Color::Black);
    }

    #[test]
    fn black_marbles_are_encoded_as_black() {
        let mut game = WasmGame::new();
        play_turn(&mut game, 0, 0);
        game.try_place(1, 0).unwrap();
        assert_eq!(game.cells()[..2], [Color::White as u8, Color::Black as u8]);
    }

    #[test]
    fn rotation_is_applied_to_the_chosen_quadrant_and_direction() {
        let mut game = WasmGame::new();
        game.try_place(3, 0).unwrap();
        rotate(&mut game, Quadrant::TopRight, Direction::CounterClockwise).unwrap();
        // The top-left corner of the top-right quadrant goes to its bottom-left corner.
        assert_eq!(game.cells()[2 * 6 + 3], Color::White as u8);
    }

    #[test]
    fn positions_off_the_board_are_refused() {
        let mut game = WasmGame::new();
        for (col, row) in [(6, 0), (0, 6), (256, 0), (0, 256), (u32::MAX, u32::MAX)] {
            assert_eq!(game.try_place(col, row), Err(ActionError::OffBoard));
        }
        assert_eq!(game.cells(), [0; 36]);
        assert_eq!(game.phase(), Phase::Placement);
    }

    #[test]
    fn every_quadrant_and_direction_is_found_by_its_number() {
        for quadrant in Quadrant::ALL {
            assert_eq!(Quadrant::from_js(quadrant as u32), Some(quadrant));
        }
        for direction in Direction::ALL {
            assert_eq!(Direction::from_js(direction as u32), Some(direction));
        }
    }

    #[test]
    fn unknown_quadrants_and_directions_are_refused() {
        let mut game = WasmGame::new();
        game.try_place(0, 0).unwrap();
        let before = game.cells();

        assert_eq!(game.try_rotate(4, 0), Err(ActionError::UnknownQuadrant));
        assert_eq!(game.try_rotate(0, 2), Err(ActionError::UnknownDirection));
        assert_eq!(
            game.try_rotate(u32::MAX, u32::MAX),
            Err(ActionError::UnknownQuadrant)
        );
        assert_eq!(game.cells(), before);
        assert_eq!(game.phase(), Phase::Rotation);
    }

    #[test]
    fn moves_refused_by_the_rules_keep_their_reason() {
        let mut game = WasmGame::new();
        assert_eq!(
            rotate(&mut game, Quadrant::TopLeft, Direction::Clockwise),
            Err(ActionError::Refused(MoveError::WrongPhase))
        );

        play_turn(&mut game, 1, 1);
        assert_eq!(
            game.try_place(1, 1),
            Err(ActionError::Refused(MoveError::CellOccupied))
        );
    }

    #[test]
    fn action_errors_read_like_the_core_ones() {
        assert_eq!(
            ActionError::OffBoard.to_string(),
            "this position is off the board"
        );
        assert_eq!(
            ActionError::from(MoveError::CellOccupied).to_string(),
            MoveError::CellOccupied.to_string()
        );
    }

    #[test]
    fn a_finished_game_reports_its_outcome_and_refuses_more_moves() {
        let mut game = game_won_by_white();
        assert_eq!(game.phase(), Phase::Over);
        assert_eq!(game.outcome(), Some(Outcome::WhiteWins));
        assert_eq!(
            game.try_place(5, 5),
            Err(ActionError::Refused(MoveError::GameOver))
        );
    }
}
