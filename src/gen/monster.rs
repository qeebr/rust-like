extern crate rand;

use rand::Rng;
use super::super::character::entity::*;
use super::super::character::monster::*;
use super::super::character::item::*;
use super::super::character::stats::*;

pub fn create_monster(player: &Entity, mn_type: u32, diff: u32) -> Monster {
    let mut monster = Monster::new(MonsterType::Zombie, Difficulty::Easy, Entity::new());

    match mn_type {
        1 => {
            monster.entity.name = "Zombie".to_string();
            monster.monster_type = MonsterType::Zombie;
        },
        2 => {
            monster.entity.name = "Crab".to_string();
            monster.monster_type = MonsterType::Crab;
        },
        3 => {
            monster.entity.name = "Goblin".to_string();
            monster.monster_type = MonsterType::Goblin;
        },
        _ => panic!("unknown monster_type."),
    };

    let player_stats = player.calculate_stats();
    let player_damage = player.weapon.get_damage();
    let mean_damage = ((player_damage.0 + player_damage.1)/2) as f32;

    match diff {
        1 => {
            calculate_monster_stats(&mut monster, player_stats, mean_damage, 0.9f32, 0.9f32, 0.2f32);

            monster.monster_difficulty = Difficulty::Easy;
            monster.entity.name = "(Easy) ".to_string() + &monster.entity.name;
        },
        2 => {
            calculate_monster_stats(&mut monster, player_stats, mean_damage, 2.0f32, 1.0f32, 0.9f32);

            monster.monster_difficulty = Difficulty::Normal;
            monster.entity.name = "(Normal) ".to_string() + &monster.entity.name;
        },
        3 => {
            calculate_monster_stats(&mut monster, player_stats, mean_damage, 2.0f32, 1.2f32, 1.0f32);

            monster.monster_difficulty = Difficulty::Hard;
            monster.entity.name = "(Hard) ".to_string() + &monster.entity.name;
        },
        _ => panic!("unknown difficulty."),
    }

    monster.entity.current_life = monster.entity.calculate_max_life();

    let weapon_drop = rand::thread_rng().gen_range(0, 101);
    if weapon_drop <= 10 {
        let item_name = generate_random_weapon_name(Type::Weapon, &monster.monster_difficulty);
        let mut new_item = Item { name: item_name, item_type: Type::Weapon, modifications: Vec::new() };

        generate_item(&mut new_item, &player.weapon, &monster.monster_difficulty);

        match monster.entity.backpack.add_item(new_item) {
            _ => { /*I don't care.*/ },
        }
    }

    let head_drop = rand::thread_rng().gen_range(0, 101);
    if head_drop <= 10 {
        let item_name = generate_random_weapon_name(Type::Head, &monster.monster_difficulty);
        let mut new_item = Item { name: item_name, item_type: Type::Head, modifications: Vec::new() };

        generate_item(&mut new_item, &player.head_item, &monster.monster_difficulty);

        match monster.entity.backpack.add_item(new_item) {
            _ => { /*I don't care.*/ },
        }
    }

    let chest_drop = rand::thread_rng().gen_range(0, 101);
    if chest_drop <= 10 {
        let item_name = generate_random_weapon_name(Type::Chest, &monster.monster_difficulty);
        let mut new_item = Item { name: item_name, item_type: Type::Chest, modifications: Vec::new() };

        generate_item(&mut new_item, &player.chest_item, &monster.monster_difficulty);

        match monster.entity.backpack.add_item(new_item) {
            _ => { /*I don't care.*/ },
        }
    }

    let legs_drop = rand::thread_rng().gen_range(0, 101);
    if legs_drop <= 10 {
        let item_name = generate_random_weapon_name(Type::Legs, &monster.monster_difficulty);
        let mut new_item = Item { name: item_name, item_type: Type::Legs, modifications: Vec::new() };

        generate_item(&mut new_item, &player.leg_item, &monster.monster_difficulty);

        match monster.entity.backpack.add_item(new_item) {
            _ => { /*I don't care.*/ },
        }
    }

    let potion_drop = rand::thread_rng().gen_range(0, 101);
    if potion_drop <= 5 {
        let healing_percentage = match monster.monster_difficulty {
            Difficulty::Easy => 5,
            Difficulty::Normal => 10,
            Difficulty::Hard => 25,
        };
        let mut potion = Item { name: "Healing Potion".to_string(), item_type: Type::Potion, modifications: Vec::new() };

        potion.modifications.push(StatsMod::Heal(healing_percentage));

        match monster.entity.backpack.add_item(potion) {
            _ => { /*I don't care.*/ }
        }
    }

    monster
}

fn calculate_monster_stats(monster: &mut Monster, player_stats : Stats, mean_damage : f32, vitality :f32, strength : f32, defense : f32) {
    monster.entity.base_stats.vitality = (mean_damage * vitality).round() as i32;
    monster.entity.base_stats.defense = (player_stats.strength as f32 * defense).round() as i32;
    monster.entity.base_stats.strength = (player_stats.defense as f32 * strength).round() as i32;
    monster.entity.base_stats.speed = player_stats.speed;
}

fn generate_item(new_item: &mut Item, current_item: &Item, monster_difficulty: &Difficulty) {
    let difficulty_bonus = match monster_difficulty {
        &Difficulty::Easy => { 2 },
        &Difficulty::Normal => { 4 },
        &Difficulty::Hard => { 8 },
    };
    let mut first_value_range = match current_item.item_type {
        Type::Nothing => {
            6 + difficulty_bonus
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

            sum + difficulty_bonus
        }
    };

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
        let min_max_damage = current_item.get_damage();

        let mut rnd_min = rand::thread_rng().gen_range(min_max_damage.0, min_max_damage.0 + difficulty_bonus);
        let mut rnd_max = rand::thread_rng().gen_range(min_max_damage.1, min_max_damage.1 + difficulty_bonus);

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
}

fn generate_random_weapon_name(item_type: Type, difficulty: &Difficulty) -> String {
    let quality = match difficulty {
        &Difficulty::Easy => "Lesser",
        &Difficulty::Normal => "Good",
        &Difficulty::Hard => "Master",
    }.to_string();

    let part = match item_type {
        Type::Head => "Helm",
        Type::Chest => "Armor",
        Type::Legs => "Trousers",
        Type::Weapon => "Sword",
        Type::Nothing | Type::Potion => "Blackhole",
    }.to_string();

    return format!("{} {}", quality, part);
}