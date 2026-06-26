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
    ItemDetail(stash_core::ids::ItemId),
    AddItem(Box<crate::ui::AddItemFormState>),
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
    ItemsLoaded(Vec<stash_core::item::Item>),
    ItemSaved(stash_core::item::Item),
    StockUpdated(stash_core::ids::ItemId, u32),
    Error(String),
}

#[derive(Debug, Clone)]
pub enum Command {
    FetchItems(stash_core::item::ItemFilter),
    SaveItem(stash_storage::repository::CreateItemInput),
    DeleteItem(stash_core::ids::ItemId),
    None,
}

#[derive(Debug, Clone)]
pub struct App {
    pub screen: Screen,
    pub items: Vec<stash_core::item::Item>,
    pub selected: usize,
    pub input_mode: InputMode,
    pub status: Option<String>,
    pub should_quit: bool,
    pub movement_page: Vec<stash_core::stock::StockMovement>,
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
            Message::ItemSaved(item) => {
                if let Some(existing) = self.items.iter_mut().find(|i| i.id == item.id) {
                    *existing = item;
                } else {
                    self.items.push(item);
                }
                self.screen = Screen::ItemList;
                None
            }
            Message::StockUpdated(_, _) => None,
            Message::Error(err) => {
                self.status = Some(err);
                None
            }
        }
    }

    pub fn handle_key(&mut self, key: crossterm::event::KeyEvent) -> Option<Command> {
        use crossterm::event::KeyCode;
        match (&self.input_mode, key.code) {
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
                if let Some(item) = self.items.get(self.selected) {
                    self.screen = Screen::ItemDetail(item.id);
                }
                None
            }
            (InputMode::Normal, KeyCode::Char('n')) => {
                self.screen = Screen::AddItem(Box::from(crate::ui::AddItemFormState::default()));
                self.input_mode = InputMode::Editing;
                None
            }
            (InputMode::Editing, KeyCode::Esc) => {
                self.input_mode = InputMode::Normal;
                self.screen = Screen::ItemList;
                None
            }
            _ => None,
        }
    }
}
