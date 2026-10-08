use pentago_core::{BOARD_SIZE, Color, Outcome, Phase, Position, QUADRANT_SIZE, Quadrant};
use ratatui::Frame;
use ratatui::layout::{Constraint, Layout};
use ratatui::style::{self, Style, Stylize};
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;

use crate::app::App;

const WIDTH: u16 = 70;
const HEIGHT: u16 = 15;
/// Rows of the board once drawn: two quadrants high, each three cells and two edges.
const BOARD_HEIGHT: u16 = 10;

// 256-color palette indexes, which every common terminal supports.
const BONE: style::Color = style::Color::Indexed(230);
const AMBER: style::Color = style::Color::Indexed(208);
const HOLE: style::Color = style::Color::Indexed(240);
const EDGE: style::Color = style::Color::Indexed(245);

const TOP_EDGE: &str = "╭─────────╮";
const BOTTOM_EDGE: &str = "╰─────────╯";

/// Draws the whole screen from the current state.
pub fn draw(frame: &mut Frame, app: &App) {
    let area = frame
        .area()
        .centered(Constraint::Length(WIDTH), Constraint::Length(HEIGHT));
    let [status, _, board, _, message, help] = Layout::vertical([
        Constraint::Length(1),
        Constraint::Length(1),
        Constraint::Length(BOARD_HEIGHT),
        Constraint::Length(1),
        Constraint::Length(1),
        Constraint::Length(1),
    ])
    .areas(area);

    frame.render_widget(status_line(app).bold().centered(), status);
    frame.render_widget(Paragraph::new(board_lines(app)).centered(), board);
    frame.render_widget(
        Line::from(app.message().unwrap_or_default())
            .red()
            .centered(),
        message,
    );
    frame.render_widget(Line::from(help_text(app)).dim().centered(), help);
}

fn status_line(app: &App) -> Line<'static> {
    let game = app.game();
    let (color, text) = match game.phase() {
        Phase::Placement => (game.current(), "to place"),
        Phase::Rotation => (game.current(), "to rotate"),
        Phase::Over(Outcome::Win(winner)) => (winner, "wins!"),
        Phase::Over(Outcome::Draw) => return Line::from("Draw"),
    };
    Line::from(vec![
        Span::styled("● ", marble_color(color)),
        Span::raw(format!("{} {text}", name(color))),
    ])
}

fn help_text(app: &App) -> &'static str {
    match app.game().phase() {
        Phase::Placement => "arrows: move   enter: place   q: quit",
        Phase::Rotation => {
            "arrows: choose quadrant   r: clockwise   l: counter-clockwise   q: quit"
        }
        Phase::Over(_) => "n: new game   q: quit",
    }
}

/// The two sides are shown as bone and amber marbles.
fn name(color: Color) -> &'static str {
    match color {
        Color::White => "Bone",
        Color::Black => "Amber",
    }
}

fn marble_color(color: Color) -> style::Color {
    match color {
        Color::White => BONE,
        Color::Black => AMBER,
    }
}

/// Four separate tiles, two per band, with a one-column gap between them.
fn board_lines(app: &App) -> Vec<Line<'static>> {
    let positions: Vec<Position> = Position::all().collect();
    let row_length = usize::from(BOARD_SIZE);
    let tile = usize::from(QUADRANT_SIZE);
    let mut lines = Vec::new();

    for band in positions.chunks(row_length * tile) {
        lines.push(edge_line(app, band, TOP_EDGE));
        for row in band.chunks(row_length) {
            let mut spans = Vec::new();
            for cells in row.chunks(tile) {
                if cells[0].col() > 0 {
                    spans.push(Span::raw(" "));
                }
                let edge = edge_style(app, cells[0].quadrant());
                spans.push(Span::styled("│", edge));
                for &position in cells {
                    spans.extend(cell(app, position));
                }
                spans.push(Span::styled("│", edge));
            }
            lines.push(Line::from(spans));
        }
        lines.push(edge_line(app, band, BOTTOM_EDGE));
    }
    lines
}

