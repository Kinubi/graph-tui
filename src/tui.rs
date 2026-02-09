use std::io;

use crossterm::event;
use crossterm::event::{ Event, KeyEventKind };
use ratatui::{
    buffer::Buffer,
    layout::{ Constraint, Direction, Layout, Rect },
    style::Stylize,
    symbols::border,
    text::{ Line, Text },
    widgets::{ Block, Clear, Paragraph, Widget },
    DefaultTerminal,
    Frame,
};

use crate::app::{ App, CurrentScreen, CurrentlyEditing, EdgeEditorMode, InOut, NodeEditorMode };

pub struct Tui;

impl Tui {
    pub fn new() -> Self {
        Self
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal, app: &mut App) -> io::Result<()> {
        while !app.should_exit() {
            terminal.draw(|frame| Self::draw(app, frame))?;
            self.handle_events(app)?;
            //app.on_tick();
        }
        Ok(())
    }

    fn draw(app: &App, frame: &mut Frame) {
        frame.render_widget(app, frame.area());
    }

    fn handle_events(&mut self, app: &mut App) -> io::Result<()> {
        if event::poll(std::time::Duration::from_millis(16))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    app.on_key(key);
                }
            }
        }
        Ok(())
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        match self.current_screen {
            CurrentScreen::Main => render_main(self, area, buf),
            CurrentScreen::Graph => render_graph(self, area, buf),
            CurrentScreen::GraphEditor => render_graph_editor(self, area, buf),
            CurrentScreen::NodeEditor => render_node_editor(self, area, buf),
            CurrentScreen::EdgeEditor => render_edge_editor(self, area, buf),
            CurrentScreen::Exiting => render_exiting(self, area, buf),
            _ => {}
        }
    }
}

fn render_main(app: &App, area: Rect, buf: &mut Buffer) {
    let title = Line::from(" Tui Graph Editor ".bold());
    let instructions = Line::from(
        vec![" Graph Editor ".into(), "<G>".blue().bold(), " Quit ".into(), "<Q> ".blue().bold()]
    );
    let block = Block::bordered()
        .title(title.centered())
        .title_bottom(instructions.centered())
        .border_set(border::THICK);

    let welcome_text = Text::from(
        "Welcome to the Tui Graph Editor! Press 'G' to start editing your graph."
    );

    Paragraph::new(welcome_text).centered().block(block).render(area, buf);
}

fn render_graph(app: &App, area: Rect, buf: &mut Buffer) {
    let title = Line::from(" Tui Graph Editor: Graph Overview ".bold());
    let instructions = Line::from(
        vec![" Edit Graph ".into(), "<E>".blue().bold(), " Quit ".into(), "<Q> ".blue().bold()]
    );
    let block = Block::bordered()
        .title(title.centered())
        .title_bottom(instructions.centered())
        .border_set(border::THICK);

    let graph_text = Text::from(build_graph_lines(app));

    Paragraph::new(graph_text).block(block).render(area, buf);
}

fn render_graph_editor(app: &App, area: Rect, buf: &mut Buffer) {
    let title = Line::from(" Tui Graph Editor: Graph ".bold());
    let instructions = Line::from(
        vec![
            " Add Node ".into(),
            "<N>".blue().bold(),
            " Add Edge ".into(),
            "<E>".blue().bold(),
            " Back ".into(),
            "<Q> ".blue().bold()
        ]
    );
    let block = Block::bordered()
        .title(title.centered())
        .title_bottom(instructions.centered())
        .border_set(border::THICK);

    let graph_text = Text::from(build_graph_lines(app));
    Paragraph::new(graph_text).block(block).render(area, buf);
}

fn render_node_editor(app: &App, area: Rect, buf: &mut Buffer) {
    render_graph_editor(app, area, buf);

    let title = Line::from(" Add Node ".bold());
    let instructions = Line::from(
        vec![
            " Type label ".into(),
            "<A..Z>".blue().bold(),
            " Save ".into(),
            "<Enter>".blue().bold(),
            " Cancel ".into(),
            "<Esc> ".blue().bold()
        ]
    );
    let block = Block::bordered()
        .title(title.centered())
        .title_bottom(instructions.centered())
        .border_set(border::THICK);

    let label_active = match &app.currently_editing {
        Some(CurrentlyEditing::Node(NodeEditorMode::Label)) => true,
        Some(CurrentlyEditing::Edge(_)) => false,
        None => false,
    };

    let label_prefix = if label_active { "Label:".yellow().underlined() } else { "Label:".into() };
    let label_value = app.label.clone().yellow();

    let lines = vec![
        Line::from(""),
        Line::from(vec![label_prefix, " ".into(), label_value]),
        Line::from(""),
        Line::from("Enter to save, Esc to cancel, Q to go back.")
    ];

    let body = Text::from(lines);
    let popup_area = centered_rect(60, 40, area);
    Clear.render(popup_area, buf);
    Paragraph::new(body).block(block).render(popup_area, buf);
}

