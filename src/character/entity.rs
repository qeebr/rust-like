use super::stats::*;
use super::item::*;
use super::backpack::*;

pub struct Entity {
    pub id: u32,
    pub name: String,
    pub base_stats: Stats,

    pub head_item: Item,
    pub chest_item: Item,
    pub leg_item: Item,
    pub weapon: Item,

    pub backpack: Backpack,

    pub pos_row: i32,
    pub pos_col: i32,
    pub current_life: i32,

    pub monster_type: MonsterType,
    pub monster_difficulty: Difficulty,
}

pub enum MonsterType {
    Unknown,
    Crab,
    Zombie,
    Goblin,
}

#[derive(PartialEq)]
pub enum Difficulty {
    Easy,
    Normal,
    Hard,
}

impl Entity {
    pub fn new(id: u32) -> Entity {
        let base_stats = Stats { vitality: 20, strength: 10, speed: 5, defense: 10 };

        let mut entity = Entity {
            id: id,
            //No-Name.
            name: "Unknown".to_string(),

            //Base-Stats.
            base_stats: base_stats,

            //Items.
            head_item: get_free(), chest_item: get_free(), leg_item: get_free(), weapon: get_free(),

            //Position.
            pos_row: 0, pos_col: 0,

            // Life.
            current_life: 0,

            // Backpack.
            backpack: Backpack::new(),

            monster_type : MonsterType::Unknown,
            monster_difficulty : Difficulty::Easy,
        };

        //Set correct life.
        entity.current_life = entity.calculate_max_life();

        entity
    }


    pub fn calculate_stats(&self) -> Stats {
        let mut dep_stats = Stats { ..self.base_stats };

        add_item(&mut dep_stats, &self.head_item);
        add_item(&mut dep_stats, &self.chest_item);
        add_item(&mut dep_stats, &self.leg_item);
        add_item(&mut dep_stats, &self.weapon);

        dep_stats
    }

    // Game-Design dependent code.
    pub fn calculate_max_life(&self) -> i32 {
        let dep_stats = self.calculate_stats();

        dep_stats.vitality * 10
    }

    pub fn is_death(&self) -> bool {
        self.current_life <= 0
    }

    pub fn equip(&mut self, new_item: Item) -> Item {
        match &new_item.item_type {
            &Type::Head => {
                assert_item_is_clothing(&new_item);
                change_item(&mut self.head_item, new_item)
            },
            &Type::Chest => {
                assert_item_is_clothing(&new_item);
                change_item(&mut self.chest_item, new_item)
            },
            &Type::Legs => {
                assert_item_is_clothing(&new_item);
                change_item(&mut self.leg_item, new_item)
            },
            &Type::Weapon => {
                assert_item_is_weapon(&new_item);
                change_item(&mut self.weapon, new_item)
            },
            &Type::Nothing | &Type::Potion => {
                new_item
            }
        }
    }
}

fn change_item(entity: &mut Item, new_item: Item) -> Item {
    let old = entity.clone();
    *entity = new_item;
    old
}

fn add_item(mut base_stats: &mut Stats, item: &Item) {
    for modification in &item.modifications {
        match modification {
            &StatsMod::Add(ref value) => add_stat(&mut base_stats, value),
            _ => continue,
        };
    }
}

fn add_stat(base_stats: &mut Stats, stat: &Stat) {
    match stat {
        &Stat::Vitality(value) => base_stats.vitality += value,
        &Stat::Strength(value) => base_stats.strength += value,
        &Stat::Speed(value) => base_stats.speed += value,
        &Stat::Defense(value) => base_stats.defense += value,
    };
}

#[test]
fn test_change_item() {
    let mut player_entity = Entity::new();

    let mut attributes: Vec<StatsMod> = Vec::new();
    attributes.push(StatsMod::Add(Stat::Strength(5)));
    attributes.push(StatsMod::Add(Stat::Defense(10)));
    let new_item = Item { modifications: attributes, name: "Helm".to_string(), item_type: Type::Head };

    assert_eq!("Free", player_entity.head_item.name);
    assert_eq!("Helm", new_item.name);

    let old_item = change_item(&mut player_entity.head_item, new_item);

    assert_eq!("Helm", player_entity.head_item.name);
    assert_eq!("Free", old_item.name);
}

#[test]
fn test_add_stat() {
    let mut stats = Stats { vitality: 1, strength: 2, speed: 3, defense: 5 };
    let stat = Stat::Vitality(7);

    add_stat(&mut stats, &stat);

    assert_eq!(8, stats.vitality);
}