use ratatui::crossterm::event::EnableMouseCapture;
use ratatui::crossterm::execute;
use ratatui::crossterm::terminal::{ enable_raw_mode, EnterAlternateScreen };
use std::io;

mod app;
mod tui;
mod graph;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    ratatui::run(|terminal| tui::Tui::new().run(terminal, &mut app::App::new()))?;
    Ok(())
}
