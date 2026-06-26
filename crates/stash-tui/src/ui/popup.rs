use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::widgets::{Block, Borders, Clear, Paragraph};

// Carves a centered Rect out of a parent area — standard ratatui modal trick.
fn centered_rect(percent_x: u16, percent_y: u16, parent: Rect) -> Rect {
    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(parent);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(vertical[1])[1]
}

pub fn render_confirm_delete(f: &mut ratatui::Frame, message: &str) {
    let area = centered_rect(40, 20, f.area());

    // erases whatever was drawn underneath, otherwise it bleeds through
    f.render_widget(Clear, area);

    let block = Block::default().title("Confirm Delete").borders(Borders::ALL);
    let paragraph = Paragraph::new(format!("{message}\n\n[y] Confirm   [n] Cancel")).block(block);
    f.render_widget(paragraph, area);
}
