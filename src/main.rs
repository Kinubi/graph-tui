use clap::Parser;

mod app;
mod tui;
mod graph;
mod node_builder;
mod edge;

#[derive(Debug, Parser)]
#[command(name = "graph-tui", about = "Graph Tui for editing Graphs in the Terminal")]
struct Args {
    #[arg(short, long, value_name = "input_path")]
    template: Option<String>,
    #[arg(short, long, value_name = "output_path")]
    output: Option<String>,
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
    let result = app::write_graph_to_path(&args.output.unwrap(), &app.graph, &app.node_catalog);
    match result {
        Ok(()) => {
            return Ok(());
        }
        Err(error) => {
            println!("{}", error.to_string());
        }
    }

    Ok(())
}
