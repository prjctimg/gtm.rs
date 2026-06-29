use std::time::Duration;

use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Text},
    widgets::{Block, Paragraph, Wrap},
};

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let app_result = App::new().run(terminal);
    ratatui::restore();
    app_result
}

struct App {
    counter: i32,
    exit: bool,
}

impl App {
    fn new() -> Self {
        Self {
            counter: 0,
            exit: false,
        }
    }

    fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.render(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn handle_events(&mut self) -> Result<()> {
        if event::poll(Duration::from_millis(250))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => self.exit = true,
                        KeyCode::Char('h') | KeyCode::Left => self.counter -= 1,
                        KeyCode::Char('l') | KeyCode::Right => self.counter += 1,
                        _ => {}
                    }
                }
            }
        }
        Ok(())
    }

    fn render(&self, frame: &mut Frame) {
        let area = frame.area();
        let layout = Layout::vertical([
            Constraint::Length(3),
            Constraint::Fill(1),
            Constraint::Length(3),
        ]);
        let [title_area, main_area, status_area] = area.split(&layout);

        let title = Block::bordered()
            .title(" Ratatui App ".bold())
            .style(Style::default().fg(Color::Cyan));
        frame.render_widget(title, title_area);

        let text = Text::from(vec![
            Line::from(format!("Counter: {}", self.counter)).alignment(Alignment::Center),
            Line::from(""),
            Line::from("Use h/l or ←/→ to change the counter").alignment(Alignment::Center),
            Line::from("Press q or Esc to quit").alignment(Alignment::Center),
        ]);
        let main = Paragraph::new(text)
            .block(Block::bordered())
            .wrap(Wrap { trim: false })
            .alignment(Alignment::Center);
        frame.render_widget(main, main_area);

        let status = Paragraph::new(Line::from(" Q: Quit "))
            .style(Style::default().fg(Color::Gray))
            .block(Block::bordered());
        frame.render_widget(status, status_area);
    }
}
