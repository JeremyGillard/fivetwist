use crate::color::Color;
use crate::direction::Direction;
use crate::error::CellOccupied;
use crate::position::Position;
use crate::quadrant::Quadrant;
use crate::{BOARD_SIZE, QUADRANT_SIZE, WIN_LENGTH};

const SIZE: usize = BOARD_SIZE as usize;
const QUADRANT: usize = QUADRANT_SIZE as usize;
const LINE: i8 = WIN_LENGTH as i8;

/// The four ways a line can run: right, down, down-right, up-right.
const STEPS: [(i8, i8); 4] = [(1, 0), (0, 1), (1, 1), (1, -1)];

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

    /// Places a marble, or leaves the board untouched if the cell is occupied.
    pub fn place(&mut self, pos: Position, color: Color) -> Result<(), CellOccupied> {
        let cell = &mut self.cells[usize::from(pos.row())][usize::from(pos.col())];
        if cell.is_some() {
            return Err(CellOccupied);
        }
        *cell = Some(color);
        Ok(())
    }

    pub fn is_full(&self) -> bool {
        self.cells.iter().flatten().all(|cell| cell.is_some())
    }

    /// Whether `color` has at least [`WIN_LENGTH`] marbles in a row, in any direction.
    ///
    /// [`WIN_LENGTH`]: crate::WIN_LENGTH
    pub fn has_winning_line(&self, color: Color) -> bool {
        self.winning_lines(color).next().is_some()
    }

    /// Every cell that is part of a winning line of `color`, in board order.
    ///
    /// Empty if `color` has no winning line. Six in a row gives six cells, and two crossing
    /// lines give the cells of both.
    pub fn winning_cells(&self, color: Color) -> Vec<Position> {
        let lines: Vec<Vec<Position>> = self.winning_lines(color).collect();
        Position::all()
            .filter(|position| lines.iter().any(|line| line.contains(position)))
            .collect()
    }

    /// Each run of `WIN_LENGTH` cells that all hold `color`.
    fn winning_lines(&self, color: Color) -> impl Iterator<Item = Vec<Position>> + '_ {
        Position::all()
            .flat_map(|start| STEPS.iter().filter_map(move |&step| line_from(start, step)))
            .filter(move |line| {
                line.iter()
                    .all(|&position| self.get(position) == Some(color))
            })
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

