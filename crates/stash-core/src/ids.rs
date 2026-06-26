use uuid::Uuid;

macro_rules! define_id {
    ($name:ident) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, sqlx::Type)]
        #[sqlx(transparent)]
        pub struct $name(pub Uuid);

        impl Default for $name {
            fn default() -> Self {
                Self(Uuid::new_v4())
            }
        }

        impl $name {
            #[must_use]
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl From<$name> for String {
            fn from(id: $name) -> Self {
                id.0.to_string()
            }
        }

        impl TryFrom<String> for $name {
            type Error = uuid::Error;
            fn try_from(s: String) -> Result<Self, Self::Error> {
                Ok(Self(Uuid::parse_str(&s)?))
            }
        }
    };
}

define_id!(ItemId);
define_id!(CategoryId);
define_id!(WarehouseId);
define_id!(MovementId);
