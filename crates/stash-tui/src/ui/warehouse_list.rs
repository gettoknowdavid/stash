use ratatui::style::{Modifier, Style};
use ratatui::widgets::{Block, Borders, List, ListItem};

pub fn render(f: &mut ratatui::Frame, app: &crate::app::App) {
    let items: Vec<ListItem> = app
        .warehouses
        .iter()
        .enumerate()
        .map(|(i, w)| {
            let style = if i == app.warehouse_selected {
                Style::default().add_modifier(Modifier::REVERSED)
            } else {
                Style::default()
            };
            let active = if app.selected_warehouse == Some(w.id) { " (active)" } else { "" };
            ListItem::new(format!("{}{}", w.name.0, active)).style(style)
        })
        .collect();

    f.render_widget(
        List::new(items).block(
            Block::default().title("Warehouses (n: new, Enter: view)").borders(Borders::ALL),
        ),
        f.area(),
    );
}
