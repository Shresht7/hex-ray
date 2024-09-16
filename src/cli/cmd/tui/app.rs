// Library
use crossterm::event::{Event, KeyEvent};
use ratatui::{
    crossterm::event::{self, KeyCode, KeyEventKind},
    layout::Alignment,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{
        block::{Position, Title},
        Block, Paragraph, Widget,
    },
    DefaultTerminal, Frame,
};

use super::row::Row;
use super::View;

#[derive(Debug, Default)]
pub struct App {
    data: Vec<Row>,
    total_bytes: usize,
    size: usize,
    exit: bool,
}

impl App {
    pub fn parse<T>(
        &mut self,
        cfg: &mut View,
        mut data: T,
        offset: usize,
    ) -> Result<&mut Self, Box<dyn std::error::Error>>
    where
        T: std::io::BufRead,
    {
        self.size = cfg.size;

        // Buffer to store the data
        let mut buffer = vec![0; self.size];

        // The number of bytes remaining to be read
        let mut bytes_remaining = cfg.limit.unwrap_or(usize::MAX);

        while bytes_remaining > 0 {
            // Determine the number of bytes to be read in this iteration
            let bytes_to_read = std::cmp::min(bytes_remaining, self.size);

            let bytes_read = data.read(&mut buffer[0..bytes_to_read])?;
            if bytes_read == 0 {
                break;
            }

            let row = Row::parse(&buffer, offset, cfg.group_size, bytes_read);
            self.data.push(row);

            self.total_bytes += bytes_read;
            bytes_remaining -= bytes_read;
        }

        Ok(self)
    }

    pub fn run(
        &mut self,
        terminal: &mut DefaultTerminal,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // The main draw loop
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area())
    }

    /// updates the application's state based on user input
    fn handle_events(&mut self) -> std::io::Result<()> {
        match event::read()? {
            // it's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        }
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}

impl Widget for &App {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let title = Title::from("HEX-RAY".bold());
        let instructions = Title::from(Line::from(vec![
            " Decrement ".into(),
            "<Left>".blue().bold(),
            " Increment ".into(),
            "<Right>".blue().bold(),
            " Quit ".into(),
            "<Q> ".blue().bold(),
        ]));

        let block = Block::bordered()
            .title(title.alignment(Alignment::Center))
            .title(
                instructions
                    .alignment(Alignment::Center)
                    .position(Position::Bottom),
            )
            .border_set(border::THICK);

        let lines: Vec<Line> = self
            .data
            .iter()
            .map(|l| Line::from(l.to_string()))
            .collect();
        let counter_text = Text::from(lines);

        Paragraph::new(counter_text).block(block).render(area, buf);
    }
}
