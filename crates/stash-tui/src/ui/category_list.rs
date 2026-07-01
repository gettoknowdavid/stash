use ratatui::style::{Modifier, Style};
use ratatui::widgets::{Block, Borders, List, ListItem};

pub fn render(f: &mut ratatui::Frame, app: &crate::app::App) {
    let items: Vec<ListItem> = app
        .categories
        .iter()
        .enumerate()
        .map(|(i, c)| {
            let style = if i == app.category_selected {
                Style::default().add_modifier(Modifier::REVERSED)
            } else {
                Style::default()
            };
            ListItem::new(c.name.0.clone()).style(style)
        })
        .collect();

    f.render_widget(
        List::new(items).block(
            Block::default().title("Categories (n: new, Enter: view)").borders(Borders::ALL),
        ),
        f.area(),
    );
}
