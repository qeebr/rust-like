extern crate rand;
extern crate ncurses;

pub mod character;
pub mod combat;
pub mod level;
pub mod ui;
pub mod log;

use character::entity::*;
use level::level::*;
use ui::window::*;
use combat::effect::*;
use combat::fight::*;
use log::*;

fn main() {
    let mut log = Log { messages : Vec::new() };
    let map = Level::new();
    let mut player = Entity::new();
    player.name = "qriz".to_string();

    let mut enemies : Vec<Entity> = Vec::new();
    let mut effect_list : Vec<WeaponAttack> = Vec::new();

    let mut enemy = Entity::new();
    enemy.name = "Zombie".to_string();
    enemy.pos_row = 3;
    enemy.pos_col = 3;
    enemy.base_stats.vitality = 1;
    enemy.current_life = enemy.calculate_max_life();

    enemies.push(enemy);

    enemy.pos_row = 4;

    Window::init();

    let mut row_index = 0;
    for meta_row in &map.meta {
        let mut col_index = 0;
        for meta_col in meta_row {
            if meta_col == &Tile::PlSpawn {
                player.pos_row = row_index;
                player.pos_col = col_index;
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

        Window::draw(&mut log, &map, &player, &enemies, &effect_list);
    }

    Window::clear();
}

fn handle_attack(log : &mut Log, player : &Entity, enemies : &mut Vec<Entity>, effect_list : &mut Vec<WeaponAttack>, direction : Input) {
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
            if enemy.pos_row == row && enemy.pos_col == col {

                Fight::weapon_hit(log, RndGenerator, player, &mut enemy);
            }
        }
    }

    effect_list.push(attack);
}

fn handle_move(map: &Level, player: &mut Entity, enemies : &Vec<Entity>, direction: Input) {
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
        if !enemy.is_death() && row_diff == enemy.pos_row && col_diff == enemy.pos_col {
            return;
        }
    }

    player.pos_row = row_diff as i32;
    player.pos_col = col_diff as i32;
}
