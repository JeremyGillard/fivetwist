mod app;
mod ui;

use app::App;

fn main() -> std::io::Result<()> {
    ratatui::run(|terminal| App::new().run(terminal))
}
