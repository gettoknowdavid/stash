use crate::app::App;
use ratatui::layout::Rect;
use ratatui::widgets::{Block, Borders, Paragraph};
use stash_core::ids::CategoryId;

pub fn render(f: &mut ratatui::Frame, app: &App, id: CategoryId, area: Rect) {
    let Some(c) = app.categories.iter().find(|c| c.id == id) else {
        f.render_widget(Paragraph::new("Category not found"), f.area());
        return;
    };

    let body = format!(
        "Name: {}\nDescription: {}\n\n[e] Edit  [d] Delete  [Esc] Back",
        c.name.0,
        c.description.as_deref().unwrap_or("(none)"),
    );
    f.render_widget(
        Paragraph::new(body).block(Block::default().title("Category Detail").borders(Borders::ALL)),
        area,
    );
}
