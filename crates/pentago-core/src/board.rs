use crate::color::Color;
use crate::direction::Direction;
use crate::position::Position;
use crate::quadrant::Quadrant;
use crate::{BOARD_SIZE, QUADRANT_SIZE};

const SIZE: usize = BOARD_SIZE as usize;
const QUADRANT: usize = QUADRANT_SIZE as usize;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Board {
    cells: [[Option<Color>; SIZE]; SIZE],
}

impl Board {
    pub fn new() -> Self {
        Board {
            cells: [[None; SIZE]; SIZE],
        }
    }

    pub fn get(&self, pos: Position) -> Option<Color> {
        self.cells[usize::from(pos.row())][usize::from(pos.col())]
    }

    /// Places a marble; returns `false` and leaves the board untouched if the cell is occupied.
    pub fn place(&mut self, pos: Position, color: Color) -> bool {
        let cell = &mut self.cells[usize::from(pos.row())][usize::from(pos.col())];
        if cell.is_some() {
            return false;
        }
        *cell = Some(color);
        true
    }

    pub fn is_full(&self) -> bool {
        for row in self.cells {
            for cell in row {
                if cell.is_none() {
                    return false;
                }
            }
        }
        true
    }

    /// Turns one quadrant by 90 degrees.
    pub fn rotate(&mut self, quadrant: Quadrant, direction: Direction) {
        let origin = quadrant.origin();
        let origin_row = usize::from(origin.row());
        let origin_col = usize::from(origin.col());
        let old = self.cells;

        for row in 0..QUADRANT {
            for col in 0..QUADRANT {
                let (from_row, from_col) = match direction {
                    Direction::Clockwise => (QUADRANT - 1 - col, row),
                    Direction::CounterClockwise => (col, QUADRANT - 1 - row),
                };
                self.cells[origin_row + row][origin_col + col] =
                    old[origin_row + from_row][origin_col + from_col];
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pos(col: u8, row: u8) -> Position {
        Position::new(col, row).unwrap()
    }

    #[test]
    fn new_board_is_empty() {
        let board = Board::new();
        assert_eq!(board.get(pos(0, 0)), None);
    }

    #[test]
    fn place_then_get_returns_the_color() {
        let mut board = Board::new();
        assert!(board.place(pos(2, 4), Color::White));
        assert_eq!(board.get(pos(2, 4)), Some(Color::White));
    }

    #[test]
    fn place_leaves_other_cells_empty() {
        let mut board = Board::new();
        board.place(pos(2, 4), Color::White);
        assert_eq!(board.get(pos(4, 2)), None);
    }

    #[test]
    fn place_on_occupied_cell_is_rejected() {
        let mut board = Board::new();
        board.place(pos(1, 1), Color::White);
        assert!(!board.place(pos(1, 1), Color::Black));
        assert_eq!(board.get(pos(1, 1)), Some(Color::White));
    }

    #[test]
    fn new_board_is_not_full() {
        assert!(!Board::new().is_full());
    }

    #[test]
    fn board_with_every_cell_filled_is_full() {
        let mut board = Board::new();
        for row in 0..BOARD_SIZE {
            for col in 0..BOARD_SIZE {
                board.place(pos(col, row), Color::Black);
            }
        }
        assert!(board.is_full());
    }

    #[test]
    fn board_with_one_empty_cell_is_not_full() {
        let mut board = Board::new();
        for row in 0..BOARD_SIZE {
            for col in 0..BOARD_SIZE {
                if (col, row) != (5, 5) {
                    board.place(pos(col, row), Color::Black);
                }
            }
        }
        assert!(!board.is_full());
    }

    /// A board with a single white marble.
    fn board_with_marble_at(col: u8, row: u8) -> Board {
        let mut board = Board::new();
        board.place(pos(col, row), Color::White);
        board
    }

    /// An asymmetric position touching every quadrant.
    fn scattered_board() -> Board {
        let mut board = Board::new();
        board.place(pos(0, 0), Color::White);
        board.place(pos(1, 0), Color::Black);
        board.place(pos(2, 1), Color::White);
        board.place(pos(4, 2), Color::Black);
        board.place(pos(0, 5), Color::White);
        board.place(pos(3, 3), Color::Black);
        board.place(pos(5, 4), Color::White);
        board
    }

    #[test]
    fn rotate_clockwise_moves_top_left_corner_to_top_right() {
        let mut board = board_with_marble_at(0, 0);
        board.rotate(Quadrant::TopLeft, Direction::Clockwise);
        assert_eq!(board.get(pos(2, 0)), Some(Color::White));
        assert_eq!(board.get(pos(0, 0)), None);
    }

    #[test]
    fn rotate_counter_clockwise_moves_top_left_corner_to_bottom_left() {
        let mut board = board_with_marble_at(0, 0);
        board.rotate(Quadrant::TopLeft, Direction::CounterClockwise);
        assert_eq!(board.get(pos(0, 2)), Some(Color::White));
        assert_eq!(board.get(pos(0, 0)), None);
    }

    #[test]
    fn rotate_keeps_the_quadrant_centre() {
        let mut board = board_with_marble_at(1, 1);
        board.rotate(Quadrant::TopLeft, Direction::Clockwise);
        assert_eq!(board.get(pos(1, 1)), Some(Color::White));
    }

    #[test]
    fn rotate_clockwise_moves_top_edge_to_right_edge() {
        let mut board = board_with_marble_at(1, 0);
        board.rotate(Quadrant::TopLeft, Direction::Clockwise);
        assert_eq!(board.get(pos(2, 1)), Some(Color::White));
        assert_eq!(board.get(pos(1, 0)), None);
    }

    #[test]
    fn rotate_uses_the_quadrant_origin() {
        let mut board = board_with_marble_at(3, 3);
        board.rotate(Quadrant::BottomRight, Direction::Clockwise);
        assert_eq!(board.get(pos(5, 3)), Some(Color::White));
        assert_eq!(board.get(pos(3, 3)), None);
    }

    #[test]
    fn rotate_leaves_other_quadrants_untouched() {
        let mut board = board_with_marble_at(3, 0);
        let before = board.clone();
        board.rotate(Quadrant::TopLeft, Direction::Clockwise);
        assert_eq!(board, before);
    }

    #[test]
    fn rotate_then_opposite_restores_the_board() {
        let quadrants = [
            Quadrant::TopLeft,
            Quadrant::TopRight,
            Quadrant::BottomLeft,
            Quadrant::BottomRight,
        ];
        for quadrant in quadrants {
            for direction in [Direction::Clockwise, Direction::CounterClockwise] {
                let mut board = scattered_board();
                board.rotate(quadrant, direction);
                board.rotate(quadrant, direction.opposite());
                assert_eq!(board, scattered_board());
            }
        }
    }

    #[test]
    fn four_rotations_restore_the_board() {
        let mut board = scattered_board();
        for _ in 0..4 {
            board.rotate(Quadrant::TopLeft, Direction::Clockwise);
        }
        assert_eq!(board, scattered_board());
    }

    #[test]
    fn one_rotation_changes_an_asymmetric_board() {
        let mut board = scattered_board();
        board.rotate(Quadrant::TopLeft, Direction::Clockwise);
        assert_ne!(board, scattered_board());
    }
}
