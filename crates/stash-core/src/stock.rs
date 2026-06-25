use crate::ids::{ItemId, WarehouseId};
use crate::CoreError;

pub enum StockMovement {
    Inbound { qty: u32, reason: String },
    Outbound { qty: u32, reason: String },
    Adjustment { delta: i32, reason: String },
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

                // Not sure this is the right move here
                u32::try_from(signed_qty).map_err(|_| CoreError::InsufficientStock {
                    available: self.quantity,
                    requested: delta.unsigned_abs(),
                })?
            }
        };

        Ok(Self { quantity: new_qty, ..self.clone() })
    }
}
