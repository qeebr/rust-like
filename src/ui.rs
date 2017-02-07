extern crate ncurses;

use ncurses::*;
use super::level::*;
use super::character::entity::*;
use super::character::backpack::*;
use super::character::item::*;
use super::character::stats::*;
use super::log::*;

pub struct Window {
    player_window: WINDOW,
    map_window: WINDOW,
    status_window: WINDOW,
    backpack_window: WINDOW,
    character_window: WINDOW,
    item_window: WINDOW,
    menu_window: WINDOW,
}


impl Window {
    pub fn new() -> Window {
        Window {
            player_window: create_player_window(),
            map_window: create_map_window(),
            status_window: create_status_window(),
            backpack_window: create_backpack_window(),
            character_window: create_character_window(),
            item_window: create_item_window(),
            menu_window: create_menu_window(),
        }
    }

    pub fn init() {
        let locale_conf = LcCategory::all;
        setlocale(locale_conf, "UTF-8");

        initscr();
        raw();//cbreak();
        //halfdelay(5);
        keypad(stdscr(), true);
        noecho();

        curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    }

    pub fn draw_game_over(&mut self) {
        destroy_win(self.menu_window);
        self.menu_window = create_menu_window();

        let message = "Game Over".to_string();
        mvwprintw(self.menu_window, 1, (33 - message.len() as i32) / 2 + 1, &message);

        wrefresh(self.menu_window);
    }

    pub fn draw_menu(&mut self) {
        destroy_win(self.menu_window);
        self.menu_window = create_menu_window();

        let message = "Press Q to Return to Game".to_string();
        mvwprintw(self.menu_window, 1, (33 - message.len() as i32) / 2 +1, &message);
        let message = "Press E to Exit".to_string();
        mvwprintw(self.menu_window, 2, (33 - message.len() as i32) / 2 +1, &message);

        wrefresh(self.menu_window);
    }

    pub fn draw_entity(&mut self, player: &Entity, character_pointer: Type, active: bool) {
        destroy_win(self.character_window);
        self.character_window = create_character_window();

        let character_offset_row = 1;
        let character_offset_col = 1;

        mvwprintw(self.character_window, character_offset_row + 0, character_offset_col, &format!(" Head:     {}", player.head_item.name));
        mvwprintw(self.character_window, character_offset_row + 1, character_offset_col, &format!(" Chest:    {}", player.chest_item.name));
        mvwprintw(self.character_window, character_offset_row + 2, character_offset_col, &format!(" Legs:     {}", player.leg_item.name));
        mvwprintw(self.character_window, character_offset_row + 3, character_offset_col, &format!(" Weapon:   {}", player.weapon.name));
        mvwprintw(self.character_window, character_offset_row + 4, character_offset_col, "----------------");

        let stats = player.calculate_stats();
        let damage = player.weapon.get_damage();
        mvwprintw(self.character_window, character_offset_row + 5, character_offset_col, &format!(" Vitality: {}", stats.vitality));
        mvwprintw(self.character_window, character_offset_row + 6, character_offset_col, &format!(" Strength: {}", stats.strength));
        mvwprintw(self.character_window, character_offset_row + 7, character_offset_col, &format!(" Defense:  {}", stats.defense));
        mvwprintw(self.character_window, character_offset_row + 8, character_offset_col, &format!(" Speed:    {}", stats.speed));
        mvwprintw(self.character_window, character_offset_row + 9, character_offset_col, &format!(" Damage:   {}-{}", damage.0, damage.1));

        if active {
            match character_pointer {
                Type::Head => {
                    mvwaddch(self.character_window, (character_offset_row + 0) as i32, character_offset_col, resolve_item_cursor());
                    self.draw_item(&player.head_item);
                },
                Type::Chest => {
                    mvwaddch(self.character_window, (character_offset_row + 1) as i32, character_offset_col, resolve_item_cursor());
                    self.draw_item(&player.chest_item);
                },
                Type::Legs => {
                    mvwaddch(self.character_window, (character_offset_row + 2) as i32, character_offset_col, resolve_item_cursor());
                    self.draw_item(&player.leg_item);
                },
                Type::Weapon => {
                    mvwaddch(self.character_window, (character_offset_row + 3) as i32, character_offset_col, resolve_item_cursor());
                    self.draw_item(&player.weapon);
                },
                _ => {},
            }
        }

        wrefresh(self.character_window);
    }

