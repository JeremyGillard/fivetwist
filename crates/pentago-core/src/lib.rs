//! Rules of Pentago, with no rendering and no I/O.
//!
//! A [`Game`] alternates placements and rotations until it is [`Phase::Over`].
//!
//! ```
//! use pentago_core::{Color, Direction, Game, Phase, Position, Quadrant};
//!
//! let mut game = Game::new();
//! let centre = Position::new(1, 1).expect("on the board");
//!
//! game.place(centre)?;
//! game.rotate(Quadrant::TopLeft, Direction::Clockwise)?;
//!
//! assert_eq!(game.board().get(centre), Some(Color::White));
//! assert_eq!(game.current(), Color::Black);
//! assert_eq!(game.phase(), Phase::Placement);
//! # Ok::<(), pentago_core::MoveError>(())
//! ```

mod board;
mod color;
mod direction;
mod error;
mod game;
mod position;
mod quadrant;

pub use board::Board;
pub use color::Color;
pub use direction::Direction;
pub use error::{CellOccupied, MoveError};
pub use game::{Game, Outcome, Phase};
pub use position::Position;
pub use quadrant::Quadrant;

pub const BOARD_SIZE: u8 = 6;
pub const QUADRANT_SIZE: u8 = BOARD_SIZE / 2;
pub const WIN_LENGTH: u8 = 5;
