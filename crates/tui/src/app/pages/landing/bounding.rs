use std::rc::Rc;

use ratatui::{Frame, layout::{Constraint, Layout, Rect}};

pub(super) fn bounding(
    frame: &mut Frame,
    recents: &Vec<u8>
) -> Rc<[Rect]> {
    let mut constraints = vec![
        Constraint::Length(1), // Name
        Constraint::Length(2), // Tagline
        Constraint::Length(2), // Separator
        Constraint::Length(1), // Start
        Constraint::Length(3), // New
        Constraint::Length(3), // Open
        Constraint::Length(2), // Gap
        Constraint::Length(2), // Recents
        Constraint::Length(1)  // Placeholder
    ];
    let mut height = 17;
    
    if !recents.is_empty() {
        constraints.pop().unwrap();
        height -= 1;
        
        for _ in recents {
            constraints.push(Constraint::Length(3));
            height += 3;
        }
    }

    let main = frame.area().centered(Constraint::Percentage(40), Constraint::Length(height));
    let areas = Layout::vertical(constraints).split(main);

    areas
}