/// The `WIN_LENGTH` cells from `start` along `step`, or `None` if they leave the board.
fn line_from(start: Position, (dcol, drow): (i8, i8)) -> Option<Vec<Position>> {
    (0..LINE)
        .map(|i| start.offset(dcol * i, drow * i))
        .collect()
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
        assert_eq!(board.place(pos(2, 4), Color::White), Ok(()));
        assert_eq!(board.get(pos(2, 4)), Some(Color::White));
    }

    #[test]
    fn place_leaves_other_cells_empty() {
        let mut board = Board::new();
        board.place(pos(2, 4), Color::White).unwrap();
        assert_eq!(board.get(pos(4, 2)), None);
    }

    #[test]
    fn place_on_occupied_cell_is_rejected() {
        let mut board = Board::new();
        board.place(pos(1, 1), Color::White).unwrap();
        assert_eq!(board.place(pos(1, 1), Color::Black), Err(CellOccupied));
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
                board.place(pos(col, row), Color::Black).unwrap();
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
                    board.place(pos(col, row), Color::Black).unwrap();
                }
            }
        }
        assert!(!board.is_full());
    }

    /// A board with a single white marble.
    fn board_with_marble_at(col: u8, row: u8) -> Board {
        let mut board = Board::new();
        board.place(pos(col, row), Color::White).unwrap();
        board
    }

    /// An asymmetric position touching every quadrant.
    fn scattered_board() -> Board {
        let mut board = Board::new();
        board.place(pos(0, 0), Color::White).unwrap();
        board.place(pos(1, 0), Color::Black).unwrap();
        board.place(pos(2, 1), Color::White).unwrap();
        board.place(pos(4, 2), Color::Black).unwrap();
        board.place(pos(0, 5), Color::White).unwrap();
        board.place(pos(3, 3), Color::Black).unwrap();
        board.place(pos(5, 4), Color::White).unwrap();
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
        for quadrant in Quadrant::ALL {
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

    fn place_many(board: &mut Board, color: Color, cells: &[(u8, u8)]) {
        for &(col, row) in cells {
            board.place(pos(col, row), color).unwrap();
        }
    }

    fn board_with(color: Color, cells: &[(u8, u8)]) -> Board {
        let mut board = Board::new();
        place_many(&mut board, color, cells);
        board
    }

    #[test]
    fn empty_board_has_no_five_in_a_row() {
        let board = Board::new();
        assert!(!board.has_winning_line(Color::White));
        assert!(!board.has_winning_line(Color::Black));
    }

    #[test]
    fn five_in_a_row_horizontally() {
        let board = board_with(Color::White, &[(0, 2), (1, 2), (2, 2), (3, 2), (4, 2)]);
        assert!(board.has_winning_line(Color::White));
    }

    #[test]
    fn four_in_a_row_is_not_enough() {
        let board = board_with(Color::White, &[(0, 2), (1, 2), (2, 2), (3, 2)]);
        assert!(!board.has_winning_line(Color::White));
    }

    #[test]
    fn a_line_only_counts_for_its_own_color() {
        let board = board_with(Color::White, &[(0, 2), (1, 2), (2, 2), (3, 2), (4, 2)]);
        assert!(!board.has_winning_line(Color::Black));
    }

    #[test]
    fn a_line_broken_by_a_gap_does_not_count() {
        let board = board_with(Color::White, &[(0, 2), (1, 2), (2, 2), (3, 2), (5, 2)]);
        assert!(!board.has_winning_line(Color::White));
    }

    #[test]
    fn a_line_broken_by_the_opponent_does_not_count() {
        let mut board = board_with(Color::White, &[(0, 2), (1, 2), (3, 2), (4, 2), (5, 2)]);
        board.place(pos(2, 2), Color::Black).unwrap();
        assert!(!board.has_winning_line(Color::White));
        assert!(!board.has_winning_line(Color::Black));
    }

    #[test]
    fn five_in_a_row_vertically() {
        let board = board_with(Color::Black, &[(3, 1), (3, 2), (3, 3), (3, 4), (3, 5)]);
        assert!(board.has_winning_line(Color::Black));
    }

    #[test]
    fn five_in_a_row_on_the_down_right_diagonal() {
        let board = board_with(Color::White, &[(0, 0), (1, 1), (2, 2), (3, 3), (4, 4)]);
        assert!(board.has_winning_line(Color::White));
    }

    #[test]
    fn five_in_a_row_on_an_off_centre_diagonal() {
        let board = board_with(Color::White, &[(1, 0), (2, 1), (3, 2), (4, 3), (5, 4)]);
        assert!(board.has_winning_line(Color::White));
    }

    #[test]
    fn five_in_a_row_on_the_up_right_diagonal() {
        let board = board_with(Color::White, &[(0, 5), (1, 4), (2, 3), (3, 2), (4, 1)]);
        assert!(board.has_winning_line(Color::White));
    }

    #[test]
    fn six_in_a_row_counts() {
        let board = board_with(
            Color::White,
            &[(0, 0), (1, 0), (2, 0), (3, 0), (4, 0), (5, 0)],
        );
        assert!(board.has_winning_line(Color::White));
    }

    #[test]
    fn a_line_does_not_wrap_around_the_edge() {
        let board = board_with(Color::White, &[(4, 0), (5, 0), (0, 1), (1, 1), (2, 1)]);
        assert!(!board.has_winning_line(Color::White));
    }

    #[test]
    fn five_in_a_row_touching_the_bottom_right_corner() {
        let board = board_with(Color::White, &[(1, 5), (2, 5), (3, 5), (4, 5), (5, 5)]);
        assert!(board.has_winning_line(Color::White));
    }

    #[test]
    fn full_board_without_a_line_has_no_five_in_a_row() {
        // Blocks two rows high and three columns wide, alternating: no run longer than four.
        let mut board = Board::new();
        for position in Position::all() {
            let stripe = (position.row() / 2 + position.col() / 3) % 2;
            let color = if stripe == 0 {
                Color::White
            } else {
                Color::Black
            };
            board.place(position, color).unwrap();
        }
        assert!(board.is_full());
        assert!(!board.has_winning_line(Color::White));
        assert!(!board.has_winning_line(Color::Black));
    }

    fn positions(cells: &[(u8, u8)]) -> Vec<Position> {
        cells.iter().map(|&(col, row)| pos(col, row)).collect()
    }

    #[test]
    fn no_winning_cells_without_a_line() {
        let board = board_with(Color::White, &[(0, 2), (1, 2), (2, 2), (3, 2)]);
        assert_eq!(board.winning_cells(Color::White), []);
        assert_eq!(Board::new().winning_cells(Color::White), []);
    }

    #[test]
    fn winning_cells_are_the_five_in_a_row() {
        // The extra marbles around the line are not part of it.
        let mut board = board_with(Color::White, &[(1, 2), (2, 2), (3, 2), (4, 2), (5, 2)]);
        place_many(&mut board, Color::White, &[(0, 0), (3, 4)]);
        place_many(&mut board, Color::Black, &[(0, 2), (3, 3)]);

        assert_eq!(
            board.winning_cells(Color::White),
            positions(&[(1, 2), (2, 2), (3, 2), (4, 2), (5, 2)])
        );
        assert_eq!(board.winning_cells(Color::Black), []);
    }

    #[test]
    fn winning_cells_follow_a_diagonal() {
        let board = board_with(Color::Black, &[(0, 5), (1, 4), (2, 3), (3, 2), (4, 1)]);
        // Board order is row by row from the top.
        assert_eq!(
            board.winning_cells(Color::Black),
            positions(&[(4, 1), (3, 2), (2, 3), (1, 4), (0, 5)])
        );
    }

    #[test]
    fn six_in_a_row_gives_six_winning_cells() {
        let cells = [(0, 0), (1, 0), (2, 0), (3, 0), (4, 0), (5, 0)];
        let board = board_with(Color::White, &cells);
        assert_eq!(board.winning_cells(Color::White), positions(&cells));
    }

    #[test]
    fn crossing_lines_give_the_cells_of_both_once() {
        let mut board = board_with(Color::White, &[(0, 2), (1, 2), (2, 2), (3, 2), (4, 2)]);
        place_many(&mut board, Color::White, &[(2, 0), (2, 1), (2, 3), (2, 4)]);

        assert_eq!(
            board.winning_cells(Color::White),
            positions(&[
                (2, 0),
                (2, 1),
                (0, 2),
                (1, 2),
                (2, 2),
                (3, 2),
                (4, 2),
                (2, 3),
                (2, 4),
            ])
        );
    }
}
