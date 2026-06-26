use ratatui::layout::Constraint;
use ratatui::prelude::Color;
use ratatui::style::{Modifier, Style};
use ratatui::widgets::{Block, Borders, Cell, Row, Table};

// IMPORTANT: do NOT filter/sort app.items here on every frame for large datasets.
// Instead, App should hold a `filtered_items: Vec<usize>` (indices into `items`)
// that's recomputed only in update() when search/sort/data changes — not in render().
pub fn render(f: &mut ratatui::Frame, app: &crate::app::App) {
    let area = f.area();

    let header = Row::new(vec!["SKU", "Name", "Category", "Qty", "Status"])
        .style(Style::default().add_modifier(Modifier::BOLD));

    // ratatui's Table only actually renders the rows that fit on screen —
    // but we still must avoid re-sorting/re-filtering the full Vec every frame.
    let rows: Vec<Row> = app
        .items
        .iter()
        .enumerate()
        .map(|(i, item)| {
            let below = false;
            let style = if below {
                Style::default().fg(Color::Red)
            } else if i == app.selected {
                Style::default().bg(Color::Blue)
            } else {
                Style::default()
            };

            Row::new(vec![
                Cell::from(item.sku.0.clone()),
                Cell::from(item.name.clone()),
                Cell::from(item.category_id.0.to_string()),
                Cell::from("—".to_string()),
                Cell::from(if below { "LOW" } else { "OK" }),
            ])
            .style(style)
        })
        .collect();

    let widths = [
        Constraint::Length(12),
        Constraint::Percentage(30),
        Constraint::Length(15),
        Constraint::Length(8),
        Constraint::Length(8),
    ];

    let table = Table::new(rows, widths)
        .header(header)
        .block(Block::default().title("Items").borders(Borders::ALL));

    f.render_widget(table, area);
}
