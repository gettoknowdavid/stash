use tui_input::backend::crossterm::EventHandler;

pub mod bridge;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct KeyBinding {
    pub code: crossterm::event::KeyCode,
    pub modifiers: crossterm::event::KeyModifiers,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Action {
    Quit,
    MoveUp,
    MoveDown,
    Select,
    NewItem,
    Delete,
    Cancel,
}

pub struct KeyMap(pub std::collections::HashMap<KeyBinding, Action>);

#[derive(Debug, Clone)]
pub enum Screen {
    Dashboard,
    ItemList,
    CategoryList,
    WarehouseList,
    ItemDetail(stash_core::ids::ItemId),
    CategoryDetail(stash_core::ids::CategoryId),
    WarehouseDetail(stash_core::ids::WarehouseId),
    AddItem(Box<crate::ui::ItemFormState>),
    AddCategory(Box<crate::ui::CategoryFormState>),
    AddWarehouse(Box<crate::ui::WarehouseFormState>),
    StockMovementLog,
    Settings,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum InputMode {
    #[default]
    Normal,
    Editing,
    Searching,
    ConfirmingDelete,
}

#[derive(Debug, Clone)]
pub enum Message {
    Tick,
    KeyPressed(crossterm::event::KeyEvent),
    ItemsLoaded(Vec<stash_core::item::ItemWithStock>),
    ItemSaved(stash_core::item::ItemWithStock),
    CategoriesLoaded(Vec<stash_core::category::Category>),
    CategorySaved(stash_core::category::Category),
    WarehousesLoaded(Vec<stash_core::warehouse::Warehouse>),
    WarehouseSaved(stash_core::warehouse::Warehouse),
    StockUpdated(stash_core::ids::ItemId, u32),
    MovementsLoaded(Vec<stash_core::stock::StockMovementRecord>),
    Error(String),
    None,
}

#[derive(Debug, Clone)]
pub enum Command {
    FetchItems(stash_core::item::ItemFilter),
    SaveItem(stash_storage::item_repository::CreateItemInput),
    UpdateItem(stash_storage::item_repository::UpdateItemInput),
    DeleteItem(stash_core::ids::ItemId),
    FetchCategories,
    FetchWarehouses,
    SaveCategory(stash_storage::category_repository::CreateCategoryInput),
    UpdateCategory(stash_storage::category_repository::UpdateCategoryInput),
    DeleteCategory(stash_core::ids::CategoryId),
    SaveWarehouse(stash_storage::warehouse_repository::CreateWarehouseInput),
    UpdateWarehouse(stash_storage::warehouse_repository::UpdateWarehouseInput),
    DeleteWarehouse(stash_core::ids::WarehouseId),
    FetchMovements {
        item_id: Option<stash_core::ids::ItemId>,
        limit: u32,
        offset: u32,
    },
    RecordMovement {
        item_id: stash_core::ids::ItemId,
        warehouse_id: stash_core::ids::WarehouseId,
        movement: stash_core::stock::StockMovement,
    },
    None,
}

#[derive(Debug, Clone)]
pub struct App {
    pub screen: Screen,
    pub items: Vec<stash_core::item::ItemWithStock>,
    pub categories: Vec<stash_core::category::Category>,
    pub warehouses: Vec<stash_core::warehouse::Warehouse>,
    pub selected: usize,
    pub input_mode: InputMode,
    pub status: Option<String>,
    pub should_quit: bool,
    pub movement_page: Vec<stash_core::stock::StockMovementRecord>,
}
impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
impl App {
    #[must_use]
    pub fn new() -> Self {
        Self {
            screen: Screen::Dashboard,
            items: Vec::new(),
            categories: Vec::new(),
            warehouses: Vec::new(),
            selected: 0,
            input_mode: Default::default(),
            status: None,
            should_quit: false,
            movement_page: Vec::new(),
        }
    }

