use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::widgets::{Block, Borders, Paragraph, Sparkline};

pub fn render(f: &mut ratatui::Frame, app: &crate::app::App) {
    let area = f.area();

    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Length(5), Constraint::Min(3)])
        .split(area);

    let header = Paragraph::new("Stash Dashboard").block(Block::default().borders(Borders::ALL));
    f.render_widget(header, rows[0]);

    let stats_col = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(33),
            Constraint::Percentage(33),
            Constraint::Percentage(34),
        ])
        .split(rows[1]);

    let total_skus = app.items.len();
    let low_stock = app.items.iter().filter(|i| i.qty < i.item.reorder_threshold).count();
    let total_value: i64 = app.items.iter().map(|i| i.item.unit_cost.0).sum();

    f.render_widget(stat_widget("Total SKUs", total_skus.to_string()), stats_col[0]);
    f.render_widget(stat_widget("Low Stock", low_stock.to_string()), stats_col[1]);
    f.render_widget(stat_widget("Inventory Value", naira(total_value)), stats_col[2]);

    // Real recent-movement magnitudes, oldest-first so the sparkline reads left-to-right
    // chronologically, fed from the FetchMovements{item_id: None} call made at startup.
    let trend_data: Vec<u64> = app
        .movement_page
        .iter()
        .rev()
        .map(|m| m.movement.quantity_delta().unsigned_abs())
        .collect();

    let sparkline = Sparkline::default()
        .block(Block::default().title("Recent Movement").borders(Borders::ALL))
        .data(&trend_data);
    f.render_widget(sparkline, rows[2]);
}

fn stat_widget(label: &str, value: String) -> Paragraph<'static> {
    Paragraph::new(format!("{label}\n{value}")).block(Block::default().borders(Borders::ALL))
}

fn naira(value: i64) -> String {
    format!("₦{:.2}", value as f64 / 100.0)
}
