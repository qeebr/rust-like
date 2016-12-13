extern crate ncurses;

use ncurses::*;
use super::super::level::level::*;
use super::super::character::entity::*;
use super::super::character::monster::*;
use super::super::combat::effect::*;
use super::super::log::*;

pub struct Window;

impl Window {
    pub fn init() {
        initscr();
        raw();//cbreak();
        halfdelay(5);
        keypad(stdscr(), true);
        noecho();


        curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    }

    pub fn draw(log: &mut Log, level: &Level, player: &Entity, enemies: &Vec<Monster>, effect_list: &Vec<WeaponAttack>) {
        /* Get the screen bounds. */
        let mut max_x = 0;
        let mut max_y = 0;
        getmaxyx(stdscr(), &mut max_y, &mut max_x);

        let mut row_index = 0;
        for row in &level.level {
            let mut col_index = 0;

            for col in row {
                mv(row_index, col_index);
                addch(resolve_tile(col));

                col_index += 1;
            }

            row_index += 1;
        }

        for enemy in enemies {
            mv(enemy.entity.pos_row, enemy.entity.pos_col);
            addch(resolve_enemy(enemy));
        }

        mv(player.pos_row, player.pos_col);
        addch(resolve_player());

        for effect in effect_list {
            for &(row, col) in &effect.area {
                if &level.level[row as usize][col as usize] != &Tile::Wall {
                    mv(row, col);
                    addch(resolve_effect());
                }
            }
        }

        let msg = log.get_message();
        match msg {
            Option::Some(val) => {
                mvprintw(LINES() - 1, 0, &val);
            },
            Option::None => {
                mvprintw(LINES() - 1, 0, "                                      ");
            },
        }
    }

    pub fn get_input() -> Input {
        resolve_input(getch())
    }

    pub fn clear() {
        getch();
        endwin();
    }
}

pub enum Input {
    Nothing,
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
    Quit,

    AttackUp,
    AttackDown,
    AttackLeft,
    AttackRight,
}

fn resolve_input(input: i32) -> Input {
    //a start 97 than alphabet.
    match input {
        KEY_LEFT => Input::MoveLeft,
        KEY_RIGHT => Input::MoveRight,
        KEY_UP => Input::MoveUp,
        KEY_DOWN => Input::MoveDown,
        113 => Input::Quit, //113 is Q.

        97 => Input::AttackLeft,
        119 => Input::AttackUp,
        100 => Input::AttackRight,
        115 => Input::AttackDown,

        _ => Input::Nothing,
    }
}

fn resolve_enemy(enemy: &Monster) -> u32{
    if enemy.entity.is_death() {
        '_' as u32
    } else {
        '|' as u32
    }
}

fn resolve_effect() -> u32 {
    '-' as u32
}

fn resolve_player() -> u32 {
    '@' as u32
}

fn resolve_tile(tile: &Tile) -> u32 {
    match tile {
        &Tile::Floor => '.' as u32,
        &Tile::Wall => '#' as u32,
        &Tile::Nothing => ' ' as u32,
        &Tile::PlSpawn => '!' as u32,
        &Tile::MnSpawn{..} => '?' as u32,
    }
}
