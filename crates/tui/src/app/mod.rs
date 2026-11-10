mod colors;

mod pages;
use pages::{editor, landing, Page};

use std::io::{Result, Stdout};

use crossterm::event::{Event, MouseEventKind, read};
use ratatui::{Terminal, backend::CrosstermBackend};

pub(super) fn app(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<()> {
    // Content organization
    let mut page = Page::Landing;
    
    loop {
        terminal.draw(|frame| {
            match page {
                Page::Landing => landing(frame, vec![]),
                Page::Editor => editor(frame)
            }
        })?;
    }
}