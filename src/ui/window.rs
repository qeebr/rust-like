extern crate ncurses;

use ncurses::*;
use super::super::level::level::*;
use super::super::character::entity::*;
use super::super::character::monster::*;
use super::super::character::backpack::*;
use super::super::character::item::*;
use super::super::character::stats::*;
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

    pub fn draw_item(item: &Item) {
        let item_offset_row = 0;
        let item_offset_col = 20;

        let mut row = item_offset_row;
        mvprintw(row as i32, item_offset_col, &item.name);
        row += 1;

        let type_str = match item.item_type {
            Type::Head => {
                "Head"
            },
            Type::Chest => {
                "Chest"
            },
            Type::Legs => {
                "Legs"
            },
            Type::Weapon => {
                "Weapon"
            },
            Type::Nothing => {
                "Nothing"
            }
        };
        mvprintw(row as i32, item_offset_col, &type_str);
        row += 1;

        for attribute in &item.modifications {
            match attribute {
                &StatsMod::Damage{min, max} => {
                    mvprintw(row as i32, item_offset_col, &format!("Damage {}-{}", min, max));
                },
                &StatsMod::AttackSpeed(val) => {
                    mvprintw(row as i32, item_offset_col, &format!("Speed {}", val));
                }
                &StatsMod::Add(val) => {
                    match val {
                        Stat::Defense(val) => {
                            mvprintw(row as i32, item_offset_col, &format!("Defense {}", val));
                        },
                        Stat::Speed(val) => {
                            mvprintw(row as i32, item_offset_col, &format!("Speed {}", val));
                        },
                        Stat::Strength(val) => {
                            mvprintw(row as i32, item_offset_col, &format!("Strength {}", val));
                        },
                        Stat::Vitality(val) => {
                            mvprintw(row as i32, item_offset_col, &format!("Vitality {}", val));
                        }
                    }
                }
            }
            row += 1;
        }
    }

    pub fn draw_loot(backpack: &Backpack, backpack_index: usize) {
        let loot_offset_row = 0;
        let loot_offset_col = 50;
        let display_row_count = 5;

        let mut items: Vec<&Item> = Vec::new();
        let mut item_count = 0;
        let start_index = display_row_count * (backpack_index / display_row_count);

        //Fill items vector with items to display.
        for index in start_index .. start_index + display_row_count {
            if !backpack.empty_slot(index) {
                item_count+=1;
                items.push(&backpack.items[index]);
            }
        }

        //Display items.
        let mut counter = 0;
        for item in items {
            mvprintw((counter + loot_offset_row) as i32, 1 + loot_offset_col, &item.name);
            counter += 1;
        }

        //Mark current items.
        mvaddch(((backpack_index % display_row_count) + loot_offset_row) as i32, loot_offset_col, resolve_item_cursor());

        //Fill empty spaces.
        while counter < display_row_count {
            mvprintw((counter + loot_offset_row) as i32, 1 + loot_offset_col, "Empty");
            counter += 1;
        }

        //Draw full item
        Window::draw_item(&backpack.items[backpack_index]);
    }

    pub fn draw(log: &mut Log, level: &Level, player: &Entity, enemies: &Vec<Monster>, effect_list: &Vec<WeaponAttack>) {
        clear();

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
    Use,

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

        97 => Input::AttackLeft,
        119 => Input::AttackUp,
        100 => Input::AttackRight,
        115 => Input::AttackDown,

        113 => Input::Quit, //113 is Q.
        101 => Input::Use, //101 is E.

        _ => Input::Nothing,
    }
}

fn resolve_item_cursor() -> u32 {
    '>' as u32
}

fn resolve_enemy(enemy: &Monster) -> u32 {
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
        &Tile::MnSpawn { .. } => '?' as u32,
    }
}
