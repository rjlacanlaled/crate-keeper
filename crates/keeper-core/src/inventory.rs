use std::collections::HashMap;

/// The `pub enum Error` is defining a custom error type for the inventory system. It lists different
/// possible errors that can occur when interacting with the inventory. These errors include:
pub enum Error {
    InventoryFull,
    ItemNotFound,
    ItemAlreadyExists,
    InvalidQuantity,
}

/// The `Item` struct is a generic type that represents an item with an id, name, quantity, and
/// properties.
///
/// Properties:
///
/// * `id`: A unique identifier for the item.
/// * `name`: The `name` property is of type `String`. It represents the name of the item.
/// * `quantity`: The `quantity` property is of type `u32`, which stands for unsigned 32-bit integer. It
/// represents the number of items of this type that are available.
/// * `properties`: The `properties` field is a `HashMap` that stores additional properties of type `T`
/// for each item. The keys of the `HashMap` are of type `String`, and the values are of type `T`. This
/// allows you to associate any type of property with each item in the `
pub struct Item<T> {
    pub id: String,
    pub name: String,
    pub quantity: u32,
    pub properties: HashMap<String, T>,
}

/// The `pub trait Inventory<T>` is defining a trait named `Inventory` that represents an inventory
/// system. The `T` is a generic type parameter that allows the trait to work with different types of
/// items.
pub trait Inventory<T> {
    /// The `fn add_item(&mut self, item: Item<T>) -> Result<(), Error>;` is a method defined in the
    /// `Inventory` trait. It takes ownership of an `Item<T>` and attempts to add it to the inventory.
    /// The `&mut self` parameter indicates that the method can modify the inventory object it is called
    /// on.
    fn add_item(&mut self, item: Item<T>) -> Result<(), Error>;

    /// The `fn update_item(&mut self, item: Item<T>) -> Result<(), Error>;` is a method defined in the
    /// `Inventory` trait. It takes ownership of an `Item<T>` and attempts to update the corresponding
    /// item in the inventory.
    fn update_item(&mut self, item: Item<T>) -> Result<(), Error>;

    /// The `delete_item` method is defined in the `Inventory` trait. It takes a mutable reference to
    /// `self` (the inventory object) and an `item_id` of type `&str` as parameters. It attempts to
    /// delete the item with the specified `item_id` from the inventory.
    fn delete_item(&mut self, item_id: &str) -> Result<(), Error>;

    /// The `get_item` method is defined in the `Inventory` trait. It takes a reference to `self` (the
    /// inventory object) and an `item_id` of type `&str` as parameters. It attempts to retrieve the
    /// item with the specified `item_id` from the inventory.
    fn get_item(&self, item_id: &str) -> Option<&Item<T>>;

    /// The `fn get_items(&self) -> Vec<&Item<T>>;` method is defined in the `Inventory` trait. It
    /// returns a vector of references to items (`&Item<T>`) that are currently in the inventory.
    fn get_items(&self) -> Vec<&Item<T>>;

    /// The `fn get_total_quantity(&self) -> u32;` method is defined in the `Inventory` trait. It
    /// returns the total quantity of all items in the inventory as an unsigned 32-bit integer (`u32`).
    /// This method allows you to retrieve the sum of the quantities of all items in the inventory.
    fn get_total_quantity(&self) -> u32;
}
