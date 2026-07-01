use ratatui::widgets::{Block, Borders, List, ListItem};

pub fn render(f: &mut ratatui::Frame, app: &crate::app::App) {
    let items: Vec<ListItem> = app
        .movement_page
        .iter()
        .map(|m| {
            ListItem::new(format!(
                "{} {} {:+} ({})",
                m.created_at,
                m.movement.kind_str(),
                m.movement.quantity_delta(),
                m.movement.reason()
            ))
        })
        .collect();

    f.render_widget(
        List::new(items).block(Block::default().title("Movements").borders(Borders::ALL)),
        f.area(),
    );
}
