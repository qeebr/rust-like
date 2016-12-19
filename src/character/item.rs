use super::stats::Stat;

#[derive(Debug)]
pub struct Item {
    pub item_type: Type,
    pub name: String,
    pub modifications: Vec<StatsMod>,
}

impl Item {
    pub fn get_damage(&self) -> (i32, i32) {
        for modification in &self.modifications {
            match modification {
                &StatsMod::Damage { min, max } => return (min, max),
                _ => (),
            };
        }

        panic!("Method Item::get_damage should only be called on weapons!");
    }
}

impl Clone for Item {
    fn clone(&self) -> Self {
        let mut modifications = Vec::new();

        for modification in &self.modifications {
            modifications.push(modification.clone());
        }

        Item { item_type: self.item_type, name: self.name.clone(), modifications: modifications }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Type {
    Head,
    Chest,
    Legs,
    Weapon,
    Nothing,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum StatsMod {
    Add(Stat),
    Damage {
        min: i32,
        max: i32
    },
    AttackSpeed(i32),
}

pub fn get_fist() -> Item {
    let modifications: Vec<StatsMod> = vec!(StatsMod::Damage { min: 1, max: 5 }, StatsMod::AttackSpeed(1));
    Item { item_type: Type::Weapon, name: "Fists".to_string(), modifications: modifications }
}

pub fn get_free() -> Item {
    new_plain_item(Type::Nothing, "Free".to_string())
}

fn new_plain_item(item_type: Type, name: String) -> Item {
    Item { item_type: item_type, name: name, modifications: Vec::new() }
}

pub fn assert_item_is_clothing(given_item: &Item) {
    if !item_is_clothing(given_item) {
        panic!("Item {:?} is not clothing!");
    }
}

fn item_is_clothing(given_item: &Item) -> bool {
    match &given_item.item_type {
        &Type::Head | &Type::Chest | &Type::Legs => true,
        _ => false,
    }
}

pub fn assert_item_is_weapon(given_item: &Item) {
    if !item_is_weapon(given_item) {
        panic!("Item {:?} is not a Weapon!");
    }
}

fn item_is_weapon(given_item: &Item) -> bool {
    match &given_item.item_type {
        &Type::Weapon => (),
        _ => return false,
    };

    let mut has_damage = false;
    let mut has_attack_speed = false;

    for modification in &given_item.modifications {
        match modification {
            &StatsMod::Damage { .. } => has_damage = true,
            &StatsMod::AttackSpeed(..) => has_attack_speed = true,
            _ => (),
        }
    }

    return has_damage & has_attack_speed;
}

#[test]
fn test_clothing_check() {
    let legs = Item { item_type: Type::Legs, name: "Legs".to_string(), modifications: Vec::new() };

    assert_eq!(true, item_is_clothing(&legs));

    let modifications: Vec<StatsMod> = vec!(StatsMod::Damage { min: 1, max: 5 }, StatsMod::AttackSpeed(1));
    let weapon = Item { item_type: Type::Weapon, name: "Fists".to_string(), modifications: modifications };

    assert_eq!(false, item_is_clothing(&weapon));
}

#[test]
fn test_weapon_check() {
    let modifications: Vec<StatsMod> = vec!(StatsMod::Damage { min: 1, max: 5 }, StatsMod::AttackSpeed(1));
    let fists = Item { item_type: Type::Weapon, name: "Fists".to_string(), modifications: modifications };

    assert_eq!(true, item_is_weapon(&fists));

    let legs = Item { item_type: Type::Legs, name: "Legs".to_string(), modifications: Vec::new() };

    assert_eq!(false, item_is_weapon(&legs));
}

#[test]
fn test_malformed_weapon() {
    let modifications = vec![StatsMod::Add(Stat::Strength(5)), StatsMod::Damage { min: 10, max: 20 }];
    let error_weapon = Item { item_type: Type::Weapon, name: "ErrorWeapon".to_string(), modifications: modifications };

    assert_eq!(false, item_is_weapon(&error_weapon));
}
