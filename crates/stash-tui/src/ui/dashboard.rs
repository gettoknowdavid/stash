use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::widgets::{Block, Borders, Paragraph, Sparkline};

pub fn render(f: &mut ratatui::Frame, app: &crate::app::App) {
    let area = f.area();

    // Split the whole screen into 3 vertical rows: a header strip, a stats row, a chart row.
    // Constraint::Length(3) = fixed 3 rows; Constraint::Min(0) = take remaining space.
    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Length(5), Constraint::Length(0)])
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
    f.render_widget(stat_widget("Inventory Value", Naira::from(total_value)), stats_col[2]);

    // Sparkline needs &[u64] data — pull from recent movement counts
    // (precompute, don't allocate per frame if avoidable)
    let trend_data: Vec<u64> = vec![3, 5, 2, 8, 6, 9, 4];
    let sparkline = Sparkline::default()
        .block(Block::default().title("Recent Movement").borders(Borders::ALL))
        .data(&trend_data);
    f.render_widget(sparkline, rows[2]);
}

fn stat_widget(label: &str, value: String) -> Paragraph<'static> {
    Paragraph::new(format!("{label}\n{value}")).block(Block::default().borders(Borders::ALL))
}

struct Naira;
impl Naira {
    #[must_use]
    pub fn from(value: i64) -> String {
        format!("₦{:.2}", value as f64 / 100.0)
    }
}