    pub fn draw_item(&mut self, item: &Item) {
        destroy_win(self.item_window);
        self.item_window = create_item_window();

        let item_offset_row = 1;
        let item_offset_col = 1;

        let mut row = item_offset_row;
        mvwprintw(self.item_window, row as i32, item_offset_col, &item.name);
        row += 1;

        let type_str = resolve_type(item.item_type);
        mvwprintw(self.item_window, row as i32, item_offset_col, &type_str);
        row += 1;

        for attribute in &item.modifications {
            match attribute {
                &StatsMod::Damage { min, max } => {
                    mvwprintw(self.item_window, row as i32, item_offset_col, &format!("Damage {}-{}", min, max));
                },
                &StatsMod::AttackSpeed(val) => {
                    mvwprintw(self.item_window, row as i32, item_offset_col, &format!("Speed {}", val));
                }
                &StatsMod::Add(val) => {
                    match val {
                        Stat::Defense(val) => {
                            mvwprintw(self.item_window, row as i32, item_offset_col, &format!("Defense {}", val));
                        },
                        Stat::Speed(val) => {
                            mvwprintw(self.item_window, row as i32, item_offset_col, &format!("Speed {}", val));
                        },
                        Stat::Strength(val) => {
                            mvwprintw(self.item_window, row as i32, item_offset_col, &format!("Strength {}", val));
                        },
                        Stat::Vitality(val) => {
                            mvwprintw(self.item_window, row as i32, item_offset_col, &format!("Vitality {}", val));
                        }
                    }
                }
                &StatsMod::Heal(val) => {
                    mvwprintw(self.item_window, row as i32, item_offset_col, &format!("Heals {}%", val));
                }
            }
            row += 1;
        }

        wrefresh(self.item_window);
    }

    pub fn draw_loot(&mut self, backpack: &Backpack, backpack_index: usize, active: bool, name: &String) {
        destroy_win(self.backpack_window);
        self.backpack_window = create_backpack_window();

        let mut loot_offset_row = 1;
        let loot_offset_col = 2;
        let display_row_count = 5;

        let mut items: Vec<&Item> = Vec::new();
        let start_index = display_row_count * (backpack_index / display_row_count);

        //Fill items vector with items to display.
        for index in start_index..start_index + display_row_count {
            if !backpack.empty_slot(index) {
                items.push(&backpack.items[index]);
            }
        }

        //Display name
        if name.len() > 0 {
            mvwprintw(self.backpack_window, loot_offset_row as i32, loot_offset_col, &name);
            loot_offset_row += 1;
        }

        //Display items.
        let mut counter = 0;
        for item in items {
            mvwprintw(self.backpack_window, (counter + loot_offset_row) as i32, 1 + loot_offset_col, &item.name);
            counter += 1;
        }

        if active {
            //Mark current items.
            mvwaddch(self.backpack_window, ((backpack_index % display_row_count) + loot_offset_row) as i32, loot_offset_col, resolve_item_cursor());
        }

        //Fill empty spaces.
        while counter < display_row_count {
            mvwprintw(self.backpack_window, (counter + loot_offset_row) as i32, 1 + loot_offset_col, "Empty");
            counter += 1;
        }

        if active {
            //Draw full item
            self.draw_item(&backpack.items[backpack_index]);
        }

        wrefresh(self.backpack_window);
    }

    pub fn draw(&mut self, log: &mut Log, level: &Level, player: &Entity, enemies: &Vec<Entity>) {

        self.draw_player(player, level);
        self.draw_game_msg(log);
        self.draw_map(level, player, enemies);

    }

    pub fn draw_player(&mut self, player: &Entity, level: &Level) {
        destroy_win(self.player_window);
        self.player_window = create_player_window();

        mvwaddch(self.player_window, 1, 1, '[' as u32);
        let health = ((player.current_life as f32 / player.calculate_max_life() as f32) * 100.0f32) as u32;
        if health >= 10 {
            mvwaddch(self.player_window, 1, 2, '#' as u32);
        }
        if health >= 20 {
            mvwaddch(self.player_window, 1, 3, '#' as u32);
        }
        if health >= 30 {
            mvwaddch(self.player_window, 1, 4, '#' as u32);
        }
        if health >= 40 {
            mvwaddch(self.player_window, 1, 5, '#' as u32);
        }
        if health >= 50 {
            mvwaddch(self.player_window, 1, 6, '#' as u32);
        }
        if health >= 60 {
            mvwaddch(self.player_window, 1, 7, '#' as u32);
        }
        if health >= 70 {
            mvwaddch(self.player_window, 1, 8, '#' as u32);
        }
        if health >= 80 {
            mvwaddch(self.player_window, 1, 9, '#' as u32);
        }
        if health >= 90 {
            mvwaddch(self.player_window, 1, 10, '#' as u32);
        }
        if health >= 100 {
            mvwaddch(self.player_window, 1, 11, '#' as u32);
        }
        mvwaddch(self.player_window, 1, 12, ']' as u32);
        mvwprintw(self.player_window, 1, 14, &player.name);

        let x = getmaxx(self.player_window);
        let dungeon = format!("{} Dungeon", level.level);
        mvwaddstr(self.player_window, 1, x - (dungeon.len() + 1) as i32, &dungeon);

        wrefresh(self.player_window);
    }

