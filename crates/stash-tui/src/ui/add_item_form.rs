#[derive(Debug, Clone, Default)]
pub struct AddItemFormState {
    pub sku: tui_input::Input,
    pub name: tui_input::Input,
    pub unit_cost: tui_input::Input,
    pub focused_field: usize,
    pub error: Option<String>,
}
