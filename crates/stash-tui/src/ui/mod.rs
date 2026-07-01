use crate::app::Screen;

pub mod category_detail;
pub mod category_form;
pub mod category_list;
pub mod dashboard;
pub mod item_detail;
pub mod item_form;
pub mod item_list;
pub mod movement_log;
pub mod popup;
pub mod settings;
pub mod sidebar;
pub mod statusbar;
pub mod title;
pub mod warehouse_detail;
pub mod warehouse_form;
pub mod warehouse_list;

pub fn render(f: &mut ratatui::Frame, app: &crate::app::App) {
    use ratatui::layout::{Constraint, Direction, Layout};

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Min(0),
            Constraint::Length(1),
        ])
        .split(f.area());

    let (title, content, status) = (chunks[1], chunks[2], chunks[3]);

    title::render(f, app, title);

    let body = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(22), Constraint::Min(0)])
        .split(content);

    sidebar::render(f, app, body[0]);

    match &app.screen {
        Screen::Dashboard => dashboard::render(f, app, body[1]),
        Screen::ItemList => item_list::render(f, app, body[1]),
        Screen::ItemDetail(id) => item_detail::render(f, app, *id, body[1]),
        Screen::AddItem(form) => item_form::render(f, app, form, body[1]),
        Screen::CategoryList => category_list::render(f, app, body[1]),
        Screen::CategoryDetail(id) => category_detail::render(f, app, *id, body[1]),
        Screen::AddCategory(form) => category_form::render(f, app, form, body[1]),
        Screen::WarehouseList => warehouse_list::render(f, app, body[1]),
        Screen::WarehouseDetail(id) => warehouse_detail::render(f, app, *id, body[1]),
        Screen::AddWarehouse(form) => warehouse_form::render(f, app, form, body[1]),
        Screen::StockMovementLog => movement_log::render(f, app, body[1]),
        Screen::Settings => settings::render(f, app, body[1]),
    }

    statusbar::render(f, app, status);

    if app.input_mode == crate::app::InputMode::ConfirmingDelete {
        popup::render_confirm_delete(f, "Delete this? This cannot be undone.");
    }
}
