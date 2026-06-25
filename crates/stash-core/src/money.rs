// Money is stored as integer KOBO, never as a float.
// Floats (f64) lose precision for currency — 0.1 + 0.2 != 0.3 in
// floating point. Kobo-as-i64 sidesteps that entirely.

#[derive(Debug, Clone, serde::Serialize, sqlx::Type)]
#[sqlx(transparent)]
pub struct Money(pub i64);
impl Money {
    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    pub fn from_naira(naira: f64) -> Self {
        Self((naira * 100.0).round() as i64)
    }

    pub fn checked_add(&self, other: &Self) -> Option<Self> {
        self.0.checked_add(other.0).map(Money)
    }
}
impl From<i64> for Money {
    fn from(value: i64) -> Self {
        Self(value)
    }
}

impl From<Money> for i64 {
    fn from(money: Money) -> Self {
        money.0
    }
}
