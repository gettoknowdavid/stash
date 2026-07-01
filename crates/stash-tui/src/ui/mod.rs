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
pub mod warehouse_detail;
pub mod warehouse_form;
pub mod warehouse_list;

pub fn render(f: &mut ratatui::Frame, app: &crate::app::App) {
    match &app.screen {
        crate::app::Screen::Dashboard => dashboard::render(f, app),
        crate::app::Screen::ItemList => item_list::render(f, app),
        crate::app::Screen::ItemDetail(id) => item_detail::render(f, app, *id),
        crate::app::Screen::AddItem(form) => item_form::render(f, app, form),
        crate::app::Screen::CategoryList => category_list::render(f, app),
        crate::app::Screen::CategoryDetail(id) => category_detail::render(f, app, *id),
        crate::app::Screen::AddCategory(form) => category_form::render(f, form),
        crate::app::Screen::WarehouseList => warehouse_list::render(f, app),
        crate::app::Screen::WarehouseDetail(id) => warehouse_detail::render(f, app, *id),
        crate::app::Screen::AddWarehouse(form) => warehouse_form::render(f, form),
        crate::app::Screen::StockMovementLog => movement_log::render(f, app),
        crate::app::Screen::Settings => settings::render(f, app),
    }

    if app.input_mode == crate::app::InputMode::ConfirmingDelete {
        popup::render_confirm_delete(f, "Delete this? This cannot be undone.");
    }
}
