use crate::board::Board;
use crate::color::Color;
use crate::direction::Direction;
use crate::error::MoveError;
use crate::position::Position;
use crate::quadrant::Quadrant;

/// How a game ended.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Outcome {
    Win(Color),
    Draw,
}

/// What the game expects next.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Phase {
    Placement,
    Rotation,
    Over(Outcome),
}

/// A game of Pentago: a turn is one placement followed by one rotation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Game {
    board: Board,
    current: Color,
    phase: Phase,
}

impl Game {
    /// A new game: empty board, White places first.
    pub fn new() -> Self {
        Game {
            board: Board::new(),
            current: Color::White,
            phase: Phase::Placement,
        }
    }

    pub fn board(&self) -> &Board {
        &self.board
    }

    /// The color whose turn it is (the last mover once the game is over).
    pub fn current(&self) -> Color {
        self.current
    }

    pub fn phase(&self) -> Phase {
        self.phase
    }

    /// The cells of every winning line on the board, in board order.
    ///
    /// Empty unless the game ended with five in a row; after a draw where both colors lined
    /// up, it holds the cells of both.
    pub fn winning_cells(&self) -> Vec<Position> {
        let white = self.board.winning_cells(Color::White);
        let black = self.board.winning_cells(Color::Black);
        Position::all()
            .filter(|position| white.contains(position) || black.contains(position))
            .collect()
    }

    /// Places a marble of the current color.
    ///
    /// Five in a row ends the game at once, without a rotation.
    pub fn place(&mut self, pos: Position) -> Result<(), MoveError> {
        match self.phase {
            Phase::Placement => {}
            Phase::Rotation => return Err(MoveError::WrongPhase),
            Phase::Over(_) => return Err(MoveError::GameOver),
        }
        self.board.place(pos, self.current)?;

        self.phase = if self.board.has_winning_line(self.current) {
            Phase::Over(Outcome::Win(self.current))
        } else {
            Phase::Rotation
        };
        Ok(())
    }

    /// Rotates a quadrant, which ends the turn.
    pub fn rotate(&mut self, quadrant: Quadrant, direction: Direction) -> Result<(), MoveError> {
        match self.phase {
            Phase::Rotation => {}
            Phase::Placement => return Err(MoveError::WrongPhase),
            Phase::Over(_) => return Err(MoveError::GameOver),
        }
        self.board.rotate(quadrant, direction);

        match self.outcome_after_rotation() {
            Some(outcome) => self.phase = Phase::Over(outcome),
            None => {
                self.current = self.current.opponent();
                self.phase = Phase::Placement;
            }
        }
        Ok(())
    }

    /// A rotation can align either color, or both at once.
    fn outcome_after_rotation(&self) -> Option<Outcome> {
        let white = self.board.has_winning_line(Color::White);
        let black = self.board.has_winning_line(Color::Black);
        match (white, black) {
            (true, true) => Some(Outcome::Draw),
            (true, false) => Some(Outcome::Win(Color::White)),
            (false, true) => Some(Outcome::Win(Color::Black)),
            (false, false) if self.board.is_full() => Some(Outcome::Draw),
            (false, false) => None,
        }
    }

