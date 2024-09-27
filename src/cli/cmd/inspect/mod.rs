// Library
mod app;
mod events;
mod row;
mod ui;

// Library
use super::View;
use crate::utils::helpers;
use app::App;

impl View {
    /// View the hex-dump in an interactive session
    pub fn execute_interactively(self) -> Result<(), Box<dyn std::error::Error>> {
        self.init(); // Initialize the configuration

        // Get the reader and starting offset
        let (reader, offset) = helpers::get_reader_and_offset(self.filepath.as_ref(), self.offset)?;

        // Initialize the terminal
        let mut terminal = ratatui::init();
        terminal.clear()?;

        // Initialize the application
        let size = terminal.size()?;
        let mut app = App::new(self, size.height);
        app.parse(reader, offset)?;

        // Run the application
        let app_result = app.run(&mut terminal);

        // Restore the terminal and return
        ratatui::restore();
        app_result
    }
}