/// The top or bottom edge of the two tiles in `band`.
fn edge_line(app: &App, band: &[Position], edge: &'static str) -> Line<'static> {
    let tile = usize::from(QUADRANT_SIZE);
    let left = edge_style(app, band[0].quadrant());
    let right = edge_style(app, band[tile].quadrant());
    Line::from(vec![
        Span::styled(edge, left),
        Span::raw(" "),
        Span::styled(edge, right),
    ])
}

/// The quadrant about to be rotated gets an amber outline.
fn edge_style(app: &App, quadrant: Quadrant) -> Style {
    if app.game().phase() == Phase::Rotation && quadrant == app.quadrant() {
        Style::new().fg(AMBER).bold()
    } else {
        Style::new().fg(EDGE)
    }
}

/// One cell, three columns wide; the cursor is shown as brackets around it.
fn cell(app: &App, position: Position) -> [Span<'static>; 3] {
    let symbol = match app.game().board().get(position) {
        Some(color) => Span::styled("●", marble_color(color)),
        None => Span::styled("·", HOLE),
    };
    if app.game().phase() == Phase::Placement && position == app.cursor() {
        let bracket = Style::new().fg(AMBER).bold();
        [
            Span::styled("[", bracket),
            symbol,
            Span::styled("]", bracket),
        ]
    } else {
        [Span::raw(" "), symbol, Span::raw(" ")]
    }
}

#[cfg(test)]
mod tests {
    use ratatui::Terminal;
    use ratatui::backend::TestBackend;
    use ratatui::buffer::Buffer;
    use ratatui::crossterm::event::KeyCode;

    use super::*;

    fn render_buffer(app: &App) -> Buffer {
        let mut terminal = Terminal::new(TestBackend::new(WIDTH, HEIGHT)).unwrap();
        terminal.draw(|frame| draw(frame, app)).unwrap();
        terminal.backend().buffer().clone()
    }

    /// The screen as text, one string per row, without colors.
    fn render(app: &App) -> Vec<String> {
        let buffer = render_buffer(app);
        buffer
            .content()
            .chunks(usize::from(buffer.area.width))
            .map(|row| row.iter().map(|cell| cell.symbol()).collect())
            .collect()
    }

    /// The foreground color of every cell showing `symbol`, in reading order.
    fn colors_of(app: &App, symbol: &str) -> Vec<style::Color> {
        render_buffer(app)
            .content()
            .iter()
            .filter(|cell| cell.symbol() == symbol)
            .map(|cell| cell.fg)
            .collect()
    }

    fn count(screen: &[String], symbol: char) -> usize {
        screen
            .iter()
            .flat_map(|row| row.chars())
            .filter(|&c| c == symbol)
            .count()
    }

    fn press(app: &mut App, keys: &[KeyCode]) {
        for &key in keys {
            app.on_key(key);
        }
    }

    #[test]
    fn new_game_shows_four_empty_tiles_and_whose_turn_it_is() {
        let screen = render(&App::new());
        assert_eq!(screen[0].trim(), "● Bone to place");
        assert_eq!(screen[2].trim(), "╭─────────╮ ╭─────────╮");
        assert_eq!(screen[4].trim(), "│ ·  ·  · │ │ ·  ·  · │");
        assert_eq!(screen[6].trim(), "╰─────────╯ ╰─────────╯");
        assert_eq!(screen[7].trim(), "╭─────────╮ ╭─────────╮");
        assert_eq!(screen[11].trim(), "╰─────────╯ ╰─────────╯");
        assert_eq!(count(&screen, '·'), 36);
    }

    #[test]
    fn the_cursor_is_shown_as_brackets_while_placing() {
        let mut app = App::new();
        assert_eq!(render(&app)[3].trim(), "│[·] ·  · │ │ ·  ·  · │");
        app.on_key(KeyCode::Right);
        assert_eq!(render(&app)[3].trim(), "│ · [·] · │ │ ·  ·  · │");
    }

    #[test]
    fn a_placed_marble_is_drawn_where_it_was_placed() {
        let mut app = App::new();
        press(&mut app, &[KeyCode::Right, KeyCode::Enter]);
        let screen = render(&app);
        assert_eq!(screen[0].trim(), "● Bone to rotate");
        assert_eq!(screen[3].trim(), "│ ·  ●  · │ │ ·  ·  · │");
    }

    #[test]
    fn the_two_sides_have_different_marble_colors() {
        let mut app = App::new();
        // Bone at (1, 1), which stays put when its quadrant turns; Amber at (4, 1).
        press(&mut app, &[KeyCode::Right, KeyCode::Down, KeyCode::Enter]);
        app.on_key(KeyCode::Char('r'));
        press(&mut app, &[KeyCode::Right; 3]);
        app.on_key(KeyCode::Enter);

        // Status marble (still Amber's turn), then the two marbles on the board.
        assert_eq!(colors_of(&app, "●"), [AMBER, BONE, AMBER]);
        assert_eq!(render(&app)[4].trim(), "│ ·  ●  · │ │ ·  ●  · │");
    }

    #[test]
    fn only_the_selected_quadrant_is_outlined_while_rotating() {
        let mut app = App::new();
        assert!(colors_of(&app, "╭").iter().all(|&color| color == EDGE));

        press(&mut app, &[KeyCode::Enter, KeyCode::Right]);
        // Reading order: top-left, top-right, bottom-left, bottom-right.
        assert_eq!(colors_of(&app, "╭"), [EDGE, AMBER, EDGE, EDGE]);
    }

    #[test]
    fn cells_have_no_background_color() {
        let mut app = App::new();
        press(&mut app, &[KeyCode::Enter]);
        let buffer = render_buffer(&app);
        assert!(
            buffer
                .content()
                .iter()
                .all(|cell| cell.bg == style::Color::Reset)
        );
    }

    #[test]
    fn help_follows_the_phase() {
        let mut app = App::new();
        assert!(render(&app)[14].contains("enter: place"));
        app.on_key(KeyCode::Enter);
        assert!(render(&app)[14].contains("r: clockwise"));
    }

    #[test]
    fn a_refused_action_shows_its_reason() {
        let mut app = App::new();
        press(&mut app, &[KeyCode::Down, KeyCode::Right, KeyCode::Enter]);
        press(&mut app, &[KeyCode::Char('r'), KeyCode::Enter]);
        assert_eq!(render(&app)[13].trim(), "this cell is already occupied");
    }
}
