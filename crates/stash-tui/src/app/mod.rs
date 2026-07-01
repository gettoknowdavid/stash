use crossterm::event::{KeyCode, KeyEvent};
use stash_core::ids::{ItemId, WarehouseId};
use tui_input::backend::crossterm::EventHandler;

pub mod bridge;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct KeyBinding {
    pub code: crossterm::event::KeyCode,
    pub modifiers: crossterm::event::KeyModifiers,
}
impl KeyBinding {
    const fn plain(code: crossterm::event::KeyCode) -> Self {
        Self { code, modifiers: crossterm::event::KeyModifiers::NONE }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Action {
    Quit,
    MoveUp,
    MoveDown,
    Select,
    NewItem,
    NewCategory,
    NewWarehouse,
    Delete,
    Confirm,
    Cancel,
    Search,
}

#[derive(Clone, Debug)]
pub struct KeyMap(pub std::collections::HashMap<KeyBinding, Action>);
impl Default for KeyMap {
    fn default() -> Self {
        use crossterm::event::KeyCode;
        let mut m = std::collections::HashMap::new();
        m.insert(KeyBinding::plain(KeyCode::Char('q')), Action::Quit);
        m.insert(KeyBinding::plain(KeyCode::Char('j')), Action::MoveDown);
        m.insert(KeyBinding::plain(KeyCode::Down), Action::MoveDown);
        m.insert(KeyBinding::plain(KeyCode::Char('k')), Action::MoveUp);
        m.insert(KeyBinding::plain(KeyCode::Up), Action::MoveUp);
        m.insert(KeyBinding::plain(KeyCode::Enter), Action::Select);
        m.insert(KeyBinding::plain(KeyCode::Char('n')), Action::NewItem);
        m.insert(KeyBinding::plain(KeyCode::Char('d')), Action::Delete);
        m.insert(KeyBinding::plain(KeyCode::Delete), Action::Delete);
        m.insert(KeyBinding::plain(KeyCode::Char('y')), Action::Confirm);
        m.insert(KeyBinding::plain(KeyCode::Char('/')), Action::Search);
        Self(m)
    }
}
impl KeyMap {
    pub fn resolve(&self, key: KeyEvent) -> Option<Action> {
        self.0.get(&KeyBinding { code: key.code, modifiers: key.modifiers }).copied()
    }
}

#[derive(Debug, Clone)]
pub enum Screen {
    Dashboard,
    ItemList,
    CategoryList,
    WarehouseList,
    ItemDetail(ItemId),
    CategoryDetail(stash_core::ids::CategoryId),
    WarehouseDetail(WarehouseId),
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AdjustKind {
    #[default]
    Inbound,
    Outbound,
    Adjustment,
}
impl AdjustKind {
    const fn next(self) -> Self {
        match self {
            Self::Inbound => Self::Outbound,
            Self::Outbound => Self::Adjustment,
            Self::Adjustment => Self::Inbound,
        }
    }

    pub const fn label(self) -> &'static str {
        match self {
            Self::Inbound => "Inbound (+)",
            Self::Outbound => "Outbound (-)",
            Self::Adjustment => "Adjustment (±)",
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct ItemDetailState {
    pub adjust_input: tui_input::Input,
    pub kind: AdjustKind,
    pub error: Option<String>,
}

#[derive(Debug, Clone)]
pub enum Message {
    Tick,
    KeyPressed(KeyEvent),

    ItemsLoaded(Vec<stash_core::item::ItemWithStock>),
    ItemSaved(stash_core::item::ItemWithStock),
    ItemDeleted(ItemId),

    CategoriesLoaded(Vec<stash_core::category::Category>),
    CategorySaved(stash_core::category::Category),
    CategoryDeleted(stash_core::ids::CategoryId),

    WarehousesLoaded(Vec<stash_core::warehouse::Warehouse>),
    WarehouseSaved(stash_core::warehouse::Warehouse),
    WarehouseDeleted(WarehouseId),

    StockUpdated(ItemId, u32),
    MovementsLoaded(Vec<stash_core::stock::StockMovementRecord>),
    Error(String),
    None,
}

#[derive(Debug, Clone)]
pub enum Command {
    FetchItems(stash_core::item::ItemFilter),
    SaveItem(stash_storage::item_repository::CreateItemInput),
    UpdateItem(stash_storage::item_repository::UpdateItemInput),
    DeleteItem(ItemId),

    FetchCategories,
    SaveCategory(stash_storage::category_repository::CreateCategoryInput),
    UpdateCategory(stash_storage::category_repository::UpdateCategoryInput),
    DeleteCategory(stash_core::ids::CategoryId),

    FetchWarehouses,
    SaveWarehouse(stash_storage::warehouse_repository::CreateWarehouseInput),
    UpdateWarehouse(stash_storage::warehouse_repository::UpdateWarehouseInput),
    DeleteWarehouse(WarehouseId),

    FetchMovements {
        item_id: Option<ItemId>,
        limit: u32,
        offset: u32,
    },
    RecordMovement {
        item_id: ItemId,
        warehouse_id: WarehouseId,
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
    pub selected_warehouse: Option<WarehouseId>,
    pub input_mode: InputMode,
    pub status: Option<String>,
    pub should_quit: bool,
    pub movement_page: Vec<stash_core::stock::StockMovementRecord>,
    pub item_detail: ItemDetailState,
    pub pending_delete: Option<ItemId>,
    pub search_input: tui_input::Input,
    pub filtered: Vec<usize>,
    pub keymap: KeyMap,
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
            selected_warehouse: None,
            input_mode: InputMode::default(),
            status: None,
            should_quit: false,
            movement_page: Vec::new(),
            item_detail: ItemDetailState::default(),
            pending_delete: None,
            search_input: tui_input::Input::default(),
            filtered: Vec::new(),
            keymap: KeyMap::default(),
        }
    }

    fn recompute_filter(&mut self) {
        use fuzzy_matcher::skim::SkimMatcherV2;
        use fuzzy_matcher::FuzzyMatcher;

        let query = self.search_input.value();
        if query.trim().is_empty() {
            self.filtered = (0..self.items.len()).collect();
            return;
        }

        let matcher = SkimMatcherV2::default();
        let mut scored: Vec<(usize, i64)> = self
            .items
            .iter()
            .enumerate()
            .filter_map(|(i, entry)| {
                let haystack = format!("{} {}", entry.item.sku.0, entry.item.name);
                matcher.fuzzy_match(&haystack, query).map(|score| (i, score))
            })
            .collect();
        scored.sort_by(|a, b| b.1.cmp(&a.1));
        self.filtered = scored.into_iter().map(|(i, _)| i).collect();
        self.selected = 0;
    }

    pub fn update(&mut self, msg: Message) -> Option<Command> {
        match msg {
            Message::Tick => None,
            Message::KeyPressed(key) => self.handle_key(key),
            Message::ItemsLoaded(items) => {
                self.items = items;
                self.recompute_filter();
                self.selected = self.selected.min(self.filtered.len().saturating_sub(1));
                None
            }
            Message::ItemSaved(v) => {
                if let Some(existing) = self.items.iter_mut().find(|i| i.item.id == v.item.id) {
                    *existing = v;
                } else {
                    self.items.push(v);
                }
                self.recompute_filter();
                self.screen = Screen::ItemList;
                self.input_mode = InputMode::Normal;
                None
            }
            Message::ItemDeleted(id) => {
                self.items.retain(|i| i.item.id != id);
                self.recompute_filter();
                self.selected = self.selected.min(self.filtered.len().saturating_sub(1));
                self.screen = Screen::ItemList;
                self.input_mode = InputMode::Normal;
                self.status = Some("Item deleted".into());
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
            Message::CategoryDeleted(id) => {
                self.categories.retain(|c| c.id != id);
                self.screen = Screen::CategoryList;
                self.input_mode = InputMode::Normal;
                self.status = Some("Category deleted".into());
                None
            }
            Message::WarehousesLoaded(warehouses) => {
                if self.selected_warehouse.is_none() {
                    self.selected_warehouse = warehouses.first().map(|w| w.id);
                }
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
            Message::WarehouseDeleted(id) => {
                self.warehouses.retain(|w| w.id != id);
                self.screen = Screen::WarehouseList;
                self.input_mode = InputMode::Normal;
                self.status = Some("Warehouse deleted".into());
                None
            }
            Message::StockUpdated(item_id, qty) => {
                if let Some(entry) = self.items.iter_mut().find(|i| i.item.id == item_id) {
                    entry.qty = i64::from(qty);
                }
                self.item_detail = ItemDetailState::default();
                self.status = Some("Stock updated".into());
                Some(Command::FetchMovements { item_id: Some(item_id), limit: 20, offset: 0 })
            }
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
    pub fn handle_key(&mut self, key: KeyEvent) -> Option<Command> {
        use crossterm::event::KeyCode;
        match &self.input_mode {
            InputMode::Normal => self.handle_key_normal(key),
            InputMode::Editing => self.handle_key_editing(key),
            InputMode::Searching => {
                match key.code {
                    KeyCode::Esc => {
                        self.input_mode = InputMode::Normal;
                        self.search_input = tui_input::Input::default();
                        self.recompute_filter();
                    }
                    KeyCode::Enter => self.input_mode = InputMode::Normal,
                    _ => {
                        self.search_input.handle_event(&crossterm::event::Event::Key(key));
                        self.recompute_filter();
                    }
                }
                None
            }
            InputMode::ConfirmingDelete => match self.keymap.resolve(key) {
                Some(Action::Confirm) => {
                    self.input_mode = InputMode::Normal;
                    self.pending_delete.take().map(Command::DeleteItem)
                }
                Some(Action::Cancel) | _
                    if matches!(key.code, KeyCode::Char('n') | KeyCode::Esc) =>
                {
                    self.input_mode = InputMode::Normal;
                    self.pending_delete = None;
                    None
                }
                _ => None,
            },
        }
    }
    fn handle_key_normal(&mut self, key: KeyEvent) -> Option<Command> {
        let Some(action) = self.keymap.resolve(key) else { return None };
        match (action, &self.screen) {
            (Action::Quit, _) => {
                self.should_quit = true;
                None
            }
            (Action::MoveDown, Screen::ItemList | Screen::CategoryList | Screen::WarehouseList) => {
                self.selected =
                    self.selected.saturating_add(1).min(self.filtered.len().saturating_sub(1));
                None
            }
            (Action::MoveUp, Screen::ItemList | Screen::CategoryList | Screen::WarehouseList) => {
                self.selected = self.selected.saturating_sub(1);
                None
            }
            (Action::Search, Screen::ItemList) => {
                self.input_mode = InputMode::Searching;
                None
            }
            (Action::Select, Screen::ItemList) => {
                let idx = self.filtered.get(self.selected).copied()?;
                let id = self.items.get(idx)?.item.id;
                self.screen = Screen::ItemDetail(id);
                self.item_detail = ItemDetailState::default();
                Some(Command::FetchMovements { item_id: Some(id), limit: 20, offset: 0 })
            }
            (Action::NewItem, Screen::ItemList) => {
                self.screen = Screen::AddItem(Box::new(crate::ui::ItemFormState::default()));
                self.input_mode = InputMode::Editing;
                None
            }
            (Action::NewCategory, Screen::CategoryList) => {
                self.screen =
                    Screen::AddCategory(Box::new(crate::ui::CategoryFormState::default()));
                self.input_mode = InputMode::Editing;
                None
            }
            (Action::NewWarehouse, Screen::WarehouseList) => {
                self.screen =
                    Screen::AddWarehouse(Box::new(crate::ui::WarehouseFormState::default()));
                self.input_mode = InputMode::Editing;
                None
            }
            (Action::Delete, Screen::ItemDetail(id)) => {
                self.pending_delete = Some(*id);
                self.input_mode = InputMode::ConfirmingDelete;
                None
            }
            (Action::Cancel, Screen::ItemDetail(_)) => {
                self.screen = Screen::ItemList;
                None
            }
            _ => None,
        }
    }
    fn handle_key_editing(&mut self, key: KeyEvent) -> Option<Command> {
        use crossterm::event::KeyCode;

        // The item-detail "edit" screen handles its own field (a single quantity input)
        // separately from the add-item form's multi-field tabbing.
        if let Screen::ItemDetail(id) = &self.screen {
            let id = *id;
            return self.handle_key_item_detail(key, id);
        }

        match key.code {
            KeyCode::Esc => {
                self.input_mode = InputMode::Normal;
                self.screen = Screen::ItemList;
                None
            }
            KeyCode::BackTab => {
                if let Screen::AddItem(form) = &mut self.screen {
                    form.focused_field = form.focused_field.checked_sub(1).unwrap_or(3);
                }
                None
            }
            KeyCode::Enter => {
                if let Screen::AddItem(form) = &mut self.screen {
                    if form.focused_field < 3 {
                        form.focused_field += 1;
                        return None;
                    }
                }
                self.try_submit_add_item_form()
            }
            KeyCode::Left | KeyCode::Right if self.is_category_field_focused() => {
                if let Screen::AddItem(form) = &mut self.screen {
                    if !self.categories.is_empty() {
                        let len = self.categories.len();
                        form.category_index = if key.code == KeyCode::Right {
                            (form.category_index + 1) % len
                        } else {
                            form.category_index.checked_sub(1).unwrap_or(len - 1)
                        };
                    }
                }
                None
            }
            KeyCode::Tab => {
                if let Screen::AddItem(form) = &mut self.screen {
                    form.focused_field = (form.focused_field + 1) % 4;
                }
                None
            }
            _ => {
                if let Screen::AddItem(form) = &mut self.screen {
                    if form.focused_field != 3 {
                        let field = match form.focused_field {
                            0 => &mut form.sku,
                            1 => &mut form.name,
                            _ => &mut form.unit_cost,
                        };
                        field.handle_event(&crossterm::event::Event::Key(key));
                    }
                }
                None
            }
        }
    }
    fn is_category_field_focused(&self) -> bool {
        matches!(&self.screen, Screen::AddItem(form) if form.focused_field == 3)
    }
    fn handle_key_item_detail(&mut self, key: KeyEvent, item_id: ItemId) -> Option<Command> {
        use crossterm::event::KeyCode;
        match key.code {
            KeyCode::Esc => {
                self.input_mode = InputMode::Normal;
                self.screen = Screen::ItemList;
                None
            }
            KeyCode::Tab => {
                self.item_detail.kind = self.item_detail.kind.next();
                None
            }
            KeyCode::Char('d') => {
                self.pending_delete = Some(item_id);
                self.input_mode = InputMode::ConfirmingDelete;
                None
            }
            KeyCode::Enter => self.try_submit_stock_adjustment(item_id),
            _ => {
                self.item_detail.adjust_input.handle_event(&crossterm::event::Event::Key(key));
                None
            }
        }
    }
    fn try_submit_stock_adjustment(&mut self, item_id: ItemId) -> Option<Command> {
        self.item_detail.error = None;

        let Some(warehouse_id) = self.selected_warehouse else {
            self.item_detail.error = Some("no warehouse available".into());
            return None;
        };

        let raw = self.item_detail.adjust_input.value();
        let movement = match self.item_detail.kind {
            AdjustKind::Inbound => match raw.parse::<u32>() {
                Ok(qty) => stash_core::stock::StockMovement::Inbound {
                    qty,
                    reason: "manual adjustment".into(),
                },
                Err(_) => {
                    self.item_detail.error = Some("enter a whole, non-negative quantity".into());
                    return None;
                }
            },
            AdjustKind::Outbound => match raw.parse::<u32>() {
                Ok(qty) => stash_core::stock::StockMovement::Outbound {
                    qty,
                    reason: "manual adjustment".into(),
                },
                Err(_) => {
                    self.item_detail.error = Some("enter a whole, non-negative quantity".into());
                    return None;
                }
            },
            AdjustKind::Adjustment => match raw.parse::<i32>() {
                Ok(delta) => stash_core::stock::StockMovement::Adjustment {
                    delta,
                    reason: "manual adjustment".into(),
                },
                Err(_) => {
                    self.item_detail.error = Some("enter a whole number (e.g. -3 or 5)".into());
                    return None;
                }
            },
        };

        Some(Command::RecordMovement { item_id, warehouse_id, movement })
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

        let Some(category) = self.categories.get(form.category_index) else {
            form.error = Some("no category available — create one first".into());
            return None;
        };

        let input = stash_storage::item_repository::CreateItemInput {
            id: ItemId::new(),
            sku,
            name: form.name.value().to_string(),
            description: None,
            category_id: category.id,
            unit_cost: stash_core::money::Money(unit_cost),
            reorder_threshold: 0,
        };

        Some(Command::SaveItem(input))
    }
}
