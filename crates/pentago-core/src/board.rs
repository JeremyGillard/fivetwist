use crate::BOARD_SIZE;
use crate::color::Color;
use crate::position::Position;

const SIZE: usize = BOARD_SIZE as usize;

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
}
