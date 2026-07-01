use crate::app;
use ratatui::layout::Rect;
use ratatui::style::{Modifier, Style};
use ratatui::widgets::Paragraph;

pub fn render(f: &mut ratatui::Frame, app: &app::App, area: Rect) {
    let text = "[ Stash Inventory Management System • 1.0.1 ]";
    let style = Style::default().fg(app.theme.title).add_modifier(Modifier::BOLD);
    f.render_widget(Paragraph::new(text).right_aligned().style(style), area);
}