    #[cfg(test)]
    fn from_parts(board: Board, current: Color, phase: Phase) -> Self {
        Game {
            board,
            current,
            phase,
        }
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pos(col: u8, row: u8) -> Position {
        Position::new(col, row).unwrap()
    }

    fn place_all(board: &mut Board, color: Color, cells: &[(u8, u8)]) {
        for &(col, row) in cells {
            board.place(pos(col, row), color).unwrap();
        }
    }

    /// Blocks two rows high and three columns wide, alternating: no five in a row.
    fn block_color(position: Position) -> Color {
        if (position.row() / 2 + position.col() / 3).is_multiple_of(2) {
            Color::White
        } else {
            Color::Black
        }
    }

    /// A board filled with the block pattern, except for the cells in `empty`.
    fn blocks_board_without(empty: &[(u8, u8)]) -> Board {
        let mut board = Board::new();
        for position in Position::all() {
            if !empty.contains(&(position.col(), position.row())) {
                board.place(position, block_color(position)).unwrap();
            }
        }
        board
    }

    /// White gets the top row when the top-left quadrant turns clockwise.
    fn white_one_rotation_from_a_line() -> Board {
        let mut board = Board::new();
        place_all(
            &mut board,
            Color::White,
            &[(0, 0), (0, 1), (0, 2), (3, 0), (4, 0)],
        );
        board
    }

    #[test]
    fn new_game_starts_with_white_placing_on_an_empty_board() {
        let game = Game::new();
        assert_eq!(game.phase(), Phase::Placement);
        assert_eq!(game.current(), Color::White);
        assert_eq!(game.board(), &Board::new());
    }

    #[test]
    fn place_puts_the_current_color_and_expects_a_rotation() {
        let mut game = Game::new();
        assert_eq!(game.place(pos(2, 3)), Ok(()));
        assert_eq!(game.board().get(pos(2, 3)), Some(Color::White));
        assert_eq!(game.phase(), Phase::Rotation);
        assert_eq!(game.current(), Color::White);
    }

    #[test]
    fn place_during_rotation_is_refused() {
        let mut game = Game::new();
        game.place(pos(2, 3)).unwrap();
        let before = game.clone();
        assert_eq!(game.place(pos(4, 4)), Err(MoveError::WrongPhase));
        assert_eq!(game, before);
    }

    #[test]
    fn place_on_an_occupied_cell_is_refused() {
        let mut game = Game::new();
        game.place(pos(1, 1)).unwrap();
        game.rotate(Quadrant::TopLeft, Direction::Clockwise)
            .unwrap();
        let before = game.clone();
        assert_eq!(game.place(pos(1, 1)), Err(MoveError::CellOccupied));
        assert_eq!(game, before);
    }

    #[test]
    fn rotate_during_placement_is_refused() {
        let mut game = Game::new();
        assert_eq!(
            game.rotate(Quadrant::TopLeft, Direction::Clockwise),
            Err(MoveError::WrongPhase)
        );
        assert_eq!(game, Game::new());
    }

    #[test]
    fn rotate_turns_the_board_and_passes_the_turn() {
        let mut game = Game::new();
        game.place(pos(0, 0)).unwrap();
        assert_eq!(game.rotate(Quadrant::TopLeft, Direction::Clockwise), Ok(()));
        assert_eq!(game.board().get(pos(2, 0)), Some(Color::White));
        assert_eq!(game.board().get(pos(0, 0)), None);
        assert_eq!(game.phase(), Phase::Placement);
        assert_eq!(game.current(), Color::Black);
    }

    #[test]
    fn five_in_a_row_on_placement_wins_without_a_rotation() {
        let mut board = Board::new();
        place_all(&mut board, Color::White, &[(0, 0), (1, 0), (2, 0), (3, 0)]);
        let mut game = Game::from_parts(board, Color::White, Phase::Placement);

        game.place(pos(4, 0)).unwrap();

        assert_eq!(game.phase(), Phase::Over(Outcome::Win(Color::White)));
    }

    #[test]
    fn nothing_is_allowed_once_the_game_is_over() {
        let mut game = Game::from_parts(Board::new(), Color::White, Phase::Over(Outcome::Draw));
        let before = game.clone();
        assert_eq!(game.place(pos(0, 0)), Err(MoveError::GameOver));
        assert_eq!(
            game.rotate(Quadrant::TopLeft, Direction::Clockwise),
            Err(MoveError::GameOver)
        );
        assert_eq!(game, before);
    }

    #[test]
    fn rotation_aligning_the_mover_wins() {
        let board = white_one_rotation_from_a_line();
        let mut game = Game::from_parts(board, Color::White, Phase::Rotation);

        game.rotate(Quadrant::TopLeft, Direction::Clockwise)
            .unwrap();

        assert_eq!(game.phase(), Phase::Over(Outcome::Win(Color::White)));
    }

    #[test]
    fn rotation_aligning_only_the_opponent_makes_the_opponent_win() {
        let board = white_one_rotation_from_a_line();
        let mut game = Game::from_parts(board, Color::Black, Phase::Rotation);

        game.rotate(Quadrant::TopLeft, Direction::Clockwise)
            .unwrap();

        assert_eq!(game.phase(), Phase::Over(Outcome::Win(Color::White)));
    }

    #[test]
    fn rotation_aligning_both_colors_is_a_draw() {
        let mut board = white_one_rotation_from_a_line();
        place_all(
            &mut board,
            Color::Black,
            &[(1, 0), (1, 1), (1, 2), (3, 1), (4, 1)],
        );
        assert!(!board.has_winning_line(Color::White));
        assert!(!board.has_winning_line(Color::Black));
        let mut game = Game::from_parts(board, Color::White, Phase::Rotation);

        game.rotate(Quadrant::TopLeft, Direction::Clockwise)
            .unwrap();

        assert_eq!(game.phase(), Phase::Over(Outcome::Draw));
    }

    #[test]
    fn rotation_without_a_line_continues_the_game() {
        let mut game = Game::new();
        game.place(pos(1, 1)).unwrap();
        game.rotate(Quadrant::BottomRight, Direction::CounterClockwise)
            .unwrap();
        assert_eq!(game.phase(), Phase::Placement);
    }

    #[test]
    fn last_marble_completing_a_line_is_a_win_not_a_draw() {
        // Top row is W W W B B B in the pattern; make it W W W W _ B.
        let mut board = Board::new();
        for position in Position::all() {
            let color = match (position.col(), position.row()) {
                (4, 0) => continue,
                (3, 0) => Color::White,
                _ => block_color(position),
            };
            board.place(position, color).unwrap();
        }
        assert!(!board.has_winning_line(Color::White));
        assert!(!board.has_winning_line(Color::Black));
        let mut game = Game::from_parts(board, Color::White, Phase::Placement);

        game.place(pos(4, 0)).unwrap();

        assert!(game.board().is_full());
        assert_eq!(game.phase(), Phase::Over(Outcome::Win(Color::White)));
    }

    #[test]
    fn full_board_without_a_line_is_a_draw_only_after_the_rotation() {
        let board = blocks_board_without(&[(5, 5)]);
        let mut game = Game::from_parts(board, Color::Black, Phase::Placement);

        game.place(pos(5, 5)).unwrap();
        assert!(game.board().is_full());
        assert_eq!(game.phase(), Phase::Rotation);

        game.rotate(Quadrant::TopLeft, Direction::CounterClockwise)
            .unwrap();
        assert_eq!(game.phase(), Phase::Over(Outcome::Draw));
    }

    #[test]
    fn full_board_where_the_last_rotation_makes_a_line_is_a_win() {
        // Turning the top-left quadrant clockwise gives White columns 1 to 5 of the third row.
        let board = blocks_board_without(&[]);
        let mut game = Game::from_parts(board, Color::Black, Phase::Rotation);

        game.rotate(Quadrant::TopLeft, Direction::Clockwise)
            .unwrap();

        assert_eq!(game.phase(), Phase::Over(Outcome::Win(Color::White)));
    }

    #[test]
    fn a_game_in_progress_has_no_winning_cells() {
        let mut game = Game::new();
        assert_eq!(game.winning_cells(), []);
        game.place(pos(1, 1)).unwrap();
        assert_eq!(game.winning_cells(), []);
    }

    #[test]
    fn a_won_game_reports_the_winning_cells() {
        let mut board = Board::new();
        place_all(&mut board, Color::White, &[(0, 0), (1, 0), (2, 0), (3, 0)]);
        place_all(&mut board, Color::Black, &[(0, 5), (1, 5), (2, 5), (0, 4)]);
        let mut game = Game::from_parts(board, Color::White, Phase::Placement);

        game.place(pos(4, 0)).unwrap();

        let expected: Vec<Position> = (0..5).map(|col| pos(col, 0)).collect();
        assert_eq!(game.winning_cells(), expected);
    }

    #[test]
    fn a_draw_by_double_line_reports_the_cells_of_both_colors() {
        let mut board = white_one_rotation_from_a_line();
        place_all(
            &mut board,
            Color::Black,
            &[(1, 0), (1, 1), (1, 2), (3, 1), (4, 1)],
        );
        let mut game = Game::from_parts(board, Color::White, Phase::Rotation);

        game.rotate(Quadrant::TopLeft, Direction::Clockwise)
            .unwrap();

        // White holds the top row, Black the one below, both in columns 0 to 4.
        let expected: Vec<Position> = (0..2)
            .flat_map(|row| (0..5).map(move |col| pos(col, row)))
            .collect();
        assert_eq!(game.phase(), Phase::Over(Outcome::Draw));
        assert_eq!(game.winning_cells(), expected);
    }

    #[test]
    fn a_draw_by_full_board_has_no_winning_cells() {
        let board = blocks_board_without(&[(5, 5)]);
        let mut game = Game::from_parts(board, Color::Black, Phase::Placement);
        game.place(pos(5, 5)).unwrap();
        game.rotate(Quadrant::TopLeft, Direction::CounterClockwise)
            .unwrap();

        assert_eq!(game.phase(), Phase::Over(Outcome::Draw));
        assert_eq!(game.winning_cells(), []);
    }
}
