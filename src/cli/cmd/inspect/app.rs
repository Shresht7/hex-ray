use ratatui::DefaultTerminal;

use super::row::Row;
use super::View;

/// The main application state
#[derive(Debug, Default)]
pub struct App {
    pub cfg: View,            // Configuration parameters
    pub data: Vec<Row>,       // The 2D vector of data
    pub total_bytes: usize,   // The total count of bytes
    pub selected: usize,      // The index of the selected byte
    pub scroll_offset: usize, // The scroll position marking the first row to show
    pub rows_per_page: usize, // Number of rows to show per page
    pub exit: bool,           // Should exit the application
}

impl App {
    /// Instantiate a new instance of the application from the configuration parameters
    pub fn new(cfg: View, terminal_height: u16) -> Self {
        // The number of rows to show per scroll-page is determined by the terminal height. (Minimum: 10 rows)
        // We subtract 6 to account for the border, help-line etc. both above and below the viewport.
        let rows_per_page = std::cmp::max(10, terminal_height as usize - 6);
        Self {
            cfg,
            rows_per_page,
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

        // Keep iterating until we run out of bytes to read...
        while bytes_remaining > 0 {
            // Determine the number of bytes to be read in this iteration
            let bytes_to_read = std::cmp::min(bytes_remaining, self.cfg.size);

            // Read the bytes into the buffer
            let bytes_read = reader.read(&mut buffer[0..bytes_to_read])?;
            if bytes_read == 0 {
                break; // break the loop if no bytes were read
            }

            // Add the row data to the vector
            let row = Row::new(&buffer, offset + self.total_bytes);
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
            terminal.draw(|frame| self.draw(frame))?; // Render UI
            self.handle_events()?; // Handle Events
        }
        Ok(())
    }

    /// Get the row number for the given index position
    pub fn row(&mut self, index: usize) -> usize {
        index / self.cfg.size
    }

    /// Get the column number for the given index position
    pub fn col(&mut self, index: usize) -> usize {
        index % self.cfg.size
    }

    /// Calculate the index offset for the given number of rows
    pub fn rows(&mut self, n: usize) -> usize {
        n * self.cfg.size
    }
}
