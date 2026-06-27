pub mod add_item_form;
pub mod dashboard;
pub mod item_list;
pub mod movement_log;
pub mod popup;

pub use add_item_form::AddItemFormState;

pub fn render(f: &mut ratatui::Frame, app: &crate::app::App) {
    match &app.screen {
        crate::app::Screen::Dashboard => dashboard::render(f, app),
        crate::app::Screen::ItemList => item_list::render(f, app),
        crate::app::Screen::ItemDetail(_) => {}
        crate::app::Screen::AddItem(form) => add_item_form::render(f, app, form),
        crate::app::Screen::StockMovementLog => movement_log::render(f, app),
        crate::app::Screen::Settings => {}
    }
}
