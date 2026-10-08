use std::io;

use pentago_core::{Direction, Game, Phase, Position, Quadrant};
use ratatui::DefaultTerminal;
use ratatui::crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};

use crate::ui;

/// Everything the screen is drawn from.
pub struct App {
    game: Game,
    /// Cell highlighted while placing.
    cursor: Position,
    /// Quadrant highlighted while rotating.
    quadrant: Quadrant,
    /// Why the last action was refused, if it was.
    message: Option<String>,
    quit: bool,
}

impl App {
    pub fn new() -> Self {
        App {
            game: Game::new(),
            cursor: Quadrant::TopLeft.origin(),
            quadrant: Quadrant::TopLeft,
            message: None,
            quit: false,
        }
    }

    pub fn game(&self) -> &Game {
        &self.game
    }

    pub fn cursor(&self) -> Position {
        self.cursor
    }

    pub fn quadrant(&self) -> Quadrant {
        self.quadrant
    }

    pub fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }

    /// Draws, waits for a key, updates; until the player quits.
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.quit {
            terminal.draw(|frame| ui::draw(frame, self))?;

            if let Event::Key(key) = event::read()?
                && key.kind == KeyEventKind::Press
            {
                // Raw mode disables the terminal's own Ctrl-C handling.
                if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c') {
                    self.quit = true;
                } else {
                    self.on_key(key.code);
                }
            }
        }
        Ok(())
    }

    /// What a key means depends on the phase of the game.
    pub fn on_key(&mut self, code: KeyCode) {
        self.message = None;
        match (self.game.phase(), code) {
            (_, KeyCode::Char('q') | KeyCode::Esc) => self.quit = true,

            (Phase::Placement, KeyCode::Enter | KeyCode::Char(' ')) => self.place(),
            (Phase::Placement, arrow) => self.move_cursor(arrow),

            (Phase::Rotation, KeyCode::Char('r')) => self.rotate(Direction::Clockwise),
            (Phase::Rotation, KeyCode::Char('l')) => self.rotate(Direction::CounterClockwise),
            (Phase::Rotation, arrow) => self.select_quadrant(arrow),

            (Phase::Over(_), KeyCode::Char('n')) => *self = App::new(),
            (Phase::Over(_), _) => {}
        }
    }

    fn place(&mut self) {
        match self.game.place(self.cursor) {
            Ok(()) => self.quadrant = self.cursor.quadrant(),
            Err(error) => self.message = Some(error.to_string()),
        }
    }

    fn rotate(&mut self, direction: Direction) {
        if let Err(error) = self.game.rotate(self.quadrant, direction) {
            self.message = Some(error.to_string());
        }
    }

    /// Moves the cursor one cell; it stays put at the edge of the board.
    fn move_cursor(&mut self, arrow: KeyCode) {
        let (dcol, drow) = match arrow {
            KeyCode::Left => (-1, 0),
            KeyCode::Right => (1, 0),
            KeyCode::Up => (0, -1),
            KeyCode::Down => (0, 1),
            _ => return,
        };
        if let Some(position) = self.cursor.offset(dcol, drow) {
            self.cursor = position;
        }
    }

    fn select_quadrant(&mut self, arrow: KeyCode) {
        use Quadrant::{BottomLeft, BottomRight, TopLeft, TopRight};

        self.quadrant = match (self.quadrant, arrow) {
            (TopRight, KeyCode::Left) | (BottomLeft, KeyCode::Up) => TopLeft,
            (TopLeft, KeyCode::Right) | (BottomRight, KeyCode::Up) => TopRight,
            (BottomRight, KeyCode::Left) | (TopLeft, KeyCode::Down) => BottomLeft,
            (BottomLeft, KeyCode::Right) | (TopRight, KeyCode::Down) => BottomRight,
            (unchanged, _) => unchanged,
        };
    }
}

#[cfg(test)]
mod tests {
    use pentago_core::{Color, Outcome};

    use super::*;

    fn pos(col: u8, row: u8) -> Position {
        Position::new(col, row).unwrap()
    }

    fn press(app: &mut App, keys: &[KeyCode]) {
        for &key in keys {
            app.on_key(key);
        }
    }

    /// Places at the cursor, then turns the empty bottom-right quadrant.
    fn play_turn(app: &mut App, target: Position) {
        app.cursor = target;
        app.on_key(KeyCode::Enter);
        app.quadrant = Quadrant::BottomRight;
        app.on_key(KeyCode::Char('r'));
    }

    /// White takes the top row, Black the bottom-left corner.
    fn app_won_by_white() -> App {
        let mut app = App::new();
        let white = [(0, 0), (1, 0), (2, 0), (3, 0)];
        let black = [(0, 5), (1, 5), (2, 5), (0, 4)];
        for ((white_col, white_row), (black_col, black_row)) in white.into_iter().zip(black) {
            play_turn(&mut app, pos(white_col, white_row));
            play_turn(&mut app, pos(black_col, black_row));
        }
        app.cursor = pos(4, 0);
        app.on_key(KeyCode::Enter);
        app
    }

    #[test]
    fn cursor_starts_in_the_top_left_corner() {
        assert_eq!(App::new().cursor(), pos(0, 0));
    }

    #[test]
    fn arrows_move_the_cursor() {
        let mut app = App::new();
        press(&mut app, &[KeyCode::Right, KeyCode::Right, KeyCode::Down]);
        assert_eq!(app.cursor(), pos(2, 1));
        press(&mut app, &[KeyCode::Left, KeyCode::Up]);
        assert_eq!(app.cursor(), pos(1, 0));
    }

