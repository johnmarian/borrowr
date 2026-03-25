use crate::domain::item::ItemId;

pub struct AddItemCommand {
    pub description: String,
}

pub struct UpdateItemCommand {
    pub id: ItemId,
    pub description: String,
}

pub struct RemoveItemCommand {
    pub id: ItemId,
}
