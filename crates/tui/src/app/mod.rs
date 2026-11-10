mod colors;

mod pages;
use pages::{editor, landing, Page};

use std::io::{Result, Stdout};

use ratatui::{Terminal, backend::CrosstermBackend};

pub(super) fn app(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<()> {
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