use super::{BranchId, ItemId};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Item {
    pub id: ItemId,
    pub bib_id: String,
    pub barcode: String,
    pub status: ItemStatus,
    pub location: BranchId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ItemStatus {
    Available,
    CheckedOut,
    OnHoldShelf,
    Lost,
    InTransit,
}
