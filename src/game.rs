use super::character::entity::*;
use super::character::monster::*;
use super::level::level::*;
use super::ui::window::*;
use super::combat::effect::*;
use super::combat::fight::*;
use super::log::*;

pub fn game() {
    let mut log = Log { messages: Vec::new() };
    let map = Level::new();
    let mut player = Entity::new();
    player.name = "qriz".to_string();

    let mut enemies: Vec<Monster> = Vec::new();
    let mut effect_list: Vec<WeaponAttack> = Vec::new();

    Window::init();

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

    Window::draw(&mut log, &map, &player, &enemies, &effect_list);

    loop {
        effect_list.clear();
        let input = Window::get_input();

        match input {
            Input::MoveUp | Input::MoveDown | Input::MoveLeft | Input::MoveRight => {
                handle_move(&map, &mut player, &enemies, input);
            },

            Input::AttackUp | Input::AttackDown | Input::AttackLeft | Input::AttackRight => {
                handle_attack(&mut log, &player, &mut enemies, &mut effect_list, input);
            },

            Input::Nothing => {},
            Input::Quit => break,
        }

        handle_ki(&mut log, &map, &mut player, &mut enemies, &mut effect_list);

        Window::draw(&mut log, &map, &player, &enemies, &effect_list);
    }

    Window::clear();
}

fn create_monster(player: &Entity, mn_type: u32, diff: u32) -> Monster {
    let mut enemy = Entity::new();
    enemy.name = "Zombie".to_string();
    enemy.pos_row = 5;
    enemy.pos_col = 5;
    enemy.base_stats.vitality = 1;
    enemy.current_life = enemy.calculate_max_life();

    Monster::new(MonsterType::Zombie, Difficulty::Easy, enemy)
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
