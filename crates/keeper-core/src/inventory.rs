use std::collections::HashMap;

pub enum Error {
    InventoryFull,
    ItemNotFound,
    ItemAlreadyExists,
    InvalidQuantity,
}

pub struct Item<T> {
    pub id: String,
    pub name: String,
    pub quantity: u32,
    pub properties: HashMap<String, T>,
}

pub trait Inventory<T> {
    fn add_item(&mut self, item: Item<T>) -> Result<(), Error>;
    fn update_item(&mut self, item: Item<T>) -> Result<(), Error>;
    fn delete_item(&mut self, item_id: &str) -> Result<(), Error>;
    fn get_item(&self, item_id: &str) -> Option<&Item<T>>;
    fn get_items(&self) -> Vec<&Item<T>>;
    fn get_total_quantity(&self) -> u32;
}
