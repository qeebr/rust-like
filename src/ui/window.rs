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

    pub fn draw_menu() {
        let message = "Press Q to Return to Game".to_string();
        mvprintw(12, (85-message.len() as i32)/2, &message);
        let message = "Press E to Exit".to_string();
        mvprintw(13, (85-message.len() as i32)/2, &message);
    }

    pub fn draw_entity(player : &Entity, character_pointer: Type, active : bool) {
        let character_offset_row = 2;
        let character_offset_col = 0;

        mvprintw(character_offset_row + 0, character_offset_col, &format!(" Head:     {}", player.head_item.name));
        mvprintw(character_offset_row + 1, character_offset_col, &format!(" Chest:    {}", player.chest_item.name));
        mvprintw(character_offset_row + 2, character_offset_col, &format!(" Legs:     {}", player.leg_item.name));
        mvprintw(character_offset_row + 3, character_offset_col, &format!(" Weapon:   {}", player.weapon.name));
        mvprintw(character_offset_row + 4, character_offset_col, "----------------");

        let stats = player.calculate_stats();
        mvprintw(character_offset_row + 5, character_offset_col, &format!(" Vitality: {}", stats.vitality));
        mvprintw(character_offset_row + 6, character_offset_col, &format!(" Strength: {}", stats.strength));
        mvprintw(character_offset_row + 7, character_offset_col, &format!(" Defense:  {}", stats.defense));
        mvprintw(character_offset_row + 8, character_offset_col, &format!(" Speed:    {}", stats.speed));

        if active {
            match character_pointer {
                Type::Head => {
                    mvaddch((character_offset_row + 0) as i32, character_offset_col, resolve_item_cursor());
                    Window::draw_item(&player.head_item);
                },
                Type::Chest => {
                    mvaddch((character_offset_row + 1) as i32, character_offset_col, resolve_item_cursor());
                    Window::draw_item(&player.chest_item);

                },
                Type::Legs => {
                    mvaddch((character_offset_row + 2) as i32, character_offset_col, resolve_item_cursor());
                    Window::draw_item(&player.leg_item);

                },
                Type::Weapon => {
                    mvaddch((character_offset_row + 3) as i32, character_offset_col, resolve_item_cursor());
                    Window::draw_item(&player.weapon);
                },
                _ => {},
            }
        }
    }

    pub fn draw_item(item: &Item) {
        let item_offset_row = 1;
        let item_offset_col = 27;

        let mut row = item_offset_row;
        mvprintw(row as i32, item_offset_col, &item.name);
        row += 1;

        let type_str = resolve_type(item.item_type);
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
                &StatsMod::Heal(val) => {
                    mvprintw(row as i32, item_offset_col, &format!("Heals {}%", val));
                }
            }
            row += 1;
        }
    }

    pub fn draw_loot(backpack: &Backpack, backpack_index: usize, active : bool, name : &String) {
        let mut loot_offset_row = 1;
        let loot_offset_col = 54;
        let display_row_count = 5;

        let mut items: Vec<&Item> = Vec::new();
        let start_index = display_row_count * (backpack_index / display_row_count);

        //Fill items vector with items to display.
        for index in start_index .. start_index + display_row_count {
            if !backpack.empty_slot(index) {
                items.push(&backpack.items[index]);
            }
        }

        //Display name
        if name.len() > 0 {
            mvprintw(loot_offset_row as i32, loot_offset_col, &name);
            loot_offset_row += 1;
        }

        //Display items.
        let mut counter = 0;
        for item in items {
            mvprintw((counter + loot_offset_row) as i32, 1 + loot_offset_col, &item.name);
            counter += 1;
        }

        if active {
            //Mark current items.
            mvaddch(((backpack_index % display_row_count) + loot_offset_row) as i32, loot_offset_col, resolve_item_cursor());
        }

        //Fill empty spaces.
        while counter < display_row_count {
            mvprintw((counter + loot_offset_row) as i32, 1 + loot_offset_col, "Empty");
            counter += 1;
        }

        if active {
            //Draw full item
            Window::draw_item(&backpack.items[backpack_index]);
        }
    }

    pub fn draw(log: &mut Log, level: &Level, player: &Entity, enemies: &Vec<Monster>, effect_list: &Vec<WeaponAttack>) {
        clear();

        //Draw Map.
        let mut row_index : usize = 0;
        for row in &level.level {
            let mut col_index : usize = 0;

            for col in row {
                mv(row_index as i32, col_index as i32);

                match &level.meta[row_index][col_index] {
                    &Tile::PlSpawn | &Tile::Next => {
                        addch(resolve_tile(&level.meta[row_index][col_index]));
                    },
                    _ => {
                        addch(resolve_tile(col));
                    }
                }

                col_index += 1;
            }

            row_index += 1;
        }

        //Draw Enemies.
        for enemy in enemies {
            mv(enemy.entity.pos_row, enemy.entity.pos_col);
            addch(resolve_enemy(enemy));
        }

        //Draw Player.
        mv(player.pos_row, player.pos_col);
        addch(resolve_player());

        //Draw Effects.
        for effect in effect_list {
            for &(row, col) in &effect.area {
                if &level.level[row as usize][col as usize] != &Tile::Wall {
                    mv(row, col);
                    addch(resolve_effect());
                }
            }
        }

        //Draw latest game message.
        let msg = log.get_message();
        match msg {
            Option::Some(val) => {
                mvprintw(LINES() - 1, 0, &val);
            },
            Option::None => {
                mvprintw(LINES() - 1, 0, "                                      ");
            },
        }

        //Draw Player Health and Name.
        mvaddch(0,0, '[' as u32);
        let health = ((player.current_life as f32 / player.calculate_max_life() as f32) * 100.0f32) as u32;
        if health >= 10 {
            mvaddch(0,1, '#' as u32);
        }
        if health >= 20 {
            mvaddch(0,2, '#' as u32);
        }
        if health >= 30 {
            mvaddch(0,3, '#' as u32);
        }
        if health >= 40 {
            mvaddch(0,4, '#' as u32);
        }
        if health >= 50 {
            mvaddch(0,5, '#' as u32);
        }
        if health >= 60 {
            mvaddch(0,6, '#' as u32);
        }
        if health >= 70 {
            mvaddch(0,7, '#' as u32);
        }
        if health >= 80 {
            mvaddch(0,8, '#' as u32);
        }
        if health >= 90 {
            mvaddch(0,9, '#' as u32);
        }
        if health >= 100 {
            mvaddch(0,10, '#' as u32);
        }
        mvaddch(0,11,']' as u32);
        mvprintw(0, 13, &player.name);
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
    Drop,

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
        114 => Input::Drop, //114 is R.

        _ => Input::Nothing,
    }
}

fn resolve_type(item_type : Type) -> &'static str {
    match item_type {
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
        Type::Potion => {
            "Potion"
        },
        Type::Nothing => {
            "Nothing"
        }
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
        &Tile::PlSpawn => '<' as u32,
        &Tile::MnSpawn { .. } => '?' as u32,
        &Tile::Next => '>' as u32,
    }
}
