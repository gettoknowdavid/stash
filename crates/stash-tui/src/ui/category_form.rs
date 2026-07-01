use crate::app::{App, CategoryFormState};
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::widgets::{Block, Borders, Paragraph};

pub fn render(f: &mut ratatui::Frame, app: &App, form: &CategoryFormState, area: Rect) {
    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Length(3), Constraint::Min(0)])
        .split(area);

    let title = if form.editing_id.is_some() { "Edit Category" } else { "New Category" };
    let fields = [(&form.name, "Name"), (&form.description, "Description (optional)")];

    for (i, (input, label)) in fields.iter().enumerate() {
        let focused = i == form.focused_field;
        let block = Block::default()
            .title(if i == 0 { title } else { label })
            .borders(Borders::ALL)
            .style(app.theme.border_style(focused));
        f.render_widget(Paragraph::new(input.value()).block(block), rows[i]);
        if focused {
            f.set_cursor_position((rows[i].x + 1 + input.cursor() as u16, rows[i].y + 1));
        }
    }

    if let Some(err) = &form.error {
        f.render_widget(Paragraph::new(err.as_str()).style(app.theme.error), rows[2]);
    }
}
