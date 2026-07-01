use crate::app::App;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::widgets::{Block, Borders, List, ListItem, Paragraph};
use stash_core::ids::ItemId;

pub fn render(f: &mut ratatui::Frame, app: &App, item_id: ItemId, area: Rect) {
    let Some(entry) = app.items.iter().find(|e| e.item.id == item_id) else {
        f.render_widget(
            Paragraph::new("Item not found").block(Block::default().borders(Borders::ALL)),
            area,
        );
        return;
    };

    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(7), Constraint::Length(3), Constraint::Min(0)])
        .split(area);

    let info = Paragraph::new(format!(
        "SKU: {}\nName: {}\nReorder threshold: {}\nUnit cost: ₦{:.2}\nTotal qty (all warehouses): {}",
        entry.item.sku.0,
        entry.item.name,
        entry.item.reorder_threshold,
        entry.item.unit_cost.0 as f64 / 100.0,
        entry.qty,
    ))
        .block(Block::default().title("Item Detail").borders(Borders::ALL));
    f.render_widget(info, rows[0]);

    let warehouse_name = app
        .selected_warehouse
        .and_then(|wid| app.warehouses.iter().find(|w| w.id == wid))
        .map_or("(no warehouse)", |w| w.name.0.as_str());

    let adjust_block = Block::default()
        .title(format!(
            "Adjust @ {warehouse_name} — [{}, Tab to change]",
            app.item_detail.kind.label()
        ))
        .borders(Borders::ALL);
    f.render_widget(
        Paragraph::new(app.item_detail.adjust_input.value()).block(adjust_block),
        rows[1],
    );
    f.set_cursor_position((
        rows[1].x + 1 + app.item_detail.adjust_input.cursor() as u16,
        rows[1].y + 1,
    ));

    let mut help_lines =
        vec![ListItem::new("Enter: submit  Tab: change kind  d: delete  Esc: back")];
    if let Some(err) = &app.item_detail.error {
        help_lines.push(ListItem::new(format!("Error: {err}")).style(app.theme.error));
    }
    f.render_widget(List::new(help_lines), rows[2]);
}
