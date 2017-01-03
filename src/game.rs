extern crate rand;
use rand::Rng;

use super::character::entity::*;
use super::character::monster::*;
use super::character::item::*;
use super::character::stats::*;
use super::level::level::*;
use super::level::map_gen::*;
use super::ui::window::*;
use super::combat::effect::*;
use super::combat::fight::*;
use super::log::*;

pub fn game() {
    let mut log = Log { messages: Vec::new() };
    let mut map = generate_level();
    let mut player = Entity::new();
    player.name = "qriz".to_string();

    let mut enemies: Vec<Monster> = Vec::new();
    let mut effect_list: Vec<WeaponAttack> = Vec::new();

    set_player_and_monsters(&map, &mut player, &mut enemies);

    let mut game_state = Action::Game;
    let mut backpack_index: usize = 0;
    let mut inventory_pointer = InventoryPointer::Backpack;
    let mut character_pointer = Type::Head;

    Window::init();
    Window::draw(&mut log, &map, &player, &enemies, &effect_list);

    loop {
        let input = Window::get_input();

        let next_game_state = match game_state {
            Action::Game => {
                handle_game_state(&mut log, &map, &mut player, &mut enemies, &mut effect_list, input)
            },
            Action::Loot => {
                handle_loot_state(&mut log, &mut player, &mut enemies, &mut backpack_index, input)
            }
            Action::Inventory => {
                handle_inventory_state(&mut log, &mut player, &mut inventory_pointer, &mut backpack_index, &mut character_pointer, input)
            }
            Action::NextLevel => {
                enemies.clear();
                map = generate_level();
                set_player_and_monsters(&map, &mut player, &mut enemies);
                Action::Game
            }
            Action::Quit => {
                break;
            }
        };

        if (game_state == Action::Game && next_game_state == Action::Loot) ||
            (game_state == Action::Game && next_game_state == Action::Inventory) {
            backpack_index = 0;
            inventory_pointer = InventoryPointer::Backpack;
        }

        if next_game_state == Action::Quit {
            break;
        } else {
            game_state = next_game_state;
        }

        Window::draw(&mut log, &map, &player, &enemies, &effect_list);

        if game_state == Action::Loot {
            let enemy = enemies.iter().find(|x| x.entity.pos_row == player.pos_row && x.entity.pos_col == player.pos_col).unwrap();

            Window::draw_loot(&enemy.entity.backpack, backpack_index, true)
        } else if game_state == Action::Inventory {
            Window::draw_loot(&player.backpack, backpack_index, inventory_pointer == InventoryPointer::Backpack);
            Window::draw_entity(&player, character_pointer, inventory_pointer == InventoryPointer::Character);
        }
    }

    Window::clear();
}

