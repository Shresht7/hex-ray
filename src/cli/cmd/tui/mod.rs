// Library
mod app;
mod row;

// Library
use super::View;
use crate::utils::helpers;
use app::App;

impl View {
    pub fn execute_tui(self) -> Result<(), Box<dyn std::error::Error>> {
        self.init();

        let (reader, offset) = helpers::get_reader_and_offset(self.filepath.as_ref(), self.offset)?;

        let mut app = App::new(self);
        app.parse(reader, offset)?;

        app.run()
    }
}
