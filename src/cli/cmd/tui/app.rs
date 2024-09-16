// Library
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::layout::{Alignment, Constraint};
use ratatui::style::{Modifier, Style};
use ratatui::widgets::block::Title;
use ratatui::widgets::{Block, Borders, Cell, Paragraph, Row, Table, Widget};
use ratatui::{DefaultTerminal, Frame};

use super::View;

#[derive(Debug, Default)]
pub struct App {
    cfg: View,
    data: Vec<super::row::Row>,
    total_bytes: usize,
    selected: usize,
    exit: bool,
}

impl App {
    pub fn new(cfg: View) -> Self {
        Self {
            cfg,
            ..Default::default()
        }
    }

    pub fn parse<T>(
        &mut self,
        mut data: T,
        offset: usize,
    ) -> Result<&mut Self, Box<dyn std::error::Error>>
    where
        T: std::io::BufRead,
    {
        // Buffer to store the data
        let mut buffer = vec![0; self.cfg.size];

        // The number of bytes remaining to be read
        let mut bytes_remaining = self.cfg.limit.unwrap_or(usize::MAX);

        while bytes_remaining > 0 {
            // Determine the number of bytes to be read in this iteration
            let bytes_to_read = std::cmp::min(bytes_remaining, self.cfg.size);

            let bytes_read = data.read(&mut buffer[0..bytes_to_read])?;
            if bytes_read == 0 {
                break;
            }

            let row = super::row::Row::parse(&buffer, offset, self.cfg.group_size, bytes_read);
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
        frame.render_widget(self, frame.area());
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
            KeyCode::Right => self.increment(),
            KeyCode::Left => self.decrement(),
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn increment(&mut self) {
        if self.selected < self.total_bytes {
            self.selected += 1;
        }
    }

    fn decrement(&mut self) {
        if self.selected > 0 {
            self.selected -= 1;
        }
    }
}

impl Widget for &App {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        // Convert app data to table rows
        let mut rows: Vec<Row> = Vec::new();
        for (r, row) in self.data.iter().enumerate() {
            rows.push(Row::new(vec![
                Cell::from(row.format_offset()),
                Cell::from(row.format_hex_values(self.selected)),
                Cell::from(row.format_ascii_representation()),
            ]));
        }

        // Define column widths
        let widths = vec![
            Constraint::Length(20),
            Constraint::Length(60),
            Constraint::Length(30),
        ];

        // Create the table
        let table = Table::new(rows, widths)
            .header(
                Row::new(vec![
                    Cell::from("Offset"),
                    Cell::from("Hex"),
                    Cell::from("Ascii"),
                ])
                .style(Style::default().add_modifier(Modifier::BOLD)),
            )
            .block(Block::default().borders(Borders::ALL).title("Data Table"))
            .column_spacing(1);

        table.render(area, buf);
    }
}
