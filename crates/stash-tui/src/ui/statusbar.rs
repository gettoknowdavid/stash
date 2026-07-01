use crate::app;
use crate::app::{InputMode, Screen};
use ratatui::layout::Rect;
use ratatui::style::{Modifier, Style};
use ratatui::widgets::Paragraph;

fn hints_for(app: &app::App) -> &'static str {
    if app.input_mode == InputMode::ConfirmingDelete {
        return "y: confirm delete n / Esc: Cancel";
    }
    match (&app.screen, &app.input_mode) {
        (Screen::ItemList, InputMode::Searching) => {
            "type to search items   Enter: lock in   Esc: clear"
        }
        (Screen::ItemList, _) => {
            "j/k: move   Enter: view   n: new   /: search   1-6: switch screen   q: quit"
        }
        (Screen::ItemDetail(_), _) => {
            "Enter: submit adjustment   Tab: change kind   d: delete   Esc: back"
        }
        (Screen::AddItem(_), _) => {
            "Tab/Shift+Tab: field   ←/→: category   Enter: next/submit   Esc: cancel"
        }
        (Screen::CategoryList, _) | (Screen::WarehouseList, _) => {
            "j/k: move   Enter: view   n: new   1-6: switch screen   q: quit"
        }
        (Screen::CategoryDetail(_), _) | (Screen::WarehouseDetail(_), _) => {
            "e: edit   d: delete   Esc: back"
        }
        (Screen::AddCategory(_), _) | (Screen::AddWarehouse(_), _) => {
            "Tab/Shift+Tab: field   Enter: next/submit   Esc: cancel"
        }
        _ => "1-6: switch screen   q: quit",
    }
}

pub fn render(f: &mut ratatui::Frame, app: &app::App, area: Rect) {
    let (text, style) = match &app.status {
        Some(msg) => {
            (format!(" {msg}"), Style::default().fg(app.theme.error).add_modifier(Modifier::BOLD))
        }
        None => (format!(" {}", hints_for(app)), app.theme.status_bar),
    };
    f.render_widget(Paragraph::new(text).style(style), area);
}
