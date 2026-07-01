use ratatui::widgets::{Block, Borders, Paragraph};

pub fn render(f: &mut ratatui::Frame, app: &crate::app::App, id: stash_core::ids::WarehouseId) {
    let Some(w) = app.warehouses.iter().find(|w| w.id == id) else {
        f.render_widget(Paragraph::new("Warehouse not found"), f.area());
        return;
    };

    let body = format!(
        "Name: {}\nLocation: {}\n\n[e] Edit  [d] Delete  [Esc] Back",
        w.name.0,
        w.location.as_deref().unwrap_or("(none)"),
    );
    f.render_widget(
        Paragraph::new(body)
            .block(Block::default().title("Warehouse Detail").borders(Borders::ALL)),
        f.area(),
    );
}
