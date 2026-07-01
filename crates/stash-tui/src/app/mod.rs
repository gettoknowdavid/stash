use crossterm::event::KeyEvent;
use stash_core::category::Category;
use stash_core::ids::{CategoryId, ItemId, WarehouseId};
use stash_core::item::ItemWithStock;
use stash_core::stock::StockMovementRecord;
use stash_core::warehouse::Warehouse;
use tui_input::backend::crossterm::EventHandler;

pub mod bridge;

#[derive(Debug, Clone, Default)]
pub struct ItemFormState {
    pub sku: tui_input::Input,
    pub name: tui_input::Input,
    pub unit_cost: tui_input::Input,
    pub category_index: usize,
    pub focused_field: usize,
    pub error: Option<String>,
    pub editing_id: Option<ItemId>,
}

#[derive(Debug, Clone, Default)]
pub struct CategoryFormState {
    pub name: tui_input::Input,
    pub description: tui_input::Input,
    pub focused_field: usize,
    pub error: Option<String>,
    pub editing_id: Option<CategoryId>,
}

#[derive(Debug, Clone, Default)]
pub struct WarehouseFormState {
    pub name: tui_input::Input,
    pub location: tui_input::Input,
    pub focused_field: usize,
    pub error: Option<String>,
    pub editing_id: Option<WarehouseId>,
}

#[derive(Debug, Clone, Default)]
pub struct ItemDetailState {
    pub adjust_input: tui_input::Input,
    pub kind: AdjustKind,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Copy)]
pub enum PendingDelete {
    Item(ItemId),
    Category(CategoryId),
    Warehouse(WarehouseId),
}

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
    New,
    Delete,
    Edit,
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
        m.insert(KeyBinding::plain(KeyCode::Char('n')), Action::New);
        m.insert(KeyBinding::plain(KeyCode::Char('d')), Action::Delete);
        m.insert(KeyBinding::plain(KeyCode::Delete), Action::Delete);
        m.insert(KeyBinding::plain(KeyCode::Char('e')), Action::Edit);
        m.insert(KeyBinding::plain(KeyCode::Char('y')), Action::Confirm);
        m.insert(KeyBinding::plain(KeyCode::Char('/')), Action::Search);
        m.insert(KeyBinding::plain(KeyCode::Esc), Action::Cancel);
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
    AddItem(Box<ItemFormState>),
    ItemDetail(ItemId),

    CategoryList,
    AddCategory(Box<CategoryFormState>),
    CategoryDetail(CategoryId),

    WarehouseList,
    AddWarehouse(Box<WarehouseFormState>),
    WarehouseDetail(WarehouseId),

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

#[derive(Debug, Clone)]
pub enum Message {
    Tick,
    KeyPressed(KeyEvent),

    ItemsLoaded(Vec<ItemWithStock>),
    ItemSaved(ItemWithStock),
    ItemUpdated(ItemWithStock),
    ItemDeleted(ItemId),

    CategoriesLoaded(Vec<Category>),
    CategorySaved(Category),
    CategoryUpdated(Category),
    CategoryDeleted(CategoryId),

    WarehousesLoaded(Vec<Warehouse>),
    WarehouseSaved(Warehouse),
    WarehouseUpdated(Warehouse),
    WarehouseDeleted(WarehouseId),

