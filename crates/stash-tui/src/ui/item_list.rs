use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::widgets::{Block, Borders, Cell, Paragraph, Row, Table};

pub fn render(f: &mut ratatui::Frame, app: &crate::app::App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)])
        .split(area);

    let search_title = if app.input_mode == crate::app::InputMode::Searching {
        "Search (Esc to clear, Enter to lock in)"
    } else {
        "Search ( / to search )"
    };
    let is_search_bar_focused = app.focused_pane == crate::app::Pane::ItemsSearchBar;
    let search = Paragraph::new(app.search_input.value()).block(
        Block::default()
            .title(search_title)
            .borders(Borders::ALL)
            .border_style(app.theme.border_style(is_search_bar_focused)),
    );
    f.render_widget(search, chunks[0]);

    let header = Row::new(vec!["SKU", "Name", "Category", "Qty", "Status"])
        .style(Style::default().add_modifier(Modifier::BOLD));

    // We only ever iterate `app.filtered`, which is recomputed in App::update — not here —
    // so a large catalog doesn't re-filter/re-sort every single frame.
    let rows: Vec<Row> = app
        .filtered
        .iter()
        .enumerate()
        .filter_map(|(visual_idx, &item_idx)| {
            let entry = app.items.get(item_idx)?;
            let below = entry.qty < entry.item.reorder_threshold;
            let style = if below {
                app.theme.low_stock
            } else if visual_idx == app.item_selected {
                app.theme.selected
            } else {
                app.theme.ok_stock
            };

            Some(
                Row::new(vec![
                    Cell::from(entry.item.sku.0.clone()),
                    Cell::from(entry.item.name.clone()),
                    Cell::from(entry.item.category_id.0.to_string()),
                    Cell::from(entry.qty.to_string()),
                    Cell::from(if below { "LOW" } else { "OK" }),
                ])
                .style(style),
            )
        })
        .collect();

    let widths = [
        Constraint::Length(12),
        Constraint::Percentage(30),
        Constraint::Length(15),
        Constraint::Length(8),
        Constraint::Length(8),
    ];

    let is_table_focused = app.focused_pane == crate::app::Pane::ItemsList;
    let table = Table::new(rows, widths).header(header).block(
        Block::default()
            .title("Items")
            .borders(Borders::ALL)
            .border_style(app.theme.border_style(is_table_focused)),
    );

    f.render_widget(table, chunks[1]);
}
