use crate::BOARD_SIZE;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    col: u8,
    row: u8,
}

impl Position {
    pub fn new(col: u8, row: u8) -> Option<Self> {
        if col < BOARD_SIZE && row < BOARD_SIZE {
            Some(Position { col, row })
        } else {
            None
        }
    }

    pub fn col(self) -> u8 {
        self.col
    }

    pub fn row(self) -> u8 {
        self.row
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn origin_is_valid() {
        assert!(Position::new(0, 0).is_some());
    }

    #[test]
    fn col_out_of_bounds_is_none() {
        assert_eq!(Position::new(BOARD_SIZE, 0), None);
    }

    #[test]
    fn row_out_of_bounds_is_none() {
        assert_eq!(Position::new(0, BOARD_SIZE), None);
    }

    #[test]
    fn upper_boundary_is_valid() {
        assert!(Position::new(BOARD_SIZE - 1, BOARD_SIZE - 1).is_some());
    }

    #[test]
    fn accessors_return_col_and_row() {
        let pos = Position::new(2, 3).unwrap();
        assert_eq!(pos.col(), 2);
        assert_eq!(pos.row(), 3);
    }
}