    StockUpdated(ItemId, u32),
    MovementsLoaded(Vec<StockMovementRecord>),

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
    DeleteCategory(CategoryId),

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
    pub items: Vec<ItemWithStock>,
    pub categories: Vec<Category>,
    pub warehouses: Vec<Warehouse>,
    pub item_selected: usize,
    pub category_selected: usize,
    pub warehouse_selected: usize,
    pub selected_warehouse: Option<WarehouseId>,
    pub input_mode: InputMode,
    pub status: Option<String>,
    pub should_quit: bool,
    pub movement_page: Vec<StockMovementRecord>,
    pub item_detail: ItemDetailState,
    pub pending_delete: Option<PendingDelete>,
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
            item_selected: 0,
            category_selected: 0,
            warehouse_selected: 0,
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
        self.item_selected = 0;
    }

    pub fn update(&mut self, msg: Message) -> Option<Command> {
        match msg {
            Message::Tick => None,
            Message::KeyPressed(key) => self.handle_key(key),
            Message::ItemsLoaded(items) => {
                self.items = items;
                self.recompute_filter();
                self.item_selected = self.item_selected.min(self.filtered.len().saturating_sub(1));
                None
            }
            Message::ItemSaved(v) | Message::ItemUpdated(v) => {
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
                self.item_selected = self.item_selected.min(self.filtered.len().saturating_sub(1));
                self.screen = Screen::ItemList;
                self.input_mode = InputMode::Normal;
                self.status = Some("Item deleted".into());
                None
            }
            Message::CategoriesLoaded(categories) => {
                self.categories = categories;
                self.category_selected =
                    self.category_selected.min(self.categories.len().saturating_sub(1));
                None
            }
            Message::CategorySaved(c) | Message::CategoryUpdated(c) => {
                if let Some(existing) = self.categories.iter_mut().find(|x| x.id == c.id) {
                    *existing = c;
                } else {
                    self.categories.push(c);
                }
                self.screen = Screen::CategoryList;
                self.input_mode = InputMode::Normal;
                None
            }
            Message::CategoryDeleted(id) => {
                self.categories.retain(|c| c.id != id);
                self.category_selected =
                    self.category_selected.min(self.categories.len().saturating_sub(1));
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
                self.warehouse_selected =
                    self.warehouse_selected.min(self.warehouses.len().saturating_sub(1));
                None
            }
            Message::WarehouseSaved(w) | Message::WarehouseUpdated(w) => {
                if let Some(existing) = self.warehouses.iter_mut().find(|x| x.id == w.id) {
                    *existing = w;
                } else {
                    self.warehouses.push(w);
                }
                self.screen = Screen::WarehouseList;
                self.input_mode = InputMode::Normal;
                None
            }
            Message::WarehouseDeleted(id) => {
                self.warehouses.retain(|w| w.id != id);
                if self.selected_warehouse == Some(id) {
                    self.selected_warehouse = self.warehouses.first().map(|w| w.id);
                }
                self.warehouse_selected =
                    self.warehouse_selected.min(self.warehouses.len().saturating_sub(1));
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
                match &mut self.screen {
                    Screen::AddItem(form) => form.error = Some(err.clone()),
                    Screen::AddCategory(form) => form.error = Some(err.clone()),
                    Screen::AddWarehouse(form) => form.error = Some(err.clone()),
                    Screen::ItemDetail(_) => self.item_detail.error = Some(err.clone()),
                    _ => {}
                }
                self.status = Some(err);
                None
            }
            Message::None => None,
        }
    }
    pub fn handle_key(&mut self, key: KeyEvent) -> Option<Command> {
        use crossterm::event::KeyCode;

        // Global screen-switching, available from Normal mode anywhere. Kept out of
        // KeyMap deliberately — these are navigation shortcuts, not remappable actions.
        if self.input_mode == InputMode::Normal {
            match key.code {
                KeyCode::Char('1') => {
                    self.screen = Screen::Dashboard;
                    return None;
                }
                KeyCode::Char('2') => {
                    self.screen = Screen::ItemList;
                    return None;
                }
                KeyCode::Char('3') => {
                    self.screen = Screen::CategoryList;
                    return None;
                }
                KeyCode::Char('4') => {
                    self.screen = Screen::WarehouseList;
                    return None;
                }
                KeyCode::Char('5') => {
                    self.screen = Screen::StockMovementLog;
                    return Some(Command::FetchMovements { item_id: None, limit: 20, offset: 0 });
                }
                KeyCode::Char('6') => {
                    self.screen = Screen::Settings;
                    return None;
                }
                _ => {}
            }
        }

        match &self.input_mode {
            InputMode::Normal => self.handle_key_normal(key),
            InputMode::Editing => self.handle_key_editing(key),
            InputMode::Searching => self.handle_key_searching(key),
            InputMode::ConfirmingDelete => self.handle_key_confirming_delete(key),
        }
    }
    fn handle_key_normal(&mut self, key: KeyEvent) -> Option<Command> {
        let action = self.keymap.resolve(key)?;
        match (action, &self.screen) {
            (Action::Quit, _) => {
                self.should_quit = true;
                None
            }

            // --- Item list ---
            (Action::MoveDown, Screen::ItemList) => {
                self.item_selected =
                    self.item_selected.saturating_add(1).min(self.filtered.len().saturating_sub(1));
                None
            }
            (Action::MoveUp, Screen::ItemList) => {
                self.item_selected = self.item_selected.saturating_sub(1);
                None
            }
            (Action::Search, Screen::ItemList) => {
                self.input_mode = InputMode::Searching;
                None
            }
            (Action::Select, Screen::ItemList) => {
                let idx = self.filtered.get(self.item_selected).copied()?;
                let id = self.items.get(idx)?.item.id;
                self.screen = Screen::ItemDetail(id);
                self.item_detail = ItemDetailState::default();
                Some(Command::FetchMovements { item_id: Some(id), limit: 20, offset: 0 })
            }
            (Action::New, Screen::ItemList) => {
                self.screen = Screen::AddItem(Box::default());
                self.input_mode = InputMode::Editing;
                None
            }

            // --- Item detail ---
            (Action::Delete, Screen::ItemDetail(id)) => {
                self.pending_delete = Some(PendingDelete::Item(*id));
                self.input_mode = InputMode::ConfirmingDelete;
                None
            }
            (Action::Edit, Screen::ItemDetail(id)) => {
                let entry = self.items.iter().find(|e| e.item.id == *id)?;
                let mut form = ItemFormState {
                    sku: tui_input::Input::new(entry.item.sku.0.clone()),
                    name: tui_input::Input::new(entry.item.name.clone()),
                    unit_cost: tui_input::Input::new(entry.item.unit_cost.0.to_string()),
                    editing_id: Some(*id),
                    ..Default::default()
                };
                form.category_index = self
                    .categories
                    .iter()
                    .position(|c| c.id == entry.item.category_id)
                    .unwrap_or(0);
                self.screen = Screen::AddItem(Box::new(form));
                self.input_mode = InputMode::Editing;
                None
            }
            (Action::Cancel, Screen::ItemDetail(_)) => {
                self.screen = Screen::ItemList;
                None
            }

            // --- Category list ---
            (Action::MoveDown, Screen::CategoryList) => {
                self.category_selected = self
                    .category_selected
                    .saturating_add(1)
                    .min(self.categories.len().saturating_sub(1));
                None
            }
            (Action::MoveUp, Screen::CategoryList) => {
                self.category_selected = self.category_selected.saturating_sub(1);
                None
            }
            (Action::Select, Screen::CategoryList) => {
                let c = self.categories.get(self.category_selected)?;
                self.screen = Screen::CategoryDetail(c.id);
                None
            }
            (Action::New, Screen::CategoryList) => {
                self.screen = Screen::AddCategory(Box::default());
                self.input_mode = InputMode::Editing;
                None
            }

            // --- Category detail ---
            (Action::Delete, Screen::CategoryDetail(id)) => {
                self.pending_delete = Some(PendingDelete::Category(*id));
                self.input_mode = InputMode::ConfirmingDelete;
                None
            }
            (Action::Edit, Screen::CategoryDetail(id)) => {
                let c = self.categories.iter().find(|c| c.id == *id)?;
                let form = CategoryFormState {
                    name: tui_input::Input::new(c.name.0.clone()),
                    description: tui_input::Input::new(c.description.clone().unwrap_or_default()),
                    editing_id: Some(*id),
                    ..Default::default()
                };
                self.screen = Screen::AddCategory(Box::new(form));
                self.input_mode = InputMode::Editing;
                None
            }
            (Action::Cancel, Screen::CategoryDetail(_)) => {
                self.screen = Screen::CategoryList;
                None
            }

            // --- Warehouse list ---
            (Action::MoveDown, Screen::WarehouseList) => {
                self.warehouse_selected = self
                    .warehouse_selected
                    .saturating_add(1)
                    .min(self.warehouses.len().saturating_sub(1));
                None
            }
            (Action::MoveUp, Screen::WarehouseList) => {
                self.warehouse_selected = self.warehouse_selected.saturating_sub(1);
                None
            }
            (Action::Select, Screen::WarehouseList) => {
                let w = self.warehouses.get(self.warehouse_selected)?;
                self.screen = Screen::WarehouseDetail(w.id);
                None
            }
            (Action::New, Screen::WarehouseList) => {
                self.screen = Screen::AddWarehouse(Box::default());
                self.input_mode = InputMode::Editing;
                None
            }

            // --- Warehouse detail ---
            (Action::Delete, Screen::WarehouseDetail(id)) => {
                self.pending_delete = Some(PendingDelete::Warehouse(*id));
                self.input_mode = InputMode::ConfirmingDelete;
                None
            }
            (Action::Edit, Screen::WarehouseDetail(id)) => {
                let w = self.warehouses.iter().find(|w| w.id == *id)?;
                let form = WarehouseFormState {
                    name: tui_input::Input::new(w.name.0.clone()),
                    location: tui_input::Input::new(w.location.clone().unwrap_or_default()),
                    editing_id: Some(*id),
                    ..Default::default()
                };
                self.screen = Screen::AddWarehouse(Box::new(form));
                self.input_mode = InputMode::Editing;
                None
            }
            (Action::Cancel, Screen::WarehouseDetail(_)) => {
                self.screen = Screen::WarehouseList;
                None
            }
            _ => None,
        }
    }
    fn handle_key_editing(&mut self, key: KeyEvent) -> Option<Command> {
        use crossterm::event::KeyCode;

        match &self.screen {
            Screen::ItemDetail(id) => return self.handle_key_item_detail(key, *id),
            Screen::AddItem(_) => return self.handle_key_add_item(key),
            Screen::AddCategory(_) => return self.handle_key_add_category(key),
            Screen::AddWarehouse(_) => return self.handle_key_add_warehouse(key),
            _ => {}
        }

        if key.code == KeyCode::Esc {
            self.input_mode = InputMode::Normal;
        }
        None
    }
    fn handle_key_add_item(&mut self, key: KeyEvent) -> Option<Command> {
        use crossterm::event::KeyCode;
        match key.code {
            KeyCode::Esc => {
                self.input_mode = InputMode::Normal;
                self.screen = Screen::ItemList;
                None
            }
            KeyCode::Tab => {
                if let Screen::AddItem(form) = &mut self.screen {
                    if form.focused_field == 1 && form.sku.value().trim().is_empty() {
                        let suggestion = stash_core::sku::Sku::suggest_from_name(form.name.value());
                        form.sku = tui_input::Input::new(suggestion);
                    }
                    form.focused_field = (form.focused_field + 1) % 4;
                }
                None
            }
            KeyCode::BackTab => {
                if let Screen::AddItem(form) = &mut self.screen {
                    form.focused_field = form.focused_field.checked_sub(1).unwrap_or(3);
                }
                None
            }
            KeyCode::Left | KeyCode::Right if matches!(&self.screen, Screen::AddItem(f) if f.focused_field == 3) =>
            {
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
            KeyCode::Enter => {
                if let Screen::AddItem(form) = &mut self.screen {
                    if form.focused_field < 3 {
                        form.focused_field += 1;
                        return None;
                    }
                }
                self.try_submit_add_item_form()
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
            KeyCode::Enter => self.try_submit_stock_adjustment(item_id),
            _ => {
                self.item_detail.adjust_input.handle_event(&crossterm::event::Event::Key(key));
                None
            }
        }
    }
    fn handle_key_add_category(&mut self, key: KeyEvent) -> Option<Command> {
        use crossterm::event::KeyCode;
        match key.code {
            KeyCode::Esc => {
                self.input_mode = InputMode::Normal;
                self.screen = Screen::CategoryList;
                None
            }
            KeyCode::Tab | KeyCode::Down => {
                if let Screen::AddCategory(form) = &mut self.screen {
                    form.focused_field = (form.focused_field + 1) % 2;
                }
                None
            }
            KeyCode::BackTab | KeyCode::Up => {
                if let Screen::AddCategory(form) = &mut self.screen {
                    form.focused_field = form.focused_field.checked_sub(1).unwrap_or(1);
                }
                None
            }
            KeyCode::Enter => {
                if let Screen::AddCategory(form) = &mut self.screen {
                    if form.focused_field == 0 {
                        form.focused_field = 1;
                        return None;
                    }
                }
                self.try_submit_category_form()
            }
            _ => {
                if let Screen::AddCategory(form) = &mut self.screen {
                    let field = if form.focused_field == 0 {
                        &mut form.name
                    } else {
                        &mut form.description
                    };
                    field.handle_event(&crossterm::event::Event::Key(key));
                }
                None
            }
        }
    }
    fn handle_key_add_warehouse(&mut self, key: KeyEvent) -> Option<Command> {
        use crossterm::event::KeyCode;
        match key.code {
            KeyCode::Esc => {
                self.input_mode = InputMode::Normal;
                self.screen = Screen::WarehouseList;
                None
            }
            KeyCode::Tab | KeyCode::Down => {
                if let Screen::AddWarehouse(form) = &mut self.screen {
                    form.focused_field = (form.focused_field + 1) % 2;
                }
                None
            }
            KeyCode::BackTab | KeyCode::Up => {
                if let Screen::AddWarehouse(form) = &mut self.screen {
                    form.focused_field = form.focused_field.checked_sub(1).unwrap_or(1);
                }
                None
            }
            KeyCode::Enter => {
                if let Screen::AddWarehouse(form) = &mut self.screen {
                    if form.focused_field == 0 {
                        form.focused_field = 1;
                        return None;
                    }
                }
                self.try_submit_warehouse_form()
            }
            _ => {
                if let Screen::AddWarehouse(form) = &mut self.screen {
                    let field =
                        if form.focused_field == 0 { &mut form.name } else { &mut form.location };
                    field.handle_event(&crossterm::event::Event::Key(key));
                }
                None
            }
        }
    }
    fn handle_key_searching(&mut self, key: KeyEvent) -> Option<Command> {
        use crossterm::event::{Event, KeyCode};
        match key.code {
            KeyCode::Esc => {
                self.input_mode = InputMode::Normal;
                self.search_input = tui_input::Input::default();
                self.recompute_filter();
            }
            KeyCode::Enter => self.input_mode = InputMode::Normal,
            _ => {
                self.search_input.handle_event(&Event::Key(key));
                self.recompute_filter();
            }
        }
        None
    }
    fn handle_key_confirming_delete(&mut self, key: KeyEvent) -> Option<Command> {
        use crossterm::event::KeyCode;
        match key.code {
            KeyCode::Char('y') => {
                self.input_mode = InputMode::Normal;
                match self.pending_delete.take()? {
                    PendingDelete::Item(id) => Some(Command::DeleteItem(id)),
                    PendingDelete::Category(id) => Some(Command::DeleteCategory(id)),
                    PendingDelete::Warehouse(id) => Some(Command::DeleteWarehouse(id)),
                }
            }
            KeyCode::Char('n') | KeyCode::Esc => {
                self.input_mode = InputMode::Normal;
                self.pending_delete = None;
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
    fn try_submit_category_form(&mut self) -> Option<Command> {
        let Screen::AddCategory(form) = &mut self.screen else { return None };
        form.error = None;

        let name = match stash_core::category::CategoryName::parse(form.name.value()) {
            Ok(n) => n,
            Err(e) => {
                form.error = Some(e.to_string());
                return None;
            }
        };
        let description = if form.description.value().trim().is_empty() {
            None
        } else {
            Some(form.description.value().to_string())
        };

        if let Some(id) = form.editing_id {
            return Some(Command::UpdateCategory(
                stash_storage::category_repository::UpdateCategoryInput {
                    id,
                    name: Some(name),
                    description,
                },
            ));
        }

        Some(Command::SaveCategory(stash_storage::category_repository::CreateCategoryInput {
            id: CategoryId::new(),
            name,
            description,
        }))
    }
    fn try_submit_warehouse_form(&mut self) -> Option<Command> {
        let Screen::AddWarehouse(form) = &mut self.screen else { return None };
        form.error = None;

        let name = match stash_core::warehouse::WarehouseName::parse(form.name.value()) {
            Ok(n) => n,
            Err(e) => {
                form.error = Some(e.to_string());
                return None;
            }
        };
        let location = if form.location.value().trim().is_empty() {
            None
        } else {
            Some(form.location.value().to_string())
        };

        if let Some(id) = form.editing_id {
            return Some(Command::UpdateWarehouse(
                stash_storage::warehouse_repository::UpdateWarehouseInput {
                    id,
                    name: Some(name),
                    location,
                },
            ));
        }

        Some(Command::SaveWarehouse(stash_storage::warehouse_repository::CreateWarehouseInput {
            id: WarehouseId::new(),
            name,
            location,
        }))
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
}
