// Library
use super::View;
use crate::utils::format::Format;
use crate::utils::helpers;

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

impl View {
    pub fn execute_tui(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.init();

        let (reader, offset) = helpers::get_reader_and_offset(self.filepath.as_ref(), self.offset)?;

        let mut app = App::default();
        app.parse(self, reader, offset)?;

        let mut terminal = ratatui::init();
        terminal.clear()?;
        let app_result = app.run(&mut terminal);
        ratatui::restore();
        app_result
    }
}

#[derive(Debug)]
struct Row {
    data: Vec<u8>,
    offset: usize,
    group_size: usize,
    bytes_read: usize,
}

impl Row {
    fn parse(data: &[u8], offset: usize, group_size: usize, bytes_read: usize) -> Self {
        Self {
            data: data.to_vec(),
            offset,
            group_size,
            bytes_read,
        }
    }

    fn to_string(&self) -> String {
        let offset = self.format_offset();
        let hex_values = self.format_hex_values();
        let ascii_values = self.format_ascii_representation();
        format!("│ {} │ {} │ {} │", offset, hex_values, ascii_values)
    }

    fn format_offset(&self) -> String {
        let res = Format::Octal.format(self.offset as u8);
        if res.len() > 8 {
            return format!("{:0>8}", res);
        }

        let mut padding = String::from(" ");
        for _ in 0..(8 - res.len()) {
            padding.push_str(&"·");
        }

        format!("{}{}", padding, res)
    }

    fn format_hex_values(&self) -> String {
        let mut s = String::new();
        // Print the hex values
        for (j, byte) in self.data.iter().take(self.bytes_read).enumerate() {
            // Group values by applying spacing
            if j > 0 && j % self.group_size == 0 {
                s.push_str(" ");
            }
            let value = Format::Hex.format(*byte);
            s.push_str(&value); // Format each byte as a 2-wide hexadecimal value
            s.push_str(" ");
        }

        // Print spacing if the chunk is less than size bytes
        for k in self.bytes_read..self.data.len() {
            // Group values by applying spacing
            if k > 0 && k % self.group_size == 0 {
                s.push_str(" ");
            }
            // Group values by applying spacing
            s.push_str(&" ".repeat(Format::Hex.size() + 1)); // Each missing byte is represented by 3 spaces (two for hex-digits and one space)
        }

        s
    }

    /// Print the ASCII columns
    fn format_ascii_representation(&self) -> String {
        let mut s = String::new();

        // Print the ASCII representation
        for (k, byte) in self.data.iter().enumerate() {
            // Group characters by applying spacing
            if k > 0 && k % self.group_size == 0 {
                s.push_str(" ");
            }
            // If there are still bytes to read, print the ASCII character...
            if k < self.bytes_read {
                let c = if helpers::is_printable_ascii_character(&byte) {
                    let char = (*byte as char).to_string();
                    format!("{}", char)
                } else {
                    format!("{}", "·") // Non-printable ASCII characters are replaced by a dot
                };
                s.push_str(c.as_str());
            } else {
                s.push_str(" "); // Else if there are no more bytes left in this iteration, just print an empty space
            }
        }

        s
    }
}

#[derive(Debug, Default)]
struct App {
    data: Vec<Row>,
    total_bytes: usize,
    size: usize,
    exit: bool,
}

impl App {
    fn push(&mut self, row: Row) -> &Self {
        self.data.push(row);
        self
    }

    fn parse<T>(
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
            self.push(row);

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
