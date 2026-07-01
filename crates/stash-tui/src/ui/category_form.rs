#[derive(Debug, Clone, Default)]
pub struct CategoryFormState {
    id: Option<stash_core::ids::CategoryId>,
    name: tui_input::Input,
    description: tui_input::Input,
    focused_field: usize,
    error: Option<String>,
}

pub fn render(f: &mut ratatui::Frame, _app: &crate::app::App, form: &CategoryFormState) {
    let fields = [(&form.name, "Name"), (&form.description, "Description")];
    render_form_view(f, _app, Vec::from(fields), form.focused_field, form.error.clone());
}

pub fn render_form_view(
    f: &mut ratatui::Frame,
    _app: &crate::app::App,
    fields: Vec<(&tui_input::Input, &str)>,
    focused_field: usize,
    error: Option<String>,
) {
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

    for (i, (input, label)) in fields.iter().enumerate() {
        let is_focused = i == focused_field;
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

    if let Some(err) = &error {
        let err_widget = Paragraph::new(err.as_str()).style(Style::default().fg(Color::Red));
        f.render_widget(err_widget, rows[3]);
    }
}
