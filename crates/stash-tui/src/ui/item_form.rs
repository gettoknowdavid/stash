#[derive(Debug, Clone, Default)]
pub struct ItemFormState {
    pub sku: tui_input::Input,
    pub name: tui_input::Input,
    pub unit_cost: tui_input::Input,
    pub focused_field: usize,
    pub error: Option<String>,
    pub category_id: Option<stash_core::ids::CategoryId>,
}

pub fn render(f: &mut ratatui::Frame, _app: &crate::app::App, form: &ItemFormState) {
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
            Constraint::Min(0),
        ])
        .split(area);

    let fields = [(&form.sku, "SKU"), (&form.name, "Name"), (&form.unit_cost, "Unit Cost (₦)")];

    for (i, (input, label)) in fields.iter().enumerate() {
        let is_focused = i == form.focused_field;
        let border_color = if is_focused { Color::Yellow } else { Color::White };

        let block = Block::default()
            .title(*label)
            .borders(Borders::ALL)
            .style(Style::default().fg(border_color));

        // input.value() gives current text content of that field
        let paragraph = Paragraph::new(input.value()).block(block);
        f.render_widget(paragraph, rows[i]);

        // If focused, position the terminal cursor inside this field at input.cursor()
        if is_focused {
            f.set_cursor_position((rows[i].x + 1 + input.cursor() as u16, rows[i].y + 1));
        }
    }

    if let Some(err) = &form.error {
        let err_widget = Paragraph::new(err.as_str()).style(Style::default().fg(Color::Red));
        f.render_widget(err_widget, rows[3]);
    }
}
