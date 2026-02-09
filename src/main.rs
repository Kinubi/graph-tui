mod app;
mod tui;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    ratatui::run(|terminal| tui::Tui::new().run(terminal, &mut app::App::new()))?;
    Ok(())
}
