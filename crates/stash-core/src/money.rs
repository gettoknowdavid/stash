// Money is stored as integer KOBO, never as a float.
// Floats (f64) lose precision for currency — 0.1 + 0.2 != 0.3 in
// floating point. Kobo-as-i64 sidesteps that entirely.

#[derive(Clone)]
pub struct Money(i64);
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
