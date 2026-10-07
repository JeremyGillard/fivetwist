#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Color {
    White,
    Black,
}

impl Color {
    pub fn opponent(self) -> Self {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn opponent_of_white_is_black() {
        assert_eq!(Color::White.opponent(), Color::Black);
    }

    #[test]
    fn opponent_of_black_is_white() {
        assert_eq!(Color::Black.opponent(), Color::White);
    }

    #[test]
    fn opponent_twice_is_identity() {
        let c = Color::White;
        assert_eq!(c.opponent().opponent(), c);
    }
}
