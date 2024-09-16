use ratatui::layout::{Alignment, Constraint, Direction, Layout};
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Frame;

use crate::utils::helpers;

use super::App;

impl App {
    pub fn draw(&self, f: &mut Frame) {
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
}
