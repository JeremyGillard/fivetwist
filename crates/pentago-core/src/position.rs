use crate::quadrant::Quadrant;
use crate::{BOARD_SIZE, QUADRANT_SIZE};

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

    /// The position `dcol` columns and `drow` rows away, or `None` if it is off the board.
    pub fn offset(self, dcol: i8, drow: i8) -> Option<Self> {
        let col = self.col.checked_add_signed(dcol)?;
        let row = self.row.checked_add_signed(drow)?;
        Position::new(col, row)
    }

    /// The quadrant this position belongs to.
    pub fn quadrant(self) -> Quadrant {
        match (self.col < QUADRANT_SIZE, self.row < QUADRANT_SIZE) {
            (true, true) => Quadrant::TopLeft,
            (false, true) => Quadrant::TopRight,
            (true, false) => Quadrant::BottomLeft,
            (false, false) => Quadrant::BottomRight,
        }
    }

    /// Every position on the board, row by row.
    pub fn all() -> impl Iterator<Item = Position> {
        (0..BOARD_SIZE).flat_map(|row| (0..BOARD_SIZE).map(move |col| Position { col, row }))
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

    #[test]
    fn offset_inside_the_board() {
        let pos = Position::new(2, 2).unwrap();
        assert_eq!(pos.offset(1, 1), Position::new(3, 3));
    }

    #[test]
    fn negative_offset_inside_the_board() {
        let pos = Position::new(3, 3).unwrap();
        assert_eq!(pos.offset(-1, -1), Position::new(2, 2));
    }

    #[test]
    fn offset_past_the_right_or_bottom_edge_is_none() {
        let pos = Position::new(5, 5).unwrap();
        assert_eq!(pos.offset(1, 0), None);
        assert_eq!(pos.offset(0, 1), None);
    }

    #[test]
    fn offset_past_the_left_or_top_edge_is_none() {
        let pos = Position::new(0, 0).unwrap();
        assert_eq!(pos.offset(-1, 0), None);
        assert_eq!(pos.offset(0, -1), None);
    }

    #[test]
    fn all_yields_every_position_once() {
        let all: Vec<Position> = Position::all().collect();
        assert_eq!(all.len(), 36);
        assert_eq!(all.first(), Position::new(0, 0).as_ref());
        assert_eq!(all.last(), Position::new(5, 5).as_ref());
        for (i, pos) in all.iter().enumerate() {
            assert!(!all[..i].contains(pos));
        }
    }

    #[test]
    fn quadrant_of_each_corner() {
        let quadrant = |col, row| Position::new(col, row).unwrap().quadrant();
        assert_eq!(quadrant(0, 0), Quadrant::TopLeft);
        assert_eq!(quadrant(5, 0), Quadrant::TopRight);
        assert_eq!(quadrant(0, 5), Quadrant::BottomLeft);
        assert_eq!(quadrant(5, 5), Quadrant::BottomRight);
    }

    #[test]
    fn quadrant_changes_at_the_middle_of_the_board() {
        let quadrant = |col, row| Position::new(col, row).unwrap().quadrant();
        assert_eq!(quadrant(2, 2), Quadrant::TopLeft);
        assert_eq!(quadrant(3, 2), Quadrant::TopRight);
        assert_eq!(quadrant(2, 3), Quadrant::BottomLeft);
        assert_eq!(quadrant(3, 3), Quadrant::BottomRight);
    }

    #[test]
    fn every_position_is_in_the_quadrant_that_starts_at_or_before_it() {
        for position in Position::all() {
            let origin = position.quadrant().origin();
            assert!(
                origin.col() <= position.col() && position.col() < origin.col() + QUADRANT_SIZE
            );
            assert!(
                origin.row() <= position.row() && position.row() < origin.row() + QUADRANT_SIZE
            );
        }
    }
}