    pub fn draw_map(&mut self, level: &Level, player: &Entity, enemies: &Vec<Entity>) {
        destroy_win(self.map_window);
        self.map_window = create_map_window();

        let offset = 1;

        //Draw Map.
        let mut row_index: usize = 0;
        for row in &level.map {
            let mut col_index: usize = 0;

            for col in row {
                match &level.meta[row_index][col_index] {
                    &Tile::PlSpawn | &Tile::Next => {
                        mvwaddch(self.map_window, row_index as i32 + offset, col_index as i32 + offset, resolve_tile(&level.meta[row_index][col_index]));
                    },
                    _ => {
                        mvwaddch(self.map_window, row_index as i32 + offset, col_index as i32 + offset, resolve_tile(col));
                    }
                }

                col_index += 1;
            }

            row_index += 1;
        }

        //Draw Enemies.
        let mut has_boss = false;
        for enemy in enemies {
            if enemy.monster_type == MonsterType::Boss {
                has_boss = true;
                continue;
            }

            mvwaddch(self.map_window, enemy.pos_row + offset, enemy.pos_col + offset, resolve_enemy(enemy));
        }

        //Draw Enemies with loot.
        let mut lootable_enemies_iter = enemies.iter().filter(|x| x.is_death() && x.backpack.size() > 0);

        loop {
            match lootable_enemies_iter.next() {
                Some(enemy) => {
                    if enemy.monster_type != MonsterType::Boss {
                        mvwaddch(self.map_window, enemy.pos_row + offset, enemy.pos_col + offset, resolve_enemy(enemy));
                    }
                },
                None => { break; }
            }
        }

        //Draw alive enemies, avoid that lootable enemy is over alive enemy.
        for enemy in enemies {
            if !enemy.is_death() && enemy.monster_type == MonsterType::Boss {
                mvwaddch(self.map_window, enemy.pos_row + offset, enemy.pos_col + offset, 'O' as u32);
                mvwaddch(self.map_window, enemy.pos_row - 1 + offset, enemy.pos_col + offset, 'o' as u32);
                mvwaddch(self.map_window, enemy.pos_row + offset, enemy.pos_col + 1 + offset, '-' as u32);
                mvwaddch(self.map_window, enemy.pos_row + offset, enemy.pos_col - 1 + offset, '-' as u32);
                mvwaddch(self.map_window, enemy.pos_row + 1 + offset, enemy.pos_col - 1 + offset, '/' as u32);
                mvwaddch(self.map_window, enemy.pos_row + 1 + offset, enemy.pos_col + 1 + offset, '\\' as u32);
            } else if !enemy.is_death() && enemy.monster_type != MonsterType::Boss {
                mvwaddch(self.map_window, enemy.pos_row + offset, enemy.pos_col + offset, resolve_enemy(enemy));
            }
        }

        //Draw Player.
        mvwaddch(self.map_window, player.pos_row + offset, player.pos_col + offset, resolve_player(player));
        //Draw Effects.
        /*for effect in effect_list {
            for &(row, col) in &effect.area {
                if &level.map[row as usize][col as usize] != &Tile::Wall {
                    mvwaddch(self.map_window, row+offset, col+offset, resolve_effect());
                }
            }
        }*/

        box_(self.map_window, 0, 0);
        wrefresh(self.map_window);
    }

    pub fn draw_game_msg(&mut self, log: &mut Log) {
        destroy_win(self.status_window);
        self.status_window = create_status_window();

        let msg = log.get_message();
        match msg {
            Option::Some(val) => {
                mvwaddstr(self.status_window, 1, 1, &val);
            },
            Option::None => {
                mvwaddstr(self.status_window, 1, 1, "                                      ");
            },
        }

        box_(self.status_window, 0, 0);
        wrefresh(self.status_window);
    }

