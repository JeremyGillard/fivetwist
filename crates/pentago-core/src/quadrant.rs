use crate::QUADRANT_SIZE;
use crate::position::Position;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Quadrant {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

impl Quadrant {
    pub const ALL: [Quadrant; 4] = [
        Quadrant::TopLeft,
        Quadrant::TopRight,
        Quadrant::BottomLeft,
        Quadrant::BottomRight,
    ];

    /// Top-left cell of the quadrant on the board.
    pub fn origin(self) -> Position {
        let (col, row) = match self {
            Quadrant::TopLeft => (0, 0),
            Quadrant::TopRight => (QUADRANT_SIZE, 0),
            Quadrant::BottomLeft => (0, QUADRANT_SIZE),
            Quadrant::BottomRight => (QUADRANT_SIZE, QUADRANT_SIZE),
        };
        Position::new(col, row).expect("quadrant origin is on the board")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn origin_of_top_left() {
        assert_eq!(Quadrant::TopLeft.origin(), Position::new(0, 0).unwrap());
    }

    #[test]
    fn origin_of_top_right() {
        assert_eq!(Quadrant::TopRight.origin(), Position::new(3, 0).unwrap());
    }

    #[test]
    fn origin_of_bottom_left() {
        assert_eq!(Quadrant::BottomLeft.origin(), Position::new(0, 3).unwrap());
    }

    #[test]
    fn origin_of_bottom_right() {
        assert_eq!(Quadrant::BottomRight.origin(), Position::new(3, 3).unwrap());
    }

    #[test]
    fn all_lists_each_quadrant_once() {
        for (i, quadrant) in Quadrant::ALL.iter().enumerate() {
            assert!(!Quadrant::ALL[..i].contains(quadrant));
        }
    }
}
