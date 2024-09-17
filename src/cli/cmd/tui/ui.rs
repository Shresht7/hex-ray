use ratatui::layout::{Alignment, Constraint, Direction, Layout};
use ratatui::style::{Color, Style, Stylize};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Block, Borders, Padding, Paragraph, Wrap};
use ratatui::Frame;

use crate::utils::format::Format;
use crate::utils::helpers;

use super::App;

impl App {
    pub fn draw(&self, f: &mut Frame) {
        // Create the base layout
        let base_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Length(1),
                    Constraint::Length(self.rows_per_page as u16 + 2),
                    Constraint::Length(1),
                ]
                .as_ref(),
            )
            .spacing(1)
            .split(f.area());

        // Calculate column widths based on format and configuration
        let offset_len = 8 + 4;
        let hex_len = ((self.cfg.format.size() + 1) * self.cfg.size)
            + (self.cfg.size / self.cfg.group_size)
            + 4;
        let ascii_len = (self.cfg.size + 1) + (self.cfg.size / self.cfg.group_size) + 2;

        // Create a layout with three vertical sections
        let columns = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Length(offset_len),       // Offset
                    Constraint::Length(hex_len as u16),   // Hex Values
                    Constraint::Length(ascii_len as u16), // ASCII Values
                    Constraint::Fill(1),                  // Selection Block
                ]
                .as_ref(),
            )
            .split(base_layout[1]);

        // Create a block with borders and title for each column
        let offset_block = Block::default().borders(Borders::ALL);
        let hex_block = Block::default().borders(Borders::ALL);
        let ascii_block = Block::default().borders(Borders::ALL);
        let selection_block = Block::default().padding(Padding::symmetric(4, 1));

        let mut offset_data = Vec::new();
        let mut hex_data = Vec::new();
        let mut ascii_data = Vec::new();
        let mut selection_data = Vec::new();

        let selected_styles = Style::default()
            .bg(Color::Rgb(255, 146, 92))
            .fg(Color::Black)
            .bold();
        let regular_styles = Style::default().fg(Color::White);

        let start = self.scroll_offset;
        let end = std::cmp::min(
            self.scroll_offset + self.rows_per_page * self.cfg.size,
            self.data.len(),
        );

        for (i, row) in self.data[start..end].iter().enumerate() {
            let row_index = start + i;

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

                let style: Style;
                if row_index * self.cfg.size + j == self.selected {
                    selection_data = vec![
                        Line::from(vec![
                            Span::from("Index: "),
                            Span::from(self.selected.to_string()),
                        ]),
                        Line::from(vec![
                            Span::from("\nSelected: "),
                            Span::from(byte_str.clone()),
                        ]),
                        Line::from("\n"),
                        Line::from(vec![Span::from("\nASCII: "), Span::from(ascii_str.clone())]),
                        Line::from(vec![
                            Span::from("\nDecimal: "),
                            Span::from(Format::Decimal.format(*byte)),
                        ]),
                        Line::from(vec![
                            Span::from("\nBinary: "),
                            Span::from(Format::Binary.format(*byte)),
                        ]),
                        Line::from(vec![
                            Span::from("\nOctal: "),
                            Span::from(Format::Octal.format(*byte)),
                        ]),
                        Line::from(vec![
                            Span::from("\nHexadecimal: "),
                            Span::from(Format::Hex.format(*byte)),
                        ]),
                    ];
                    style = selected_styles
                } else {
                    style = regular_styles
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
        let selection_paragraph = Paragraph::new(selection_data).block(selection_block);

        f.render_widget(offset_paragraph, columns[0]);
        f.render_widget(hex_paragraph, columns[1]);
        f.render_widget(ascii_paragraph, columns[2]);
        f.render_widget(selection_paragraph, columns[3]);

        // Help text styled and combined into a single line
        let help_text = vec![
            Span::styled("q / esc  ", Style::default().fg(Color::Green)),
            Span::styled("Quit", Style::default().fg(Color::DarkGray)),
            Span::styled("  •  ", Style::default().fg(Color::DarkGray)),
            Span::styled("← ↑ ↓ →  ", Style::default().fg(Color::Green)),
            Span::styled("Move selection", Style::default().fg(Color::DarkGray)),
        ];

        let help_line = Line::from(help_text);

        let help_paragraph = Paragraph::new(help_line)
            .alignment(Alignment::Center)
            .wrap(ratatui::widgets::Wrap { trim: false });

        f.render_widget(help_paragraph, base_layout[2]);
    }
}
