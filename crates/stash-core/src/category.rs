#[derive(sqlx::FromRow)]
pub struct Category {
    pub id: crate::ids::CategoryId,
    pub name: String,
    pub description: Option<String>,
    pub created_at: time::OffsetDateTime,
}
