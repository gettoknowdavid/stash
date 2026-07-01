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
pub mod statusbar;
pub mod warehouse_detail;
pub mod warehouse_form;
pub mod warehouse_list;

pub fn render(f: &mut ratatui::Frame, app: &crate::app::App) {
    use ratatui::layout::{Constraint, Direction, Layout};

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(1)])
        .split(f.area());

    let (content, status) = (chunks[0], chunks[1]);

    match &app.screen {
        Screen::Dashboard => dashboard::render(f, app, content),
        Screen::ItemList => item_list::render(f, app, content),
        Screen::ItemDetail(id) => item_detail::render(f, app, *id, content),
        Screen::AddItem(form) => item_form::render(f, app, form, content),
        Screen::CategoryList => category_list::render(f, app, content),
        Screen::CategoryDetail(id) => category_detail::render(f, app, *id, content),
        Screen::AddCategory(form) => category_form::render(f, app, form, content),
        Screen::WarehouseList => warehouse_list::render(f, app, content),
        Screen::WarehouseDetail(id) => warehouse_detail::render(f, app, *id, content),
        Screen::AddWarehouse(form) => warehouse_form::render(f, app, form, content),
        Screen::StockMovementLog => movement_log::render(f, app, content),
        Screen::Settings => settings::render(f, app, content),
    }

    statusbar::render(f, app, status);

    if app.input_mode == crate::app::InputMode::ConfirmingDelete {
        popup::render_confirm_delete(f, "Delete this? This cannot be undone.");
    }
}
