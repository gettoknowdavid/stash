use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, Borders, Paragraph};

pub fn render(f: &mut ratatui::Frame, form: &crate::app::WarehouseFormState) {
    let area = f.area();
    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Length(3), Constraint::Min(0)])
        .split(area);

    let title = if form.editing_id.is_some() { "Edit Warehouse" } else { "New Warehouse" };
    let fields = [(&form.name, "Name"), (&form.location, "Location (optional)")];

    for (i, (input, label)) in fields.iter().enumerate() {
        let focused = i == form.focused_field;
        let block = Block::default()
            .title(if i == 0 { title } else { label })
            .borders(Borders::ALL)
            .style(Style::default().fg(if focused { Color::Yellow } else { Color::White }));
        f.render_widget(Paragraph::new(input.value()).block(block), rows[i]);
        if focused {
            f.set_cursor_position((rows[i].x + 1 + input.cursor() as u16, rows[i].y + 1));
        }
    }

    if let Some(err) = &form.error {
        f.render_widget(
            Paragraph::new(err.as_str()).style(Style::default().fg(Color::Red)),
            rows[2],
        );
    }
}
