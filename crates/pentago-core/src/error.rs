use std::error::Error;
use std::fmt;

/// The target cell already holds a marble.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CellOccupied;

impl fmt::Display for CellOccupied {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "this cell is already occupied")
    }
}

impl Error for CellOccupied {}

/// Why a placement or a rotation was refused.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MoveError {
    /// A placement was attempted when a rotation is expected, or the reverse.
    WrongPhase,
    /// The target cell already holds a marble.
    CellOccupied,
    /// The game is finished.
    GameOver,
}

impl fmt::Display for MoveError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            MoveError::WrongPhase => "this action is not allowed in the current phase",
            MoveError::CellOccupied => return CellOccupied.fmt(f),
            MoveError::GameOver => "the game is over",
        };
        write!(f, "{message}")
    }
}

impl Error for MoveError {}

impl From<CellOccupied> for MoveError {
    fn from(_: CellOccupied) -> Self {
        MoveError::CellOccupied
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn each_error_has_its_own_message() {
        let messages = [
            MoveError::WrongPhase.to_string(),
            MoveError::CellOccupied.to_string(),
            MoveError::GameOver.to_string(),
        ];
        assert!(messages.iter().all(|message| !message.is_empty()));
        assert_ne!(messages[0], messages[1]);
        assert_ne!(messages[1], messages[2]);
        assert_ne!(messages[0], messages[2]);
    }

    #[test]
    fn move_error_is_a_std_error() {
        let error: Box<dyn Error> = Box::new(MoveError::GameOver);
        assert_eq!(error.to_string(), "the game is over");
    }

    #[test]
    fn cell_occupied_converts_into_a_move_error() {
        assert_eq!(MoveError::from(CellOccupied), MoveError::CellOccupied);
        assert_eq!(
            MoveError::CellOccupied.to_string(),
            CellOccupied.to_string()
        );
    }
}
