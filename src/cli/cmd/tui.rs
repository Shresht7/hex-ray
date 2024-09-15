// Library
use crate::utils::format::Format;
use clap::Parser;

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

// -----------
// TUI COMMAND
// -----------

#[derive(Parser, Clone)]
#[command(version, about)]
pub struct Tui {
    /// Path to the file to read (defaults to reading from `stdin` if empty)
    #[clap(aliases = ["path", "src"])]
    pub filepath: Option<std::path::PathBuf>,

    /// The byte offset at which to start reading; i.e. skip the given number of bytes.
    ///
    /// You can specify a positive or negative integer value; A positive integer offset
    /// seeks forward from the start, while a negative offset seeks backwards from the end
    #[arg(aliases = ["skip", "seek"], short, long, default_value_t = 0)]
    pub offset: i64,

    /// The number of bytes to read.
    ///
    /// The program will stop after reading the specified number of bytes.
    #[arg(short, long)]
    pub limit: Option<usize>,

    /// The size of each row
    #[arg(short, long, default_value_t = 16)]
    pub size: usize,

    /// The output display format.
    ///
    /// This can be one of the following: hex (x), HEX (X), binary (b), octal (o), decimal (d).
    ///
    /// To output with the corresponding prefixes prepend a `#` to the format (e.g. `#hex` or `#x`)
    #[arg(short, long, default_value = "hex")]
    pub format: Format,

    /// Chunk the output into groups of this size
    #[arg(alias = "chunk", short, long, default_value_t = 4)]
    pub group_size: usize,
}

impl Tui {
    pub fn execute(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut terminal = ratatui::init();
        terminal.clear()?;
        let app_result = App::default().run(&mut terminal);
        ratatui::restore();
        app_result
    }
}

#[derive(Debug, Default)]
struct App {
    counter: u8,
    exit: bool,
}

impl App {
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
            KeyCode::Right => self.increment(),
            KeyCode::Left => self.decrement(),
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn increment(&mut self) {
        self.counter += 1;
    }

    fn decrement(&mut self) {
        self.counter -= 1;
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

        let counter_text = Text::from(vec![Line::from(vec![
            "Value: ".into(),
            self.counter.to_string().yellow(),
        ])]);

        Paragraph::new(counter_text)
            .centered()
            .block(block)
            .render(area, buf);
    }
}