fn set_player_and_monsters(map : &Level, player : &mut Entity, enemies : &mut Vec<Monster>) {
    let mut row_index = 0;
    for meta_row in &map.meta {
        let mut col_index = 0;
        for meta_col in meta_row {
            match meta_col {
                &Tile::PlSpawn => {
                    player.pos_row = row_index;
                    player.pos_col = col_index;
                },
                &Tile::MnSpawn { mn_type, difficulty } => {
                    let mut monster = create_monster(&player, mn_type, difficulty);

                    monster.entity.pos_row = row_index;
                    monster.entity.pos_col = col_index;

                    enemies.push(monster);
                },
                _ => (),
            }

            col_index += 1;
        }

        row_index += 1;
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum InventoryPointer {
    Backpack,
    Character
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Action {
    Game,
    Loot,
    Inventory,
    NextLevel,
    Quit,
}

fn handle_inventory_state(log: &mut Log, player: &mut Entity, inventory_pointer: &mut InventoryPointer, backpack_index: &mut usize, character_pointer: &mut Type, input: Input) -> Action {
    match inventory_pointer {
        &mut InventoryPointer::Backpack => {
            match input {
                Input::MoveUp => {
                    if *backpack_index > 0 {
                        *backpack_index -= 1;
                    }
                },
                Input::MoveDown => {
                    if !player.backpack.empty_slot(*backpack_index + 1) {
                        *backpack_index += 1;
                    }
                },
                Input::Use => {
                    let new_item = player.backpack.items[*backpack_index].clone();
                    let name_clone = new_item.name.clone();
                    player.backpack.remove_item(*backpack_index);
                    let old_item = player.equip(new_item);

                    if old_item.item_type != Type::Nothing {
                        player.backpack.insert_item(*backpack_index, old_item);
                    }

                    log.add_message(format!("Player {} equipped {}", player.name, name_clone))
                },

                Input::MoveLeft => {
                    *inventory_pointer = InventoryPointer::Character;
                }

                Input::Quit => { return Action::Game },
                _ => {},
            };
        },
        &mut InventoryPointer::Character => {
            match input {
                Input::MoveUp => {
                    match character_pointer {
                        &mut Type::Chest => {
                            *character_pointer = Type::Head;
                        },
                        &mut Type::Legs => {
                            *character_pointer = Type::Chest;
                        },
                        &mut Type::Weapon => {
                            *character_pointer = Type::Legs;
                        },
                        _ => {},
                    }
                },
                Input::MoveDown => {
                    match character_pointer {
                        &mut Type::Head => {
                            *character_pointer = Type::Chest;
                        }
                        &mut Type::Chest => {
                            *character_pointer = Type::Legs;
                        },
                        &mut Type::Legs => {
                            *character_pointer = Type::Weapon;
                        },
                        _ => {},
                    }
                }

                Input::MoveRight => {
                    *inventory_pointer = InventoryPointer::Backpack;
                },

                Input::Quit => { return Action::Game },
                _ => {},
            }
        },
    }

    Action::Inventory
}

fn handle_loot_state(log: &mut Log, player: &mut Entity, enemies: &mut Vec<Monster>, backpack_index: &mut usize, input: Input) -> Action {
    let enemy_index = enemies.iter().position(|x| x.entity.pos_row == player.pos_row && x.entity.pos_col == player.pos_col).unwrap();


    match input {
        Input::MoveUp => {
            if *backpack_index > 0 {
                *backpack_index -= 1;
            }
        },
        Input::MoveDown => {
            if !enemies[enemy_index].entity.backpack.empty_slot(*backpack_index + 1) {
                *backpack_index += 1;
            }
        },

        Input::Use => {
            if player.backpack.has_space() {
                let item = enemies[enemy_index].entity.backpack.items[*backpack_index].clone();
                log.add_message(format!("Item {} added to Backpack", item.name));

                enemies[enemy_index].entity.backpack.remove_item(*backpack_index);
                match player.backpack.add_item(item) {
                    Result::Err(..) => {panic!("Error")},
                    _ => {},
                }
            } else {
                log.add_message("Backpack ist full!".to_string());
            }
        }

        Input::Quit => {
            return Action::Game;
        }
        _ => {},
    }


    Action::Loot
}

fn handle_game_state(log: &mut Log, map: &Level, player: &mut Entity, enemies: &mut Vec<Monster>, effect_list: &mut Vec<WeaponAttack>, input: Input) -> Action {
    effect_list.clear();

    match input {
        Input::MoveUp | Input::MoveDown | Input::MoveLeft | Input::MoveRight => {
            handle_move(map, player, enemies, input);
        },

        Input::AttackUp | Input::AttackDown | Input::AttackLeft | Input::AttackRight => {
            handle_attack(log, player, enemies, effect_list, input);
        },

        Input::Use => {
            let enemy = enemies.iter().find(|x| x.entity.pos_row == player.pos_row && x.entity.pos_col == player.pos_col);

            match enemy {
                Option::Some(..) => {
                    return Action::Loot
                },
                _ => {
                    if map.meta[player.pos_row as usize][player.pos_col as usize] == Tile::Next {
                        return Action::NextLevel;
                    }
                    return Action::Inventory;
                },
            }
        },

        Input::Quit => { return Action::Quit },

        Input::Nothing => {},
    }

    handle_ki(log, map, player, enemies, effect_list);

    Action::Game
}

fn create_monster(player: &Entity, mn_type: u32, diff: u32) -> Monster {
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

    match diff {
        1 => {
            monster.entity.base_stats.vitality = rand::thread_rng().gen_range(player_stats.vitality/10, player_stats.vitality/8 + 1);
            monster.entity.base_stats.defense = rand::thread_rng().gen_range(player_stats.vitality/10, player_stats.vitality/8 + 1);
            monster.entity.base_stats.speed = rand::thread_rng().gen_range(player_stats.vitality/10, player_stats.vitality/8 + 1);
            monster.entity.base_stats.strength = rand::thread_rng().gen_range(player_stats.vitality/10, player_stats.vitality/8 + 1);

            monster.monster_difficulty = Difficulty::Easy;
        },
        2 => {
            monster.entity.base_stats.vitality = rand::thread_rng().gen_range(player_stats.vitality/6, player_stats.vitality/4 + 1);
            monster.entity.base_stats.defense = rand::thread_rng().gen_range(player_stats.vitality/6, player_stats.vitality/4 + 1);
            monster.entity.base_stats.speed = rand::thread_rng().gen_range(player_stats.vitality/6, player_stats.vitality/4 + 1);
            monster.entity.base_stats.strength = rand::thread_rng().gen_range(player_stats.vitality/6, player_stats.vitality/4 + 1);

            monster.monster_difficulty = Difficulty::Easy;
        },
        3 => {
            monster.entity.base_stats.vitality = rand::thread_rng().gen_range(player_stats.vitality/3, player_stats.vitality/2 + 1);
            monster.entity.base_stats.defense = rand::thread_rng().gen_range(player_stats.vitality/3, player_stats.vitality/2 + 1);
            monster.entity.base_stats.speed = rand::thread_rng().gen_range(player_stats.vitality/3, player_stats.vitality/2 + 1);
            monster.entity.base_stats.strength = rand::thread_rng().gen_range(player_stats.vitality/3, player_stats.vitality/2 + 1);

            monster.monster_difficulty = Difficulty::Easy;
        },
        _ => panic!("unknown difficulty."),
    }

    monster.entity.current_life = monster.entity.calculate_max_life();

    let weapon_drop = rand::thread_rng().gen_range(0, 101);
    if weapon_drop <= 10 {
        let mut new_item = Item {name : generate_random_weapon_name(), item_type : Type::Weapon, modifications : Vec::new()};

        generate_item(&mut new_item, &player.weapon, &monster.monster_difficulty);

        match monster.entity.backpack.add_item(new_item) {
            _ => {/*I don't care.*/},
        }
    }

    let head_drop = rand::thread_rng().gen_range(0, 101);
    if head_drop <= 10 {
        let mut new_item = Item {name : generate_random_weapon_name(), item_type : Type::Head, modifications : Vec::new()};

        generate_item(&mut new_item, &player.head_item, &monster.monster_difficulty);

        match monster.entity.backpack.add_item(new_item) {
            _ => {/*I don't care.*/},
        }
    }

    let chest_drop = rand::thread_rng().gen_range(0, 101);
    if chest_drop <= 10 {
        let mut new_item = Item {name : generate_random_weapon_name(), item_type : Type::Chest, modifications : Vec::new()};

        generate_item(&mut new_item, &player.chest_item, &monster.monster_difficulty);

        match monster.entity.backpack.add_item(new_item) {
            _ => {/*I don't care.*/},
        }
    }

    let legs_drop = rand::thread_rng().gen_range(0, 101);
    if legs_drop <= 10 {
        let mut new_item = Item {name : generate_random_weapon_name(), item_type : Type::Legs, modifications : Vec::new()};

        generate_item(&mut new_item, &player.leg_item, &monster.monster_difficulty);

        match monster.entity.backpack.add_item(new_item) {
            _ => {/*I don't care.*/},
        }
    }

    monster
}

fn generate_item(new_item : &mut Item, current_item : &Item, monster_difficulty : &Difficulty) {
    let difficulty_bonus = match monster_difficulty {
        &Difficulty::Easy => { 1 },
        &Difficulty::Normal => { 3 },
        &Difficulty::Hard => { 7 },
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
        (current_item.item_type == Type::Nothing && new_item.item_type == Type::Weapon){
        let min_max_damage = match current_item.item_type {
            Type::Nothing => {
                (1, 5)
            },
            _ => {current_item.get_damage()},
        };

        //Damage.
        new_item.modifications.push(StatsMod::Damage {
            min: rand::thread_rng().gen_range(min_max_damage.0, min_max_damage.0 + difficulty_bonus),
            max: rand::thread_rng().gen_range(min_max_damage.1, min_max_damage.1 + difficulty_bonus)
        });

        //Speed
        new_item.modifications.push(StatsMod::AttackSpeed(1));
    }
}

fn generate_random_weapon_name() -> String {
    "Foobar".to_string()
}

fn handle_ki(log: &mut Log, map: &Level, player: &mut Entity, enemies: &mut Vec<Monster>, effect_list: &mut Vec<WeaponAttack>) {
    let size = enemies.len();
    for index in 0..size {
        if enemies[index].entity.is_death() {
            continue;
        }

        let row_diff = player.pos_row - enemies[index].entity.pos_row;
        let col_diff = player.pos_col - enemies[index].entity.pos_col;

        let distance = ((row_diff * row_diff + col_diff * col_diff) as f32).sqrt();

        //GameCode!
        if distance <= 1f32 {
            let direction = if row_diff == 1 {
                AttackDirection::South
            } else if row_diff == -1 {
                AttackDirection::North
            } else if col_diff == 1 {
                AttackDirection::East
            } else if col_diff == -1 {
                AttackDirection::West
            } else {
                unreachable!();
            };

            let attack = WeaponAttack::new(&enemies[index].entity, direction);

            Fight::weapon_hit(log, RndGenerator, &enemies[index].entity, player);

            effect_list.push(attack);
        } else if distance <= 4f32 {
            let direction = if row_diff > 0 && col_diff > 0 {
                if row_diff > col_diff {
                    Input::MoveDown
                } else {
                    Input::MoveRight
                }
            } else if row_diff > 0 && col_diff < 0 {
                if row_diff > -1 * col_diff {
                    Input::MoveDown
                } else {
                    Input::MoveLeft
                }
            } else if row_diff < 0 && col_diff < 0 {
                if row_diff < col_diff {
                    Input::MoveUp
                } else {
                    Input::MoveLeft
                }
            } else if row_diff < 0 && col_diff > 0 {
                if -1 * row_diff > col_diff {
                    Input::MoveUp
                } else {
                    Input::MoveRight
                }
            } else {
                Input::Nothing
            };

            let mut row_diff = enemies[index].entity.pos_row;
            let mut col_diff = enemies[index].entity.pos_col;

            match direction {
                Input::MoveUp => row_diff -= 1,
                Input::MoveDown => row_diff += 1,
                Input::MoveLeft => col_diff -= 1,
                Input::MoveRight => col_diff += 1,

                _ => { continue; },
            }

            //Collision with Wall uncool.
            if map.level[row_diff as usize][col_diff as usize] == Tile::Wall {
                continue;
            }

            if player.pos_row == row_diff && player.pos_col == col_diff {
                continue;
            }

            for inner_index in 0..size {
                if inner_index == index {
                    continue;
                }

                if !enemies[inner_index].entity.is_death() && row_diff == enemies[inner_index].entity.pos_row && col_diff == enemies[inner_index].entity.pos_col {
                    break;
                }
            }

            let mut mut_enemy = &mut enemies[index];
            mut_enemy.entity.pos_row = row_diff as i32;
            mut_enemy.entity.pos_col = col_diff as i32;
        }
    }
}

fn handle_attack(log: &mut Log, player: &Entity, enemies: &mut Vec<Monster>, effect_list: &mut Vec<WeaponAttack>, direction: Input) {
    let attack_direction = match direction {
        Input::AttackUp => AttackDirection::North,
        Input::AttackDown => AttackDirection::South,
        Input::AttackLeft => AttackDirection::West,
        Input::AttackRight => AttackDirection::East,
        _ => unreachable!(),
    };

    let attack = WeaponAttack::new(player, attack_direction);

    for mut enemy in enemies {
        for &(row, col) in &attack.area {
            if enemy.entity.pos_row == row && enemy.entity.pos_col == col {
                Fight::weapon_hit(log, RndGenerator, player, &mut enemy.entity);
            }
        }
    }

    effect_list.push(attack);
}

fn handle_move(map: &Level, player: &mut Entity, enemies: &Vec<Monster>, direction: Input) {
    let mut row_diff = player.pos_row;
    let mut col_diff = player.pos_col;

    match direction {
        Input::MoveUp => row_diff -= 1,
        Input::MoveDown => row_diff += 1,
        Input::MoveLeft => col_diff -= 1,
        Input::MoveRight => col_diff += 1,

        _ => unreachable!(),
    }

    //Collision with Wall uncool.
    if map.level[row_diff as usize][col_diff as usize] == Tile::Wall {
        return;
    }

    //Collision with alive entity uncool.
    for enemy in enemies {
        if !enemy.entity.is_death() && row_diff == enemy.entity.pos_row && col_diff == enemy.entity.pos_col {
            return;
        }
    }

    player.pos_row = row_diff as i32;
    player.pos_col = col_diff as i32;
}
