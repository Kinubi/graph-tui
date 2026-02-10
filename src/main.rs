use clap::Parser;

mod app;
mod tui;
mod graph;
mod node_builder;
mod edge;

#[derive(Debug, Parser)]
#[command(name = "graph-tui", about = "Graph Tui for editing Graphs in the Terminal")]
struct Args {
    #[arg(short, long, value_name = "PATH")]
    template: Option<String>,
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let args = Args::parse();
    let template_path = args.template;
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
