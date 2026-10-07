#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Clockwise,
    CounterClockwise,
}

impl Direction {
    pub fn opposite(self) -> Self {
        match self {
            Direction::Clockwise => Direction::CounterClockwise,
            Direction::CounterClockwise => Direction::Clockwise,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn opposite_of_clockwise_is_counter_clockwise() {
        assert_eq!(Direction::Clockwise.opposite(), Direction::CounterClockwise);
    }

    #[test]
    fn opposite_of_counter_clockwise_is_clockwise() {
        assert_eq!(Direction::CounterClockwise.opposite(), Direction::Clockwise);
    }
}
