extern crate rand;

use rand::Rng;
use super::super::character::entity::*;
use super::super::character::item::*;
use super::super::character::stats::*;
use super::item::*;

pub fn create_monster(player: &Entity, mut monster: &mut Entity, mn_type: u32, diff: u32) {
    match mn_type {
        1 => {
            monster.name = "Zombie".to_string();
            monster.monster_type = MonsterType::Zombie;
        },
        2 => {
            monster.name = "Crab".to_string();
            monster.monster_type = MonsterType::Crab;
        },
        3 => {
            monster.name = "Goblin".to_string();
            monster.monster_type = MonsterType::Goblin;
        },
        4 => {
            monster.name = "Boss".to_string();
            monster.monster_type = MonsterType::Boss;
        }
        _ => panic!("unknown monster_type."),
    };

    let player_stats = player.calculate_stats();
    let player_damage = player.weapon.get_damage();
    let mean_damage = ((player_damage.0 + player_damage.1)/2) as f32;

    match diff {
        1 => {
            calculate_monster_stats(&mut monster, player_stats, mean_damage, 0.9f32, 0.9f32, 0.2f32);

            monster.monster_difficulty = Difficulty::Easy;
            monster.name = "(Easy) ".to_string() + &monster.name;
        },
        2 => {
            calculate_monster_stats(&mut monster, player_stats, mean_damage, 1.1f32, 1.0f32, 0.9f32);

            monster.monster_difficulty = Difficulty::Normal;
            monster.name = "(Normal) ".to_string() + &monster.name;
        },
        3 => {
            calculate_monster_stats(&mut monster, player_stats, mean_damage, 2.0f32, 1.2f32, 1.0f32);

            monster.monster_difficulty = Difficulty::Hard;
            monster.name = "(Hard) ".to_string() + &monster.name;
        },
        4 => {
            calculate_monster_stats(&mut monster, player_stats, mean_damage, 2.2f32, 1.4f32, 1.1f32);

            monster.monster_difficulty = Difficulty::Boss;
        }
        _ => panic!("unknown difficulty."),
    }

    monster.current_life = monster.calculate_max_life();

    let weapon_drop = rand::thread_rng().gen_range(0, 101);
    if weapon_drop <= 10 {
        let new_item = generate_item(Type::Weapon, &player.weapon, &monster.monster_difficulty);

        match monster.backpack.add_item(new_item) {
            _ => { /*I don't care.*/ },
        }
    }

    let head_drop = rand::thread_rng().gen_range(0, 101);
    if head_drop <= 10 {
        let new_item = generate_item(Type::Head, &player.head_item, &monster.monster_difficulty);

        match monster.backpack.add_item(new_item) {
            _ => { /*I don't care.*/ },
        }
    }

    let chest_drop = rand::thread_rng().gen_range(0, 101);
    if chest_drop <= 10 {
        let new_item = generate_item(Type::Chest, &player.chest_item, &monster.monster_difficulty);

        match monster.backpack.add_item(new_item) {
            _ => { /*I don't care.*/ },
        }
    }

    let legs_drop = rand::thread_rng().gen_range(0, 101);
    if legs_drop <= 10 {
        let new_item = generate_item(Type::Legs, &player.leg_item, &monster.monster_difficulty);

        match monster.backpack.add_item(new_item) {
            _ => { /*I don't care.*/ },
        }
    }

    let potion_drop = rand::thread_rng().gen_range(0, 101);
    if potion_drop <= 10 {
        let healing_percentage = match monster.monster_difficulty {
            Difficulty::Easy => 10,
            Difficulty::Normal => 25,
            Difficulty::Hard => 50,
            Difficulty::Boss => 100,
        };
        let mut potion = Item { name: "Healing Potion".to_string(), item_type: Type::Potion, modifications: Vec::new() };

        potion.modifications.push(StatsMod::Heal(healing_percentage));

        match monster.backpack.add_item(potion) {
            _ => { /*I don't care.*/ }
        }
    }
}

fn calculate_monster_stats(monster: &mut Entity, player_stats : Stats, mean_damage : f32, vitality :f32, strength : f32, defense : f32) {
    monster.base_stats.vitality = (mean_damage * vitality).round() as i32;
    monster.base_stats.defense = (player_stats.strength as f32 * defense).round() as i32;
    monster.base_stats.strength = (player_stats.defense as f32 * strength).round() as i32;
    monster.base_stats.speed = player_stats.speed;
}