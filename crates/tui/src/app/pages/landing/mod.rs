mod background;
use background::background;

mod bounding;
use bounding::bounding;

mod start;
use start::button;

use ratatui::{Frame, style::Stylize, text::{Line, Span}, widgets::{Block, Borders}};

use crate::app::colors::{accent, border, text};

pub(crate) fn landing(
    frame: &mut Frame,
    // Placeholder, this should be something like Vec<Project>
    projects: Vec<u8>
) {
    // Render the background
    background(frame);

    // Content organization
    let areas = bounding(frame, &projects);

    // Render the name
    let name = Line::from(vec![
        Span::styled("ken", text::normal::LIGHT),
        Span::styled(".", accent::LIGHT)
    ]).bold().centered();
    frame.render_widget(name, areas[0]);

    // Render the tagline
    let tagline = Span::styled("The text editor is back.", text::muted::LIGHT).into_centered_line();
    frame.render_widget(tagline, areas[1]);

    // Render the separator
    let separator = Block::new().borders(Borders::TOP).border_style(border::LIGHT);
    frame.render_widget(separator, areas[2]);

    // Render the Start section header
    let start = Span::styled("Start", text::muted::LIGHT);
    frame.render_widget(start, areas[3]);

    // Render the New button
    let new = button(" + New");
    frame.render_widget(new, areas[4]);

    // Render the Open button
    let open = button(" ▤ Open");
    frame.render_widget(open, areas[5]);
    
    let recents = Span::styled("Recents", text::muted::LIGHT);
    frame.render_widget(recents, areas[7]);

    if projects.is_empty() {
        let empty = Span::styled("Open something to make this go away...", text::normal::LIGHT);
        frame.render_widget(empty, areas[8]);
    }
    else {
        // Would create a button for each project and stuff
    }
}