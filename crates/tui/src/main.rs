mod app;
use app::app;

use std::io::Result;

use ratatui::run;

fn main() -> Result<()> {
    run(|terminal| app(terminal))
}