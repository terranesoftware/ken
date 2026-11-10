use ratatui::{Frame, style::Stylize, widgets::Block};

use crate::app::colors::background;

pub(super) fn background(frame: &mut Frame) {
    let background = Block::new().bg(background::LIGHT);
    let screen = frame.area();
    frame.render_widget(background, screen);
}