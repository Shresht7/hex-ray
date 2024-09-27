use ratatui::layout::{Alignment, Constraint, Direction, Layout};
use ratatui::style::{Color, Style, Stylize};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Padding, Paragraph};
use ratatui::Frame;

use crate::utils::format::Format;
use crate::utils::helpers;

use super::App;

impl App {
    /// Draw the UI to the screen
    pub fn draw(&self, f: &mut Frame) {
        // Create the base layout
        let base_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Length(1),                             // Header
                    Constraint::Length(self.rows_per_page as u16 + 2), // Main Content (+2 for top and bottom border)
                    Constraint::Length(1),                             // Help
                ]
                .as_ref(),
            )
            .spacing(1)
            .split(f.area());

        // Render the Header component
        f.render_widget(self.header(), base_layout[0]);

        // Calculate column widths based on format and configuration
        let offset_len = 8 + 4; // 8 digits + (2 space + 2 borders)
        let hex_len = ((self.cfg.format.size() + 1) * self.cfg.size) // Format size (e.g. 2 for Hex) + 1 whitespace
            + (self.cfg.size / self.cfg.group_size) // Extra whitespace for group separators
            + 4; // + 2 outer space + 2 borders
        let ascii_len = (self.cfg.size + 1) + (self.cfg.size / self.cfg.group_size) + 2; // (1 ASCII char + 1 whitespace) + (group spacing) + borders

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

        // Describe the style of the selected element
        let selected_style = Style::default()
            .bg(Color::Rgb(255, 146, 92))
            .fg(Color::Black)
            .bold();

        // Determine the starting and ending rows for the data slice
        let start = self.scroll_offset;
        let end = std::cmp::min(
            self.scroll_offset + self.rows_per_page * self.cfg.size,
            self.data.len(),
        );

        // Iterate over the data slice ...
        for (i, row) in self.data[start..end].iter().enumerate() {
            let row_index = start + i; // The absolute row index
            let is_selected_row = self.row(self.selected) == row_index;

            // Offset column
            let offset_str = row.format_offset();
            let offset_spans = if is_selected_row {
                Span::from(offset_str.bold().white())
            } else {
                Span::from(offset_str)
            };
            offset_data.push(Line::from(offset_spans));

            // Hex Values column
            let mut hex_spans = Vec::new();
            let mut ascii_spans = Vec::new();
            for (j, byte) in row.data.iter().enumerate() {
                // Group values by applying spacing
                if j > 0 && j % self.cfg.group_size == 0 {
                    hex_spans.push(Span::from(" "));
                    ascii_spans.push(Span::from(" "));
                }

                // Format the byte and ascii values
                let byte_str = self.cfg.format.format(*byte);
                let ascii_str = if helpers::is_printable_ascii_character(byte) {
                    Span::from((*byte as char).to_string())
                } else {
                    Span::from("·".dark_gray())
                };

                // If this is the selected element, style it differently
                if row_index * self.cfg.size + j == self.selected {
                    selection_data = self.format_selection_block(&byte_str, &ascii_str, byte);
                    hex_spans.push(Span::styled(byte_str, selected_style));
                    ascii_spans.push(ascii_str.style(selected_style));
                } else {
                    // Otherwise, just add them as is
                    hex_spans.push(Span::from(byte_str));
                    ascii_spans.push(ascii_str);
                };
                hex_spans.push(Span::from(" "));
            }

            // Add the spans to the line
            hex_data.push(Line::from(hex_spans));
            ascii_data.push(Line::from(ascii_spans));
        }

        // Create the block paragraphs and add them to the main section
        let offset_paragraph = Paragraph::new(offset_data)
            .block(offset_block)
            .alignment(Alignment::Center);
        let hex_paragraph = Paragraph::new(hex_data)
            .block(hex_block)
            .alignment(Alignment::Center)
            .style(Color::White);
        let ascii_paragraph = Paragraph::new(ascii_data)
            .block(ascii_block)
            .alignment(Alignment::Center)
            .style(Color::White);
        let selection_paragraph = Paragraph::new(selection_data).block(selection_block);

        f.render_widget(offset_paragraph, columns[0]);
        f.render_widget(hex_paragraph, columns[1]);
        f.render_widget(ascii_paragraph, columns[2]);
        f.render_widget(selection_paragraph, columns[3]);

        // Render the Help component
        f.render_widget(self.help(), base_layout[2]);
    }

    /// Render the header
    fn header(&self) -> Paragraph<'static> {
        Paragraph::new("·• Hex·Ray •·")
            .alignment(Alignment::Center)
            .bold()
            .white()
    }

    /// Render the selection block
    fn format_selection_block(
        &self,
        byte_str: &String,
        ascii_str: &Span<'static>,
        byte: &u8,
    ) -> Vec<Line> {
        vec![
            Line::from(vec![
                Span::from("Index: "),
                Span::from(self.selected.to_string().white()),
            ]),
            Line::from(vec![
                Span::from("\nSelected:    "),
                Span::from(byte_str.clone().white()),
            ]),
            Line::from("\n"),
            Line::from(vec![
                Span::from("\nASCII:       "),
                Span::from(ascii_str.clone()).white(),
            ]),
            Line::from(vec![
                Span::from("\nBinary:      "),
                Span::from(Format::Binary.format(*byte).white()),
            ]),
            Line::from(vec![
                Span::from("\nHexadecimal: "),
                Span::from(Format::Hex.format(*byte).white()),
            ]),
            Line::from(vec![
                Span::from("\nDecimal:     "),
                Span::from(Format::Decimal.format(*byte).white()),
            ]),
            Line::from(vec![
                Span::from("\nOctal:       "),
                Span::from(Format::Octal.format(*byte).white()),
            ]),
        ]
    }

    // Render the help line
    fn help(&self) -> Paragraph<'static> {
        // Help text styled and combined into a single line
        let help_text = vec![
            Span::styled("q / esc  ", Style::default().fg(Color::Green)),
            Span::styled("Quit", Style::default().fg(Color::DarkGray)),
            Span::styled("  •  ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                "pgup home ← ↑ ↓ → end pgdn ",
                Style::default().fg(Color::Green),
            ),
            Span::styled("Move selection", Style::default().fg(Color::DarkGray)),
        ];
        let help_line = Line::from(help_text);

        Paragraph::new(help_line)
            .alignment(Alignment::Center)
            .wrap(ratatui::widgets::Wrap { trim: false })
    }
}
