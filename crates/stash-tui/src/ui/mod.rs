pub mod category_form;
pub mod dashboard;
pub mod item_form;
pub mod item_list;
pub mod movement_log;
pub mod popup;
pub mod warehouse_form;

pub use crate::app::Screen;
pub use category_form::CategoryFormState;
pub use item_form::ItemFormState;
pub use warehouse_form::WarehouseFormState;

pub fn render(f: &mut ratatui::Frame, app: &crate::app::App) {
    match &app.screen {
        Screen::Dashboard => dashboard::render(f, app),
        Screen::ItemList => item_list::render(f, app),
        Screen::CategoryList => {}
        Screen::WarehouseList => {}
        Screen::Settings => {}
        Screen::ItemDetail(form) => {}
        Screen::CategoryDetail(form) => {}
        Screen::WarehouseDetail(form) => {}
        Screen::AddItem(form) => item_form::render(f, app, form),
        Screen::AddCategory(form) => category_form::render(f, app, form),
        Screen::AddWarehouse(form) => warehouse_form::render(f, app, form),
        Screen::StockMovementLog => movement_log::render(f, app),
    }
}
