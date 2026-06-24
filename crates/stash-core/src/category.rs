#[derive(Clone)]
pub enum Category {
    Electronics,
    Perishable,
    Apparel,
    Other(String),
}
