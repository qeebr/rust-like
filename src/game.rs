use super::character::entity::*;
use super::character::monster::*;
use super::character::item::*;
use super::level::*;
use super::gen::level::*;
use super::gen::monster::*;
use super::ui::*;
use super::combat::effect::*;
use super::combat::fight::*;
use super::log::*;
use super::ki::*;

/*
Was kann ich verbessern:

    Generierung
    * Drop hat atm. nur positive oder neutrale auswirkung aber keine negativen. Lesser -> [Viel Negativ; Neutral+1] Good -> [Negativ; Neutral + 2], Master -> [Neutral: Positiv]
    * (1) Seed der Maps fest machen,
    * Alle 10 oder 20 Level/Monster ein Boss-Monster einfügen, das richtig BÄM macht -> Krasseren Loot droppt, -> den Zusatz aus Master rausnehmen und nur für diese Klasse von Items verwenden.
    * Die Stats der Items ebenfalls in KLassen einteilen, das Helme immer weniger haben wie Chests und Chests am meisten und Legs am wenigsten oder so.
    * Genierung von Monster ist atm zu unflexibel und großartig falsch.

    UI
    * (1) Anzeige wievieltes Levels man atm. ist.
    * Die einzelnen Fenster für Loot und bla überschneiden sich, String ausgabe finden die um chars verschiebt -> Anzeige Karte blendet in die Spieler anzeige.
    * Atm. nur ein Monster-Symbol.
    * Zucker anzeigen, ob Item besser ist.

    Game
    * Spezial-Attacken einfügen.
    * Monster-Generierung balancieren.
*/

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
    let mut enemy_loot_index: usize = 0;

    Window::init();
    Window::draw(&mut log, &map, &player, &enemies, &effect_list);

    loop {
        let input = Window::get_input();

        let next_game_state = match game_state {
            Action::Game => {
                handle_game_state(&mut log, &map, &mut player, &mut enemies, &mut effect_list, &mut enemy_loot_index, input)
            },
            Action::Loot => {
                handle_loot_state(&mut log, &mut player, &mut enemies, &mut backpack_index, enemy_loot_index, input)
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
            Action::Menu => {
                handle_menu_state(input)
            }
            Action::Quit => {
                break;
            }
        };

        if game_state == Action::Game && next_game_state == Action::Loot {
            backpack_index = 0;
        }

        if next_game_state == Action::Quit {
            break;
        } else {
            game_state = next_game_state;
        }

        Window::draw(&mut log, &map, &player, &enemies, &effect_list);

        if game_state == Action::Loot {
            let enemy = &enemies[enemy_loot_index];

            Window::draw_loot(&enemy.entity.backpack, backpack_index, true, &enemy.entity.name)
        } else if game_state == Action::Inventory {
            Window::draw_loot(&player.backpack, backpack_index, inventory_pointer == InventoryPointer::Backpack, &"".to_string());
            Window::draw_entity(&player, character_pointer, inventory_pointer == InventoryPointer::Character);
        } else if game_state == Action::Menu {
            Window::draw_menu();
        }
    }

    Window::clear();
}

fn set_player_and_monsters(map: &Level, player: &mut Entity, enemies: &mut Vec<Monster>) {
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
    Menu,
    Quit,
}

fn handle_menu_state(input: Input) -> Action {
    match input {
        Input::Use => {
            Action::Quit
        },
        Input::Quit => {
            Action::Game
        }
        _ => { Action::Menu }
    }
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
                    let new_item: Item = player.backpack.items[*backpack_index].clone();

                    if new_item.item_type == Type::Potion {
                        let max_life = player.calculate_max_life();
                        let heal_percentage = new_item.get_heal_percentage();
                        let actual_heal = max_life / heal_percentage;

                        player.current_life = player.current_life + actual_heal;
                        if player.current_life > max_life {
                            player.current_life = max_life;
                        }

                        player.backpack.remove_item(*backpack_index);
                        log.add_message(format!("Player {} have been healed.", player.name));
                    } else {
                        let name_clone = new_item.name.clone();
                        player.backpack.remove_item(*backpack_index);
                        let old_item = player.equip(new_item);

                        if old_item.item_type != Type::Nothing {
                            player.backpack.insert_item(*backpack_index, old_item);
                        }

                        log.add_message(format!("Player {} equipped {}", player.name, name_clone));
                    }
                },
                Input::Drop => {
                    let new_item: Item = player.backpack.items[*backpack_index].clone();

                    if new_item.item_type != Type::Nothing {
                        player.backpack.remove_item(*backpack_index);
                        log.add_message(format!("Player {} dropped {}.", player.name, new_item.name));
                    }
                }

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

fn handle_loot_state(log: &mut Log, player: &mut Entity, enemies: &mut Vec<Monster>, backpack_index: &mut usize, enemy_loot_index : usize, input: Input) -> Action {
    match input {
        Input::MoveUp => {
            if *backpack_index > 0 {
                *backpack_index -= 1;
            }
        },
        Input::MoveDown => {
            if !enemies[enemy_loot_index].entity.backpack.empty_slot(*backpack_index + 1) {
                *backpack_index += 1;
            }
        },

        Input::Use => {
            if player.backpack.has_space() {
                let item = enemies[enemy_loot_index].entity.backpack.items[*backpack_index].clone();
                log.add_message(format!("Item {} added to Backpack", item.name));

                enemies[enemy_loot_index].entity.backpack.remove_item(*backpack_index);
                match player.backpack.add_item(item) {
                    Result::Err(..) => { panic!("Error") },
                    _ => {},
                }
            } else {
                log.add_message("Backpack ist full!".to_string());
            }
        },

        Input::Quit => { return Action::Game; },
        _ => {},
    }


    Action::Loot
}

fn handle_game_state(log: &mut Log, map: &Level, player: &mut Entity, enemies: &mut Vec<Monster>, effect_list: &mut Vec<WeaponAttack>, enemy_loot_index : &mut usize, input: Input) -> Action {
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
                    let enemy_with_loot = enemies.iter().position(|x| x.entity.backpack.size() > 0 && x.entity.pos_row == player.pos_row && x.entity.pos_col == player.pos_col);

                    match enemy_with_loot {
                        Option::Some(value) => {
                            *enemy_loot_index = value;
                            return Action::Loot;
                        },
                        _ => {
                            log.add_message("Nothing to loot here.".to_string());
                            return Action::Game;
                        }
                    }
                },
                _ => {
                    if map.meta[player.pos_row as usize][player.pos_col as usize] == Tile::Next {
                        return Action::NextLevel;
                    }
                    return Action::Inventory;
                },
            }
        },

        Input::Quit => { return Action::Menu },

        Input::Nothing | Input::Drop => {},
    }

    handle_ki(log, map, player, enemies, effect_list);

    Action::Game
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
