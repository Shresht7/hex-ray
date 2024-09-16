use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};

use super::App;

impl App {
    /// updates the application's state based on user input
    pub fn handle_events(&mut self) -> std::io::Result<()> {
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
