use ratatui::widgets::{Block, Borders, List, ListItem};

pub fn render(f: &mut ratatui::Frame, app: &crate::app::App) {
    let mut items = vec![
        ListItem::new(format!("Categories: {}", app.categories.len())),
        ListItem::new(format!("Warehouses: {}", app.warehouses.len())),
    ];
    if let Some(wid) = app.selected_warehouse {
        if let Some(w) = app.warehouses.iter().find(|w| w.id == wid) {
            items.push(ListItem::new(format!("Active warehouse: {}", w.name.0)));
        }
    }
    f.render_widget(
        List::new(items).block(Block::default().title("Settings").borders(Borders::ALL)),
        f.area(),
    );
}
