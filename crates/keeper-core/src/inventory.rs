/// The `Inventory` struct represents a collection of items.
///
/// Properties:
///
/// * `items`: The `items` property is a vector (dynamic array) of `Item` structs.
pub struct Inventory {
    items: Vec<Item>,
}

/// The `Item` struct represents an item with an ID, name, and quantity in Rust.
///
/// Properties:
///
/// * `id`: A unique identifier for the item. It is of type String, which means it can store any
/// sequence of characters.
/// * `name`: The `name` property of the `Item` struct is a `String` type. It represents the name of the
/// item.
/// * `quantity`: The `quantity` property is of type `u32`, which stands for "unsigned 32-bit integer".
/// It represents the number of items of a particular type that are available.
pub struct Item {
    pub id: String,
    pub name: String,
    pub quantity: u32,
}

/// The `impl Inventory { ... }` block is an implementation block for the `Inventory` struct. It
/// contains methods that define the behavior and functionality of the `Inventory` struct.
impl Inventory {
    /// The `new` function creates a new instance of the `Inventory` struct with an empty vector of items.
    ///
    /// Returns:
    ///
    /// The `new` function is returning an instance of the `Inventory` struct.
    pub fn new() -> Self {
        Inventory { items: Vec::new() }
    }

    /// The function `add_item` adds an item to a vector of items.
    ///
    /// Arguments:
    ///
    /// * `item`: The `item` parameter is of type `Item`.
    pub fn add_item(&mut self, item: Item) {
        self.items.push(item);
    }

    /// The `update_item` function in Rust is used to update an existing item.
    ///
    /// Arguments:
    ///
    /// * `item_id`: A string representing the ID of the item that needs to be updated.
    /// * `new_item`: The `new_item` parameter is of type `Item`. It represents the new item that you want
    /// to update the existing item with.
    pub fn update_item(&mut self, item_id: &str, new_item: Item) {
        // Implement logic to update existing item
        todo!()
    }

    /// The `delete_item` function is a placeholder that needs to be implemented to delete an item by its
    /// ID.
    ///
    /// Arguments:
    ///
    /// * `item_id`: A string representing the ID of the item to be deleted.
    pub fn delete_item(&mut self, item_id: &str) {
        // Implement logic to delete item by ID
        todo!()
    }

    /// The `get_total_quantity` function calculates the total quantity of items in a collection.
    ///
    /// Returns:
    ///
    /// The `get_total_quantity` function returns an unsigned 32-bit integer (u32).
    pub fn get_total_quantity(&self) -> u32 {
        self.items.iter().map(|item| item.quantity).sum()
    }

    /// The function `get_item_by_id` returns an `Option` containing a reference to an `Item` based on the
    /// provided `item_id`.
    ///
    /// Arguments:
    ///
    /// * `item_id`: A string representing the ID of the item you want to retrieve.
    ///
    /// Returns:
    ///
    /// an `Option` type, specifically `Option<&Item>`.
    pub fn get_item_by_id(&self, item_id: &str) -> Option<&Item> {
        // Implement logic to get item by ID
        todo!()
    }

    /// The function `get_items_by_name` returns a vector of references to `Item` objects that match a given
    /// name.
    ///
    /// Arguments:
    ///
    /// * `item_name`: A string representing the name of the item to search for.
    ///
    /// Returns:
    ///
    /// a vector of references to items (`&Item`).
    pub fn get_items_by_name(&self, item_name: &str) -> Vec<&Item> {
        // Implement logic to get items by name
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_add_item_successfully() {
        let mut inventory = Inventory::new();
        let item = Item { id: "1".to_string(), name: "Item 1".to_string(), quantity: 1 };
        inventory.add_item(item);
        assert_eq!(inventory.get_total_quantity(), 1);
    }

    #[test]
    fn it_should_update_item_successfully() {
        let mut inventory = Inventory::new();
        let item = Item { id: "1".to_string(), name: "Item 1".to_string(), quantity: 1 };
        inventory.add_item(item);
        let new_item = Item { id: "1".to_string(), name: "Item 1".to_string(), quantity: 2 };
        inventory.update_item("1", new_item);
        assert_eq!(inventory.get_total_quantity(), 2);
    }

    #[test]
    fn it_should_delete_item_successfully() {
        let mut inventory = Inventory::new();
        let item = Item { id: "1".to_string(), name: "Item 1".to_string(), quantity: 1 };
        inventory.add_item(item);
        inventory.delete_item("1");
        assert_eq!(inventory.get_total_quantity(), 0);
    }

    #[test]
    fn it_should_get_total_quantity_successfully() {
        let mut inventory = Inventory::new();
        let item1 = Item { id: "1".to_string(), name: "Item 1".to_string(), quantity: 1 };
        let item2 = Item { id: "2".to_string(), name: "Item 2".to_string(), quantity: 2 };
        inventory.add_item(item1);
        inventory.add_item(item2);
        assert_eq!(inventory.get_total_quantity(), 3);
    }
}