    #[test]
    fn cursor_stops_at_the_edges() {
        let mut app = App::new();
        press(&mut app, &[KeyCode::Left, KeyCode::Up]);
        assert_eq!(app.cursor(), pos(0, 0));
        press(&mut app, &[KeyCode::Right; 9]);
        press(&mut app, &[KeyCode::Down; 9]);
        assert_eq!(app.cursor(), pos(5, 5));
    }

    #[test]
    fn enter_places_a_marble_at_the_cursor() {
        let mut app = App::new();
        press(&mut app, &[KeyCode::Right, KeyCode::Enter]);
        assert_eq!(app.game().board().get(pos(1, 0)), Some(Color::White));
        assert_eq!(app.game().phase(), Phase::Rotation);
    }

    #[test]
    fn space_places_a_marble_too() {
        let mut app = App::new();
        app.on_key(KeyCode::Char(' '));
        assert_eq!(app.game().board().get(pos(0, 0)), Some(Color::White));
    }

    #[test]
    fn placing_selects_the_quadrant_under_the_cursor() {
        let mut app = App::new();
        press(&mut app, &[KeyCode::Right; 4]);
        press(&mut app, &[KeyCode::Down; 4]);
        app.on_key(KeyCode::Enter);
        assert_eq!(app.quadrant(), Quadrant::BottomRight);
    }

    #[test]
    fn arrows_select_a_quadrant_while_rotating() {
        let mut app = App::new();
        app.on_key(KeyCode::Enter);
        assert_eq!(app.quadrant(), Quadrant::TopLeft);

        app.on_key(KeyCode::Right);
        assert_eq!(app.quadrant(), Quadrant::TopRight);
        app.on_key(KeyCode::Down);
        assert_eq!(app.quadrant(), Quadrant::BottomRight);
        app.on_key(KeyCode::Left);
        assert_eq!(app.quadrant(), Quadrant::BottomLeft);
        app.on_key(KeyCode::Up);
        assert_eq!(app.quadrant(), Quadrant::TopLeft);
    }

    #[test]
    fn quadrant_selection_stops_at_the_edges() {
        let mut app = App::new();
        app.on_key(KeyCode::Enter);
        press(&mut app, &[KeyCode::Left, KeyCode::Up]);
        assert_eq!(app.quadrant(), Quadrant::TopLeft);
    }

    #[test]
    fn arrows_do_not_move_the_cursor_while_rotating() {
        let mut app = App::new();
        press(&mut app, &[KeyCode::Enter, KeyCode::Right, KeyCode::Down]);
        assert_eq!(app.cursor(), pos(0, 0));
    }

    #[test]
    fn r_rotates_the_selected_quadrant_clockwise_and_passes_the_turn() {
        let mut app = App::new();
        press(&mut app, &[KeyCode::Enter, KeyCode::Char('r')]);
        assert_eq!(app.game().board().get(pos(2, 0)), Some(Color::White));
        assert_eq!(app.game().phase(), Phase::Placement);
        assert_eq!(app.game().current(), Color::Black);
    }

    #[test]
    fn l_rotates_the_selected_quadrant_counter_clockwise() {
        let mut app = App::new();
        press(&mut app, &[KeyCode::Enter, KeyCode::Char('l')]);
        assert_eq!(app.game().board().get(pos(0, 2)), Some(Color::White));
    }

    #[test]
    fn rotation_keys_do_nothing_while_placing() {
        let mut app = App::new();
        press(&mut app, &[KeyCode::Char('r'), KeyCode::Char('l')]);
        assert_eq!(app.game(), &Game::new());
        assert_eq!(app.message(), None);
    }

    #[test]
    fn placing_on_an_occupied_cell_shows_a_message() {
        let mut app = App::new();
        press(&mut app, &[KeyCode::Down, KeyCode::Right, KeyCode::Enter]);
        press(&mut app, &[KeyCode::Char('r'), KeyCode::Enter]);
        assert_eq!(app.message(), Some("this cell is already occupied"));
        assert_eq!(app.game().phase(), Phase::Placement);
    }

    #[test]
    fn the_next_key_clears_the_message() {
        let mut app = App::new();
        press(&mut app, &[KeyCode::Down, KeyCode::Right, KeyCode::Enter]);
        press(
            &mut app,
            &[KeyCode::Char('r'), KeyCode::Enter, KeyCode::Left],
        );
        assert_eq!(app.message(), None);
    }

    #[test]
    fn q_and_escape_quit_in_any_phase() {
        let mut app = App::new();
        app.on_key(KeyCode::Char('q'));
        assert!(app.quit);

        let mut app = App::new();
        press(&mut app, &[KeyCode::Enter, KeyCode::Esc]);
        assert!(app.quit);

        let mut app = app_won_by_white();
        app.on_key(KeyCode::Char('q'));
        assert!(app.quit);
    }

    #[test]
    fn a_full_game_can_be_played_with_keys() {
        let app = app_won_by_white();
        assert_eq!(app.game().phase(), Phase::Over(Outcome::Win(Color::White)));
    }

    #[test]
    fn game_keys_do_nothing_once_the_game_is_over() {
        let mut app = app_won_by_white();
        let finished = app.game().clone();
        press(
            &mut app,
            &[KeyCode::Right, KeyCode::Enter, KeyCode::Char('r')],
        );
        assert_eq!(app.game(), &finished);
        assert_eq!(app.message(), None);
    }

    #[test]
    fn n_starts_a_new_game_once_the_game_is_over() {
        let mut app = app_won_by_white();
        app.on_key(KeyCode::Char('n'));
        assert_eq!(app.game(), &Game::new());
        assert_eq!(app.cursor(), pos(0, 0));
    }

    #[test]
    fn n_does_not_restart_a_game_in_progress() {
        let mut app = App::new();
        press(&mut app, &[KeyCode::Enter, KeyCode::Char('n')]);
        assert_eq!(app.game().phase(), Phase::Rotation);
    }
}
