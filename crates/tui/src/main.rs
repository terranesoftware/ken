mod app;
use app::app;
use crossterm::{event::{DisableMouseCapture, EnableMouseCapture}, execute};

use std::io::{Result, stdout};

use ratatui::{init, restore};

fn main() -> Result<()> {
    let mut terminal = init();

    execute!(stdout(), EnableMouseCapture)?;
    
    app(&mut terminal)?;

    execute!(stdout(), DisableMouseCapture)?;
    
    restore();

    Ok(())
}