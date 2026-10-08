use pentago_core::{Color, Direction, Game, MoveError, Outcome, Phase, Position, Quadrant};

fn pos(col: u8, row: u8) -> Position {
    Position::new(col, row).unwrap()
}

/// Plays one full turn: a placement, then a rotation of the bottom-right quadrant.
fn play_turn(game: &mut Game, color: Color, (col, row): (u8, u8)) {
    assert_eq!(game.current(), color);
    game.place(pos(col, row)).unwrap();
    game.rotate(Quadrant::BottomRight, Direction::Clockwise)
        .unwrap();
}

/// White fills the top row while Black plays the bottom-left corner.
/// The bottom-right quadrant stays empty, so rotating it changes nothing.
#[test]
fn white_wins_a_scripted_game() {
    let white = [(0, 0), (1, 0), (2, 0), (3, 0)];
    let black = [(0, 5), (1, 5), (2, 5), (0, 4)];
    let mut game = Game::new();

    for (white_move, black_move) in white.into_iter().zip(black) {
        play_turn(&mut game, Color::White, white_move);
        play_turn(&mut game, Color::Black, black_move);
    }
    assert_eq!(game.phase(), Phase::Placement);

    game.place(pos(4, 0)).unwrap();

    assert_eq!(game.phase(), Phase::Over(Outcome::Win(Color::White)));
    assert_eq!(game.place(pos(5, 0)), Err(MoveError::GameOver));
}