    pub fn update(&mut self, msg: Message) -> Option<Command> {
        match msg {
            Message::Tick => None,
            Message::KeyPressed(key) => self.handle_key(key),
            Message::ItemsLoaded(items) => {
                self.items = items;
                None
            }
            Message::ItemSaved(v) => {
                if let Some(existing) = self.items.iter_mut().find(|i| i.item.id == v.item.id) {
                    *existing = v;
                } else {
                    self.items.push(v);
                }
                self.screen = Screen::ItemList;
                None
            }
            Message::CategoriesLoaded(categories) => {
                self.categories = categories;
                None
            }
            Message::CategorySaved(category) => {
                if let Some(existing) = self.categories.iter_mut().find(|c| c.id == category.id) {
                    *existing = category;
                } else {
                    self.categories.push(category);
                }
                self.screen = Screen::CategoryList;
                None
            }
            Message::WarehousesLoaded(warehouses) => {
                self.warehouses = warehouses;
                None
            }
            Message::WarehouseSaved(warehouse) => {
                if let Some(existing) = self.warehouses.iter_mut().find(|c| c.id == warehouse.id) {
                    *existing = warehouse;
                } else {
                    self.warehouses.push(warehouse);
                }
                self.screen = Screen::WarehouseList;
                None
            }
            Message::StockUpdated(_, _) => None,
            Message::MovementsLoaded(records) => {
                self.movement_page = records;
                None
            }
            Message::Error(err) => {
                self.status = Some(err);
                None
            }
            Message::None => None,
        }
    }
    pub fn handle_key(&mut self, key: crossterm::event::KeyEvent) -> Option<Command> {
        use crossterm::event::KeyCode;
        match (&self.input_mode, key.code) {
            (InputMode::Normal, KeyCode::Char('c')) => self.i,
            (InputMode::Normal, KeyCode::Char('q')) => {
                self.should_quit = true;
                None
            }
            (InputMode::Normal, KeyCode::Char('j') | KeyCode::Down) => {
                self.selected =
                    self.selected.saturating_add(1).min(self.items.len().saturating_sub(1));
                None
            }
            (InputMode::Normal, KeyCode::Char('k') | KeyCode::Up) => {
                self.selected = self.selected.saturating_sub(1);
                None
            }
            (InputMode::Normal, KeyCode::Enter) => {
                if let Some(entry) = self.items.get(self.selected) {
                    self.screen = Screen::ItemDetail(entry.item.id);
                }
                None
            }
            (InputMode::Normal, KeyCode::Char('n')) => {
                self.screen = Screen::AddItem(Box::from(crate::ui::ItemFormState::default()));
                self.input_mode = InputMode::Editing;
                None
            }
            (InputMode::Editing, KeyCode::Esc) => {
                self.input_mode = InputMode::Normal;
                self.screen = Screen::ItemList;
                None
            }
            (InputMode::Editing, KeyCode::Tab) => {
                if let Screen::AddItem(form) = &mut self.screen {
                    form.focused_field = (form.focused_field + 1) % 3;
                }
                None
            }
            (InputMode::Editing, KeyCode::BackTab) => {
                if let Screen::AddItem(form) = &mut self.screen {
                    form.focused_field = form.focused_field.checked_sub(1).unwrap_or(2);
                }
                None
            }
            (InputMode::Editing, KeyCode::Enter) => {
                if let Screen::AddItem(form) = &mut self.screen {
                    if form.focused_field < 2 {
                        form.focused_field += 1;
                        return None;
                    }
                }
                // last field, attempt to build + submit
                self.try_submit_add_item_form()
            }
            (InputMode::Editing, _) => {
                // every other key (chars, backspace, left/right arrows, home/end)
                // gets routed into whichever field currently has focus
                if let Screen::AddItem(form) = &mut self.screen {
                    let field = match form.focused_field {
                        0 => &mut form.sku,
                        1 => &mut form.name,
                        _ => &mut form.unit_cost,
                    };
                    field.handle_event(&crossterm::event::Event::Key(key));
                }
                None
            }
            _ => None,
        }
    }
    fn try_submit_add_item_form(&mut self) -> Option<Command> {
        let Screen::AddItem(form) = &mut self.screen else { return None };
        form.error = None;

        let sku = match stash_core::sku::Sku::parse(form.sku.value()) {
            Ok(sku) => sku,
            Err(e) => {
                form.error = Some(e.to_string());
                return None;
            }
        };

        if form.name.value().trim().is_empty() {
            form.error = Some("name is required".into());
            return None;
        }

        let unit_cost = match form.unit_cost.value().parse::<i64>() {
            Ok(v) => v,
            Err(_) => {
                form.error = Some("unit cost must be a whole number (kobo)".into());
                return None;
            }
        };

        // placeholder — you don't have category selection in the form yet (see Epic 6/7 section).
        // For now, hardcode a category_id or fetch the first one from self.categories once
        // you've wired up Categories. This is the one piece that genuinely needs category support
        // before this form can fully submit.
        let category_id = stash_core::ids::CategoryId::new(); // TEMP — replace once categories exist

        let input = stash_storage::item_repository::CreateItemInput {
            id: stash_core::ids::ItemId::new(),
            sku,
            name: form.name.value().to_string(),
            description: None,
            category_id,
            unit_cost: stash_core::money::Money(unit_cost),
            reorder_threshold: 0,
        };

        Some(Command::SaveItem(input))
    }
}
