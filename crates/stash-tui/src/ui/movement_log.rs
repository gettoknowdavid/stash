use ratatui::widgets::{Block, Borders, List, ListItem};

pub fn render(f: &mut ratatui::Frame, app: &crate::app::App) {
    let items: Vec<ListItem> = app
        .movement_page
        .iter()
        .map(|m| ListItem::new(format!("{} {:?} qty Δ", m.created_at, m.movement.kind_str())))
        .collect();

    f.render_widget(
        List::new(items).block(Block::default().title("Movements").borders(Borders::ALL)),
        f.area(),
    );
}
