use ratatui::{text::Span, widgets::{Block, BorderType, Borders, Paragraph}};

use crate::app::colors::{border, text};

pub(super) fn button(text: &'static str) -> Paragraph<'static> {
    Paragraph::new(Span::styled(text, text::normal::LIGHT))
        .block(Block::new().borders(Borders::ALL).border_style(border::LIGHT).border_type(BorderType::Rounded))
}