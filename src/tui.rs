use std::io;

use crossterm::event;
use crossterm::event::{ Event, KeyEventKind };
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::{ Line, Text },
    widgets::{ Block, Paragraph, Widget },
    DefaultTerminal,
    Frame,
};

use crate::app::CurrentScreen;

use crate::app::App;

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

    let counter_text = Text::from(vec![Line::from(vec!["Value: ".into(), "69".yellow()])]);

    Paragraph::new(counter_text).centered().block(block).render(area, buf);
}

fn render_graph_editor(app: &App, area: Rect, buf: &mut Buffer) {
    let title = Line::from(" Tui Graph Editor: Graph ".bold());
    let instructions = Line::from(
        vec![
            " Decrement ".into(),
            "<Left>".blue().bold(),
            " Increment ".into(),
            "<Right>".blue().bold(),
            " Quit ".into(),
            "<Q> ".blue().bold()
        ]
    );
    let block = Block::bordered()
        .title(title.centered())
        .title_bottom(instructions.centered())
        .border_set(border::THICK);

    let counter_text = Text::from(vec![Line::from(vec!["Value: ".into(), "69".yellow()])]);

    Paragraph::new(counter_text).centered().block(block).render(area, buf);
}

fn render_node_editor(app: &App, area: Rect, buf: &mut Buffer) {
    let title = Line::from(" Tui Graph Editor: Node ".bold());
    let instructions = Line::from(
        vec![
            " Decrement ".into(),
            "<Left>".blue().bold(),
            " Increment ".into(),
            "<Right>".blue().bold(),
            " Quit ".into(),
            "<Q> ".blue().bold()
        ]
    );
    let block = Block::bordered()
        .title(title.centered())
        .title_bottom(instructions.centered())
        .border_set(border::THICK);

    let counter_text = Text::from(vec![Line::from(vec!["Value: ".into(), "69".yellow()])]);

    Paragraph::new(counter_text).centered().block(block).render(area, buf);
}

fn render_edge_editor(app: &App, area: Rect, buf: &mut Buffer) {
    let title = Line::from(" Tui Graph Editor: Edge ".bold());
    let instructions = Line::from(
        vec![
            " Decrement ".into(),
            "<Left>".blue().bold(),
            " Increment ".into(),
            "<Right>".blue().bold(),
            " Quit ".into(),
            "<Q> ".blue().bold()
        ]
    );
    let block = Block::bordered()
        .title(title.centered())
        .title_bottom(instructions.centered())
        .border_set(border::THICK);

    let counter_text = Text::from(vec![Line::from(vec!["Value: ".into(), "69".yellow()])]);

    Paragraph::new(counter_text).centered().block(block).render(area, buf);
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