fn render_edge_editor(app: &App, area: Rect, buf: &mut Buffer) {
    render_graph_editor(app, area, buf);

    let title = Line::from(" Add Edge ".bold());
    let instructions = Line::from(
        vec![
            " Type label ".into(),
            "<A..Z>".blue().bold(),
            " Next ".into(),
            "<Enter>".blue().bold(),
            " Cancel ".into(),
            "<Esc> ".blue().bold()
        ]
    );
    let block = Block::bordered()
        .title(title.centered())
        .title_bottom(instructions.centered())
        .border_set(border::THICK);

    let (label_active, from_active, to_active) = match &app.currently_editing {
        Some(CurrentlyEditing::Edge(EdgeEditorMode::Label)) => (true, false, false),
        Some(CurrentlyEditing::Edge(EdgeEditorMode::InOuts(InOut::From))) => {
            (false, true, false)
        }
        Some(CurrentlyEditing::Edge(EdgeEditorMode::InOuts(InOut::To))) => { (false, false, true) }
        Some(CurrentlyEditing::Node(_)) => (false, false, false),
        None => (false, false, false),
    };

    let label_prefix = if label_active { "Label:".yellow().bold() } else { "Label:".into() };
    let from_prefix = if from_active { "From:".yellow().bold() } else { "From:".into() };
    let to_prefix = if to_active { "To:".yellow().bold() } else { "To:".into() };
    let label_value = app.label.clone().yellow();

    let from_value = format!("{}", app.in_outs[0]).yellow();
    let to_value = format!("{}", app.in_outs[1]).yellow();

    let lines = vec![
        Line::from(""),
        Line::from(vec![label_prefix, " ".into(), label_value]),
        Line::from(vec![from_prefix, " ".into(), from_value]),
        Line::from(vec![to_prefix, " ".into(), to_value]),
        Line::from(""),
        Line::from("Enter to advance/save, Esc to cancel, Q to go back.")
    ];

    let body = Text::from(lines);
    let popup_area = centered_rect(60, 40, area);
    Clear.render(popup_area, buf);
    Paragraph::new(body).block(block).render(popup_area, buf);
}

fn build_graph_lines(app: &App) -> Vec<Line<'static>> {
    let mut lines = Vec::new();
    lines.push(
        Line::from(format!("Nodes: {}  Edges: {}", app.graph.nodes.len(), app.graph.edges.len()))
    );

    if app.graph.nodes.is_empty() {
        lines.push(Line::from("No nodes"));
    } else {
        lines.push(Line::from("Nodes:"));
        for node in &app.graph.nodes {
            lines.push(Line::from(format!("- {}: {}", node.id, node.label)));
        }
    }

    if app.graph.edges.is_empty() {
        lines.push(Line::from("No edges"));
    } else {
        lines.push(Line::from("Edges:"));
        for edge in &app.graph.edges {
            lines.push(Line::from(format!("- {} -> {}: {}", edge.from, edge.to, edge.label)));
        }
    }

    lines
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    let horizontal = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(vertical[1]);

    horizontal[1]
}

fn render_exiting(app: &App, area: Rect, buf: &mut Buffer) {
    let title = Line::from("Exiting".bold());
    let block = Block::bordered().title(title.centered()).border_set(border::THICK);
    let counter_text = Text::from(
        vec![Line::from(vec!["Do You Wish to Exit: ".into(), "y/n".yellow()])]
    );
    Paragraph::new(counter_text).centered().block(block).render(area, buf);
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::style::Style;

    #[test]
    fn render() {
        let app = App::new();
        let mut buf = Buffer::empty(Rect::new(0, 0, 50, 4));

        app.render(buf.area, &mut buf);

        let mut expected = Buffer::with_lines(
            vec![
                "┏━━━━━━━━━━━━━ Counter App Tutorial ━━━━━━━━━━━━━┓",
                "┃                    Value: 0                    ┃",
                "┃                                                ┃",
                "┗━ Decrement <Left> Increment <Right> Quit <Q> ━━┛"
            ]
        );
        let title_style = Style::new().bold();
        let counter_style = Style::new().yellow();
        let key_style = Style::new().blue().bold();
        expected.set_style(Rect::new(14, 0, 22, 1), title_style);
        expected.set_style(Rect::new(28, 1, 1, 1), counter_style);
        expected.set_style(Rect::new(13, 3, 6, 1), key_style);
        expected.set_style(Rect::new(30, 3, 7, 1), key_style);
        expected.set_style(Rect::new(43, 3, 4, 1), key_style);

        assert_eq!(buf, expected);
    }
}
