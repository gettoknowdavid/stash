use ratatui::layout::Rect;
use ratatui::style::{Modifier, Style};
use ratatui::widgets::{Block, Borders, List, ListItem};

pub fn render(f: &mut ratatui::Frame, app: &crate::app::App, area: Rect) {
    let items: Vec<ListItem> = crate::app::SECTIONS
        .iter()
        .enumerate()
        .map(|(i, label)| {
            let mut style = Style::default();
            if i == app.sidebar_selected {
                style = style.add_modifier(Modifier::REVERSED);
            }
            ListItem::new(*label).style(style)
        })
        .collect();

    let focused = app.focused_pane == crate::app::Pane::Sidebar;
    let block = Block::default()
        .title("Sections")
        .borders(Borders::ALL)
        .border_style(app.theme.border_style(focused));
    f.render_widget(List::new(items).block(block), area);
}