    pub fn get_input() -> Input {
        resolve_input(getch())
    }

    pub fn clear() {
        endwin();
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Input {
    Nothing,

    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,

    Quit,
    Use,
    Drop,

    SpecialOne,
    SpecialTwo,
    SpecialThree,

    AttackUp,
    AttackDown,
    AttackLeft,
    AttackRight,
}

pub fn create_menu_window() -> WINDOW {
    create_windows(5, 35, 9, 25)
}

pub fn create_backpack_window() -> WINDOW {
    create_windows(8, 25, 5, 51)
}

pub fn create_character_window() -> WINDOW {
    create_windows(12, 25, 5, 1)
}

fn create_item_window() -> WINDOW {
    create_windows(7, 25, 5, 26)
}

fn create_status_window() -> WINDOW {
    create_windows(3, 80, 21, 0)
}

fn create_map_window() -> WINDOW {
    create_windows(20, 80, 2, 0)
}

fn create_player_window() -> WINDOW {
    create_windows(3, 80, 0, 0)
}

fn create_windows(height: i32, width: i32, start_row: i32, start_col: i32) -> WINDOW {
    let window = newwin(height, width, start_row, start_col);

    box_(window, 0, 0);

    wrefresh(window);

    window
}

fn destroy_win(window: WINDOW) {
    let borderless = ' ' as u32;

    wborder(window, borderless, borderless, borderless, borderless, borderless, borderless, borderless, borderless);
    wrefresh(window);
    delwin(window);
}

fn resolve_input(input: i32) -> Input {
    //a start 97 than alphabet.
    match input {
        KEY_LEFT => Input::MoveLeft,
        KEY_RIGHT => Input::MoveRight,
        KEY_UP => Input::MoveUp,
        KEY_DOWN => Input::MoveDown,

        49 => Input::SpecialOne,
        50 => Input::SpecialTwo,
        51 => Input::SpecialThree,

        97 => Input::AttackLeft, //97 is a
        119 => Input::AttackUp, //119 is w
        100 => Input::AttackRight, //100 is d
        115 => Input::AttackDown, //115 is s

        113 => Input::Quit, //113 is q.
        101 => Input::Use, //101 is e.
        114 => Input::Drop, //114 is r.

        _ => Input::Nothing,
    }
}

fn resolve_type(item_type: Type) -> &'static str {
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

fn resolve_enemy(enemy: &Entity) -> u32 {
    if enemy.is_death() && enemy.backpack.size() > 0 {
        'O' as u32
    } else if enemy.is_death() {
        '_' as u32
    } else {
        match enemy.monster_type {
            MonsterType::Unknown => {
                '?' as u32
            },
            MonsterType::Zombie => {
                match enemy.monster_difficulty {
                    Difficulty::Easy => {
                        'f' as u32
                    },
                    Difficulty::Normal => {
                        'F' as u32
                    },
                    Difficulty::Hard => {
                        'Ḟ' as u32
                    },
                    _ => {
                        unreachable!();
                    }
                }
            },
            MonsterType::Crab => {
                match enemy.monster_difficulty {
                    Difficulty::Easy => {
                        'm' as u32
                    },
                    Difficulty::Normal => {
                        'm' as u32
                    },
                    Difficulty::Hard => {
                        'Ṁ' as u32
                    },
                    _ => {
                        unreachable!();
                    }
                }
            },
            MonsterType::Goblin => {
                match enemy.monster_difficulty {
                    Difficulty::Easy => {
                        'x' as u32
                    },
                    Difficulty::Normal => {
                        'X' as u32
                    },
                    Difficulty::Hard => {
                        'Ẋ' as u32
                    },
                    _ => {
                        unreachable!();
                    }
                }
            }
            _ => {
                unreachable!();
            }
        }
    }
}

/*fn resolve_effect() -> u32 {
    '-' as u32
}*/

fn resolve_player(player: &Entity) -> u32 {
    if player.is_death() {
        '_' as u32
    } else {
        '@' as u32
    }
}

fn resolve_tile(tile: &Tile) -> u32 {
    match tile {
        &Tile::Floor => '.' as u32,
        &Tile::Wall => '#' as u32,
        &Tile::Nothing => ' ' as u32,
        &Tile::PlSpawn => '⋀' as u32,
        &Tile::MnSpawn { .. } => '?' as u32,
        &Tile::Next => '⋁' as u32,
    }
}
