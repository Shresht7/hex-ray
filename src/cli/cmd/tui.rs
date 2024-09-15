// Library
use clap::Parser;

use ratatui::{
    crossterm::event::{self, KeyCode, KeyEventKind},
    style::Stylize,
    widgets::Paragraph,
    DefaultTerminal,
};

// -----------
// TUI COMMAND
// -----------

#[derive(Parser, Clone)]
#[command(version, about)]
pub struct Tui {}

impl Tui {
    pub fn execute(self) -> Result<(), Box<dyn std::error::Error>> {
        let mut terminal = ratatui::init();
        terminal.clear()?;
        let app_result = self.run(terminal);
        ratatui::restore();
        app_result
    }

    pub fn run(self, mut terminal: DefaultTerminal) -> Result<(), Box<dyn std::error::Error>> {
        // The main draw loop
        loop {
            terminal.draw(|frame| {
                let greeting = Paragraph::new("Hello Ratatui! (press 'q' to quit)")
                    .white()
                    .on_blue();
                frame.render_widget(greeting, frame.area());
            })?;

            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                    return Ok(());
                }
            }
        }
    }
}
