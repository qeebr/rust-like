extern crate rand;

use rand::Rng;
use super::super::character::item::*;
use super::super::character::stats::*;
use super::super::character::monster::*;

pub fn generate_item(target_type: Type, player_item: &Item, monster_difficulty: &Difficulty) -> Item {
    let mut item = Item { item_type: target_type, name: generate_random_weapon_name(&target_type, &monster_difficulty), modifications: Vec::new() };

    generate_item_attributes(&mut item, &player_item, &monster_difficulty);

    item
}

fn generate_random_weapon_name(item_type: &Type, difficulty: &Difficulty) -> String {
    let quality = match difficulty {
        &Difficulty::Easy => "Lesser",
        &Difficulty::Normal => "Good",
        &Difficulty::Hard => "Master",
    }.to_string();

    let part = match item_type {
        &Type::Head => "Helm",
        &Type::Chest => "Armor",
        &Type::Legs => "Trousers",
        &Type::Weapon => "Sword",
        &Type::Nothing | &Type::Potion => "Blackhole",
    }.to_string();

    return format!("{} {}", quality, part);
}

fn generate_item_attributes(new_item: &mut Item, current_item: &Item, monster_difficulty: &Difficulty) {
    let mut first_value_range = calculate_attribute_range(&current_item, &monster_difficulty);

    let value = rand::thread_rng().gen_range(0, first_value_range);

    if value > 0 {
        first_value_range -= value;

        new_item.modifications.push(StatsMod::Add(Stat::Strength(value)));
    }

    if first_value_range > 1 {
        let value = rand::thread_rng().gen_range(0, first_value_range);

        if value > 0 {
            first_value_range -= value;

            new_item.modifications.push(StatsMod::Add(Stat::Vitality(value)));
        }
    }

    if first_value_range > 1 {
        let value = rand::thread_rng().gen_range(0, first_value_range);

        if value > 0 {
            new_item.modifications.push(StatsMod::Add(Stat::Defense(value)));
        }
    }

    //SPECIAL-CASE WHEN WEAPON!
    if current_item.item_type == Type::Weapon ||
        (current_item.item_type == Type::Nothing && new_item.item_type == Type::Weapon) {

        calculate_weapon_attributes(new_item, current_item, monster_difficulty);
    }
}

fn calculate_weapon_attributes(new_item : &mut Item, current_item : &Item, monster_difficulty: &Difficulty) {
    let min_difficulty_bonus = calculate_min_difficulty_bonus(monster_difficulty);
    let max_difficulty_bonus = calculate_max_difficulty_bonus(monster_difficulty);

    let min_max_damage = current_item.get_damage();

    let mut rnd_min = rand::thread_rng().gen_range(min_max_damage.0 + min_difficulty_bonus, min_max_damage.0 + max_difficulty_bonus);
    let mut rnd_max = rand::thread_rng().gen_range(min_max_damage.1 + min_difficulty_bonus, min_max_damage.1 + max_difficulty_bonus);

    //Just make sure that min is <= than max.
    if rnd_min == rnd_max {
        rnd_max += 1;
    }
    if rnd_min > rnd_max {
        let tmp = rnd_min;
        rnd_min = rnd_max;
        rnd_max = tmp;
    }

    //Damage.
    new_item.modifications.push(StatsMod::Damage {
        min: rnd_min,
        max: rnd_max,
    });

    //Speed
    new_item.modifications.push(StatsMod::AttackSpeed(1));
}

fn calculate_attribute_range(current_item: &Item, monster_difficulty: &Difficulty) -> i32 {
    let min_difficulty_bonus = calculate_min_difficulty_bonus(monster_difficulty);
    let max_difficulty_bonus = calculate_max_difficulty_bonus(monster_difficulty);

    let current_bonus = match current_item.item_type {
        Type::Nothing => {
            0
        }
        _ => {
            let mut sum = 0;

            for stat_mod in &current_item.modifications {
                match stat_mod {
                    &StatsMod::Add(value) => {
                        match value {
                            Stat::Vitality(val) => {
                                sum += val;
                            },
                            Stat::Defense(val) => {
                                sum += val;
                            },
                            Stat::Strength(val) => {
                                sum += val;
                            },
                            _ => {},
                        }
                    },
                    _ => {},
                }
            }

            sum
        }
    };

    let attributes_count = rand::thread_rng().gen_range(current_bonus+min_difficulty_bonus, current_bonus+max_difficulty_bonus);

    if attributes_count <= 0 {
        1
    } else {
        attributes_count
    }
}

fn calculate_min_difficulty_bonus(monster_difficulty: &Difficulty) -> i32 {
    match monster_difficulty {
        &Difficulty::Easy => { -4 },
        &Difficulty::Normal => { -2 },
        &Difficulty::Hard => { 0 },
    }
}

fn calculate_max_difficulty_bonus(monster_difficulty: &Difficulty) -> i32 {
    match monster_difficulty {
        &Difficulty::Easy => { 3 },
        &Difficulty::Normal => { 5 },
        &Difficulty::Hard => { 9 },
    }
}