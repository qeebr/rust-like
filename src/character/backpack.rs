use super::item::*;

pub const BACKPACK_SIZE: usize = 20;

pub struct Backpack {
    pub items: Vec<Item>,
}

impl Backpack {
    pub fn new() -> Backpack {
        let items = vec![
            get_free(),
            get_free(),
            get_free(),
            get_free(),
            get_free(),
            get_free(),
            get_free(),
            get_free(),
            get_free(),
            get_free(),
            get_free(),
            get_free(),
            get_free(),
            get_free(),
            get_free(),
            get_free(),
            get_free(),
            get_free(),
            get_free(),
            get_free(),
        ];

        Backpack { items: items }
    }

    pub fn add_item(&mut self, new_item: Item) -> Result<(), Item> {
        let mut free_index = 0;

        for item in &self.items {
            match item.item_type {
                Type::Nothing => break,
                _ => (),
            }

            free_index += 1;
        };

        if free_index >= BACKPACK_SIZE {
            return Result::Err(new_item);
        }

        self.items[free_index] = new_item;

        Result::Ok(())
    }

    pub fn insert_item(&mut self, backpack_index : usize, old_item : Item) {
        self.items.insert(backpack_index, old_item);
        self.items.remove(BACKPACK_SIZE);
    }

    pub fn remove_item(&mut self, index: usize) {
        self.items.remove(index);
        self.items.push(get_free());
    }

    pub fn has_space(&self) -> bool {
        for item in &self.items {
            match item.item_type {
                Type::Nothing => return true,
                _ => (),
            }
        }

        false
    }

    pub fn empty_slot(&self, index: usize) -> bool {
        if index < BACKPACK_SIZE { // usize is always >= than zero.
            return self.items[index].item_type == Type::Nothing
        }

        false
    }

    pub fn size(&self) -> usize {
        let mut counter = 0 as usize;

        for index in 0 .. BACKPACK_SIZE {
            if self.empty_slot(index) {
                break;
            }

            counter+=1;
        }

        counter
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::item::*;

    #[test]
    fn test_backpack() {
        let mut backpack = Backpack::new();
        let add_index = BACKPACK_SIZE / 2;
        let special_name = "Magic Shirt";

        // At the beginnning Backpack is empty.
        assert_eq!(true, backpack.has_space());

        // Add Items to Backpack.
        for n in 0..BACKPACK_SIZE {
            let result = backpack.add_item(create_shorts(n));

            assert_good_result(result);
        }

        // Backpack is full.
        assert_eq!(false, backpack.has_space());

        let too_much = create_special(special_name.to_string());

        // Try to add something, it has to fail.
        let result = backpack.add_item(too_much);
        let too_much = assert_bad_result(result);

        // Remove item now there is space.
        backpack.remove_item(add_index);
        assert_eq!(true, backpack.has_space());

        // Add "old" item again, backpack is full again.
        let result = backpack.add_item(too_much);
        assert_good_result(result);
        assert_eq!(false, backpack.has_space());

        // New Item is in correct position.
        assert_eq!(special_name, backpack.items[BACKPACK_SIZE-1].name);
    }

    fn assert_good_result(result: Result<(), Item>) {
        match result {
            Result::Ok(_) => (),
            Result::Err(item) => panic!("Could not add Item {}, but should have", item.name),
        }
    }

    fn assert_bad_result(result: Result<(), Item>) -> Item {
        match result {
            Result::Ok(_) => panic!("Could add Item, but should not have"),
            Result::Err(item) => item,
        }
    }

    fn create_special(name: String) -> Item {
        Item { item_type: Type::Chest, name: name, modifications: Vec::new() }
    }

    fn create_shorts(n: usize) -> Item {
        Item { item_type: Type::Legs, name: format!("{} Shorts", n), modifications: Vec::new() }
    }
}