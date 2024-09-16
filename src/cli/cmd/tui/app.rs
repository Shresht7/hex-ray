// Library
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::layout::{Alignment, Constraint, Direction, Layout};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::{DefaultTerminal, Frame};

use crate::utils::helpers;

use super::row::Row;
use super::View;

/// The main application state
#[derive(Debug, Default)]
pub struct App {
    cfg: View,          // Configuration parameters
    data: Vec<Row>,     // The 2-D vector of data
    total_bytes: usize, // The total count of bytes
    selected: usize,    // The index of the selected byte
    exit: bool,         // Should exit the application
}

impl App {
    /// Instantiate a new instance of the application from the configuration parameters
    pub fn new(cfg: View) -> Self {
        Self {
            cfg,
            ..Default::default()
        }
    }

    /// Parse the data from the reader
    pub fn parse<T>(
        &mut self,
        mut reader: T,
        offset: usize,
    ) -> Result<&mut Self, Box<dyn std::error::Error>>
    where
        T: std::io::BufRead,
    {
        // Buffer to store the data
        let mut buffer = vec![0; self.cfg.size];

        // The number of bytes remaining to be read
        let mut bytes_remaining = self.cfg.limit.unwrap_or(usize::MAX);

        // Keep iterating until we run out of bytes to read
        while bytes_remaining > 0 {
            // Determine the number of bytes to be read in this iteration
            let bytes_to_read = std::cmp::min(bytes_remaining, self.cfg.size);

            // Read the bytes into the buffer
            let bytes_read = reader.read(&mut buffer[0..bytes_to_read])?;
            if bytes_read == 0 {
                break; // break the loop if no bytes were read
            }

            // Add the row data to the vector
            let row = Row::parse(
                &buffer,
                offset + self.total_bytes,
                self.cfg.group_size,
                bytes_read,
            );
            self.data.push(row);

            // Update the total bytes and the number of bytes remaining
            self.total_bytes += bytes_read;
            bytes_remaining -= bytes_read;
        }

        Ok(self)
    }

    /// Run the application in the terminal
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

    // --
    // UI
    // --

    fn draw(&self, f: &mut Frame) {
        // Create the base layout
        let base_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(self.data.len() as u16 + 2)].as_ref())
            .split(f.area());

        let offset_len = 8 + 4;
        let hex_len = ((self.cfg.format.size() + 1) * self.cfg.size)
            + (self.cfg.size / self.cfg.group_size)
            + 2;
        let ascii_len = (self.cfg.size + 1) + (self.cfg.size / self.cfg.group_size) + 2;

        // Create a layout with three vertical sections
        let columns = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Length(offset_len),       // Offset
                    Constraint::Length(hex_len as u16),   // Hex Values
                    Constraint::Length(ascii_len as u16), // ASCII Values
                ]
                .as_ref(),
            )
            .split(base_layout[0]);

        // Create a block with borders and title for each column
        let offset_block = Block::default().borders(Borders::ALL);
        let hex_block = Block::default().borders(Borders::ALL);
        let ascii_block = Block::default().borders(Borders::ALL);

        let mut offset_data = Vec::new();
        let mut hex_data = Vec::new();
        let mut ascii_data = Vec::new();

        let selected_styles = Style::default().bg(Color::Yellow);
        let regular_styles = Style::default().fg(Color::White);

        for (i, row) in self.data.iter().enumerate() {
            // Offset column
            offset_data.push(row.format_offset());

            // Hex Values column
            let mut hex_spans = Vec::new();
            let mut ascii_spans = Vec::new();
            for (j, byte) in row.data.iter().enumerate() {
                // Group values by applying spacing
                if j > 0 && j % self.cfg.group_size == 0 {
                    hex_spans.push(Span::from(" "));
                    ascii_spans.push(Span::from(" "));
                }

                let byte_str = self.cfg.format.format(*byte);
                let ascii_str = if helpers::is_printable_ascii_character(byte) {
                    (*byte as char).to_string()
                } else {
                    String::from("·")
                };

                let style = if i * self.cfg.size + j == self.selected {
                    selected_styles
                } else {
                    regular_styles
                };
                hex_spans.push(Span::styled(byte_str, style));
                hex_spans.push(Span::from(" "));
                ascii_spans.push(Span::styled(ascii_str, style));
            }

            hex_data.push(Line::from(hex_spans));
            ascii_data.push(Line::from(ascii_spans));
        }

        let offset_paragraph = Paragraph::new(offset_data)
            .block(offset_block)
            .alignment(Alignment::Center)
            .style(Style::default().fg(Color::White));
        let hex_paragraph = Paragraph::new(hex_data)
            .block(hex_block)
            .alignment(Alignment::Center)
            .style(Style::default().fg(Color::White));
        let ascii_paragraph = Paragraph::new(ascii_data)
            .block(ascii_block)
            .alignment(Alignment::Center)
            .style(Style::default().fg(Color::White));

        f.render_widget(offset_paragraph, columns[0]);
        f.render_widget(hex_paragraph, columns[1]);
        f.render_widget(ascii_paragraph, columns[2]);
    }

    // --------------
    // EVENT HANDLERS
    // --------------

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

    /// matches the given key-event and calls the corresponding handler
    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Up => self.move_selection_up(),
            KeyCode::Right => self.move_selection_left(),
            KeyCode::Down => self.move_selection_down(),
            KeyCode::Left => self.move_selection_right(),
            _ => {}
        }
    }

    // ----------------
    // COMMAND HANDLERS
    // ----------------

    // Select the element in the row above
    fn move_selection_up(&mut self) {
        if self.selected > self.cfg.size {
            self.selected -= self.cfg.size;
        }
    }

    /// Select the previous element
    fn move_selection_right(&mut self) {
        if self.selected > 0 {
            self.selected -= 1;
        }
    }

    // Select the element in the row above
    fn move_selection_down(&mut self) {
        if self.selected + self.cfg.size < self.total_bytes {
            self.selected += self.cfg.size;
        }
    }

    /// Select the next element
    fn move_selection_left(&mut self) {
        if self.selected < self.total_bytes - 1 {
            self.selected += 1;
        }
    }

    /// Exits the application
    fn exit(&mut self) {
        self.exit = true;
    }
}
