use crate::ids::{ItemId, MovementId, WarehouseId};
use crate::CoreError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StockMovement {
    Inbound { qty: u32, reason: String },
    Outbound { qty: u32, reason: String },
    Adjustment { delta: i32, reason: String },
}
impl StockMovement {
    /// The DB-friendly tag for the `kind` column.
    #[must_use]
    pub const fn kind_str(&self) -> &'static str {
        match self {
            Self::Inbound { .. } => "inbound",
            Self::Outbound { .. } => "outbound",
            Self::Adjustment { .. } => "adjustment",
        }
    }

    /// The signed delta for the `quantity_delta` column.
    /// Inbound is always positive, Outbound always negative, Adjustment keeps its sign.
    #[must_use]
    pub fn quantity_delta(&self) -> i64 {
        match self {
            Self::Inbound { qty, .. } => i64::from(*qty),
            Self::Outbound { qty, .. } => -i64::from(*qty),
            Self::Adjustment { delta, .. } => i64::from(*delta),
        }
    }

    #[must_use]
    pub fn reason(&self) -> &str {
        match self {
            Self::Inbound { reason, .. }
            | Self::Outbound { reason, .. }
            | Self::Adjustment { reason, .. } => reason,
        }
    }

    /// Reconstructs a movement from the persisted (`kind`, `qty_delta`, `reason`) columns.
    ///
    /// # Errors
    /// Returns `CoreError::InvalidMovementKind` if `kind` isn't a recognized tag, or
    /// `CoreError::QuantityOverflow` if the stored delta doesn't fit the expected sign/width.
    pub fn from_parts(
        kind: &str,
        qty_delta: i64,
        reason: Option<String>,
    ) -> Result<Self, CoreError> {
        let reason = reason.unwrap_or_default();
        match kind {
            "inbound" => {
                let qty = u32::try_from(qty_delta).map_err(|_| CoreError::QuantityOverflow)?;
                Ok(Self::Inbound { qty, reason })
            }
            "outbound" => {
                let qty = u32::try_from(-qty_delta).map_err(|_| CoreError::QuantityOverflow)?;
                Ok(Self::Outbound { qty, reason })
            }
            "adjustment" => {
                let delta = i32::try_from(qty_delta).map_err(|_| CoreError::QuantityOverflow)?;
                Ok(Self::Adjustment { delta, reason })
            }
            other => Err(CoreError::InvalidMovementKind(other.to_string())),
        }
    }
}

#[derive(Debug, Clone)]
pub struct StockMovementRecord {
    pub id: MovementId,
    pub item_id: ItemId,
    pub warehouse_id: WarehouseId,
    pub movement: StockMovement,
    pub created_at: time::OffsetDateTime,
}

#[derive(Clone)]
pub struct StockLevel {
    pub item_id: ItemId,
    pub warehouse_id: WarehouseId,
    pub quantity: u32,
}

impl StockLevel {
    #[must_use]
    pub const fn is_below_threshold(&self, threshold: u32) -> bool {
        self.quantity < threshold
    }

    /// Applies a stock movement (inbound, outbound, or adjustment) to the current stock level
    /// and computes the corresponding updated stock level.
    ///
    /// # Parameters
    /// - `movement`: A reference to a `StockMovement` instance, representing the type of stock
    ///   operation (inbound, outbound, or adjustment) to be applied.
    ///
    /// # Returns
    /// - `Ok(StockLevel)`: If the operation is successful, returns the updated `StockLevel` instance
    ///   with the new quantity.
    /// - `Err(CoreError)`: If the operation cannot be successfully performed, returns a `CoreError`
    ///   indicating the cause of the failure:
    ///     - `CoreError::QuantityOverflow`: When adding inbound quantity causes an overflow.
    ///     - `CoreError::InsufficientStock`: When an outbound or adjustment operation results
    ///       in a stock shortfall.
    ///
    /// # Stock Movement Types
    /// - `StockMovement::Inbound`: Represents an addition of stock. The quantity to be added
    ///   is added to the current stock level. If the resulting quantity exceeds the maximum
    ///   `u32` value, an error is returned.
    /// - `StockMovement::Outbound`: Represents a removal of stock. The quantity to be deducted
    ///   is subtracted from the current stock level. If the requested quantity exceeds the
    ///   available stock, an error is returned.
    /// - `StockMovement::Adjustment`: Represents an adjustment to the stock level (positive or
    ///   negative). The delta adjustment is applied to the current stock level. If the resulting
    ///   stock level becomes negative, an error is returned.
    ///
    /// # Errors
    /// This function returns an error under the following scenarios:
    /// - If an `Inbound` operation causes a numeric overflow.
    /// - If an `Outbound` operation attempts to withdraw more stock than is available.
    /// - If an `Adjustment` operation reduces the stock level below zero.
    ///
    pub fn apply(&self, movement: &StockMovement) -> Result<Self, CoreError> {
        let new_qty = match movement {
            StockMovement::Inbound { qty, .. } => {
                self.quantity.checked_add(*qty).ok_or(CoreError::QuantityOverflow)?
            }
            StockMovement::Outbound { qty, .. } => {
                self.quantity.checked_sub(*qty).ok_or(CoreError::InsufficientStock {
                    available: self.quantity,
                    requested: *qty,
                })?
            }
            StockMovement::Adjustment { delta, .. } => {
                let signed_qty = i64::from(self.quantity) + i64::from(*delta);
                if signed_qty < 0 {
                    return Err(CoreError::InsufficientStock {
                        available: self.quantity,
                        requested: delta.unsigned_abs(),
                    });
                }
                u32::try_from(signed_qty).map_err(|_| CoreError::InsufficientStock {
                    available: self.quantity,
                    requested: delta.unsigned_abs(),
                })?
            }
        };

        Ok(Self { quantity: new_qty, ..self.clone() })
    }
}
