use std::env;

mod app;
mod tui;
mod graph;
mod node_builder;
mod edge;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let template_path = parse_template_path(env::args().skip(1));
    let mut app = if let Some(path) = template_path {
        match app::load_node_catalog_from_path(&path) {
            Ok(catalog) => app::App::new_with_catalog(catalog),
            Err(_) => app::App::new(),
        }
    } else {
        app::App::new()
    };
    ratatui::run(|terminal| tui::Tui::new().run(terminal, &mut app))?;
    Ok(())
}

fn parse_template_path<I>(mut args: I) -> Option<String> where I: Iterator<Item = String> {
    while let Some(arg) = args.next() {
        if arg == "--template" || arg == "-t" {
            return args.next();
        }
        if !arg.starts_with('-') {
            return Some(arg);
        }
    }
    None
}
