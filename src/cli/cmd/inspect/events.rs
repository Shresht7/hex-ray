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
            KeyCode::Up => self.move_selection_up(),
            KeyCode::Right => self.move_selection_right(),
            KeyCode::Down => self.move_selection_down(),
            KeyCode::Left => self.move_selection_left(),
            KeyCode::Home => self.move_selection_to_home(),
            KeyCode::End => self.move_selection_to_end(),
            KeyCode::PageUp => self.scroll_up(),
            KeyCode::PageDown => self.scroll_down(),
            KeyCode::Esc | KeyCode::Char('q') => self.exit(),
            _ => {}
        }
    }

    // ----------------
    // COMMAND HANDLERS
    // ----------------

    // Select the element in the row above
    fn move_selection_up(&mut self) {
        // If the selection is beyond the first row ...
        if self.selected >= self.cfg.size * 1 {
            // ...Subtract the size of a row to move the selection up by 1 row
            self.selected = self.selected.saturating_sub(self.cfg.size);
            self.adjust_scroll_view();
        }
    }

    /// Select the previous element
    fn move_selection_left(&mut self) {
        // If this is not the first element ...
        if self.selected > 0 {
            self.selected = self.selected.saturating_sub(1); // ...Move to the left by one
            self.adjust_scroll_view();
        }
    }

    // Select the element in the row above
    fn move_selection_down(&mut self) {
        // If the selection is not in the last row ...
        if self.selected + self.cfg.size <= self.total_bytes + 1 {
            self.selected += self.cfg.size * 1; // Move it down by one row
            self.adjust_scroll_view();
        }
    }

    /// Select the next element
    fn move_selection_right(&mut self) {
        // If this is not the last element ...
        if self.selected <= self.total_bytes {
            self.selected += 1; // ... Move it to the right by one
            self.adjust_scroll_view();
        }
    }

    /// Select the first element in the row
    fn move_selection_to_home(&mut self) {
        let remainder = self.selected % self.cfg.size;
        self.selected -= remainder;
    }

    /// Select the last element in the row
    fn move_selection_to_end(&mut self) {
        let remainder = self.selected % self.cfg.size;
        self.selected += self.cfg.size - remainder - 1;
    }

    /// Scroll up a page
    fn scroll_up(&mut self) {
        // If the selection is beyond the first page ...
        if self.selected > self.cfg.size * self.rows_per_page {
            // ... Go up one page
            self.selected = self
                .selected
                .saturating_sub(self.cfg.size * self.rows_per_page);
        } else {
            // Otherwise, just set the selection to the first element
            self.selected = 0;
        }
        // Move the scroll view up by one page
        self.scroll_offset = self.scroll_offset.saturating_sub(self.rows_per_page);
    }

    /// Scroll down a page
    fn scroll_down(&mut self) {
        // If the selected element is in the last page ...
        if self.selected >= self.total_bytes - (self.cfg.size * self.rows_per_page) {
            // ... set it to be the last element
            self.selected = self.total_bytes + 1;
        } else {
            // Otherwise, go down one page
            self.selected += self.cfg.size * self.rows_per_page;
        }
        // If the selection goes beyond the scroll view
        if self.selected > self.cfg.size * (self.rows_per_page + self.scroll_offset) {
            // Scroll down one page
            self.scroll_offset += self.rows_per_page;
        }
    }

    fn adjust_scroll_view(&mut self) {
        // Now, if the selection falls above the first row in the view ...
        if self.selected < self.cfg.size * self.scroll_offset {
            self.scroll_offset = self.scroll_offset.saturating_sub(1); // Scroll up by one row
        } else if self.selected >= self.cfg.size * (self.rows_per_page + self.scroll_offset) {
            // Otherwise if the selection goes beyond the last row in the view, scroll down by one row
            self.scroll_offset += 1;
        }
    }

    /// Exits the application
    fn exit(&mut self) {
        self.exit = true;
    }
}
