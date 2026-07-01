#[derive(Debug, Clone, Default)]
pub struct ItemFormState {
    pub sku: tui_input::Input,
    pub name: tui_input::Input,
    pub unit_cost: tui_input::Input,
    pub category_index: usize,
    pub focused_field: usize,
    pub error: Option<String>,
}

pub fn render(f: &mut ratatui::Frame, app: &crate::app::App, form: &ItemFormState) {
    use ratatui::layout::{Constraint, Direction, Layout};
    use ratatui::style::{Color, Style};
    use ratatui::widgets::{Block, Borders, Paragraph};

    let area = f.area();

    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Min(0),
        ])
        .split(area);

    let text_fields =
        [(&form.sku, "SKU"), (&form.name, "Name"), (&form.unit_cost, "Unit Cost (kobo)")];

    for (i, (input, label)) in text_fields.iter().enumerate() {
        let is_focused = i == form.focused_field;
        let border_color = if is_focused { Color::Yellow } else { Color::White };
        let block = Block::default()
            .title(*label)
            .borders(Borders::ALL)
            .style(Style::default().fg(border_color));
        f.render_widget(Paragraph::new(input.value()).block(block), rows[i]);
        if is_focused {
            f.set_cursor_position((rows[i].x + 1 + input.cursor() as u16, rows[i].y + 1));
        }
    }

    let category_focused = form.focused_field == 3;
    let category_label = app
        .categories
        .get(form.category_index)
        .map_or("(none — create one first)".to_string(), |c| c.name.0.clone());
    let category_block = Block::default()
        .title("Category (← →)")
        .borders(Borders::ALL)
        .style(Style::default().fg(if category_focused { Color::Yellow } else { Color::White }));
    f.render_widget(Paragraph::new(category_label).block(category_block), rows[3]);

    if let Some(err) = &form.error {
        let err_widget = Paragraph::new(err.as_str()).style(Style::default().fg(Color::Red));
        f.render_widget(err_widget, rows[4]);
    }
}
