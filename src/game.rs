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
    * (1) Seed der Maps fest machen,
    * Alle 10 oder 20 Level/Monster ein Boss-Monster einfügen, das richtig BÄM macht -> Krasseren Loot droppt, -> den Zusatz aus Master rausnehmen und nur für diese Klasse von Items verwenden.
    * Die Stats der Items ebenfalls in KLassen einteilen, das Helme immer weniger haben wie Chests und Chests am meisten und Legs am wenigsten oder so.

    UI
    * (1) Anzeige wievieltes Levels man atm. ist.
    * Die einzelnen Fenster für Loot und bla überschneiden sich, String ausgabe finden die um chars verschiebt -> Anzeige Karte blendet in die Spieler anzeige.
    * Atm. nur ein Monster-Symbol.
    * Zucker: Anzeigen ob Item besser ist.

    Game
    * Game-Over anzeigen, wenn Spieler tot ist.
    * Spezial-Attacken einfügen.
    * Monster-Generierung balancieren.
    * Das verkaufen von Items bringt Gold, Gold gegen Healing-Potions eintauschen.
*/

pub struct Game {
    log: Log,
    map: Level,
    player: Entity,

    enemies: Vec<Monster>,
    effect_list: Vec<WeaponAttack>,

    game_state: Action,
    backpack_index: usize,
    inventory_pointer: InventoryPointer,
    character_pointer: Type,
    enemy_loot_index: usize,
}

impl Game {
    pub fn new() -> Game {
        Game {
            log: Log::new(),
            map: Level::new(),
            player: Entity::new(),

            enemies: Vec::new(),
            effect_list: Vec::new(),

            game_state: Action::Game,
            backpack_index: 0,
            inventory_pointer: InventoryPointer::Backpack,
            character_pointer: Type::Head,
            enemy_loot_index: 0
        }
    }

    pub fn init(&mut self) {
        self.player.name = "qriz".to_string();

        self.map = generate_level();

        self.set_player_and_monsters();

        Window::init();
    }

    pub fn run(&mut self) {
        Window::draw(&mut self.log, &self.map, &self.player, &self.enemies, &self.effect_list);

        loop {
            let input = Window::get_input();

            let next_game_state = match self.game_state {
                Action::Game => {
                    self.handle_game_state(input)
                },
                Action::GameOver => {
                    self.handle_game_over_state(input)
                },
                Action::Loot => {
                    self.handle_loot_state(input)
                }
                Action::Inventory => {
                    self.handle_inventory_state(input)
                }
                Action::NextLevel => {
                    self.enemies.clear();
                    self.map = generate_level();
                    self.set_player_and_monsters();
                    Action::Game
                }
                Action::Menu => {
                    self.handle_menu_state(input)
                }
                Action::Quit => {
                    break;
                }
            };

            if self.game_state == Action::Game && next_game_state == Action::Loot {
                self.backpack_index = 0;
            }

            if next_game_state == Action::Quit {
                break;
            } else {
                self.game_state = next_game_state;
            }

            Window::draw(&mut self.log, &self.map, &self.player, &self.enemies, &self.effect_list);

            if self.game_state == Action::Loot {
                let enemy = &self.enemies[self.enemy_loot_index];

                Window::draw_loot(&enemy.entity.backpack, self.backpack_index, true, &enemy.entity.name)
            } else if self.game_state == Action::Inventory {
                Window::draw_loot(&self.player.backpack, self.backpack_index, self.inventory_pointer == InventoryPointer::Backpack, &"".to_string());
                Window::draw_entity(&self.player, self.character_pointer, self.inventory_pointer == InventoryPointer::Character);
            } else if self.game_state == Action::Menu {
                Window::draw_menu();
            } else if self.game_state == Action::GameOver {
                Window::draw_game_over();
            }
        }
    }

    pub fn cleanup(&mut self) {
        Window::clear();
    }

    fn set_player_and_monsters(&mut self) {
        let mut row_index = 0;
        for meta_row in &self.map.meta {
            let mut col_index = 0;
            for meta_col in meta_row {
                match meta_col {
                    &Tile::PlSpawn => {
                        self.player.pos_row = row_index;
                        self.player.pos_col = col_index;
                    },
                    &Tile::MnSpawn { mn_type, difficulty } => {
                        let mut monster = create_monster(&self.player, mn_type, difficulty);

                        monster.entity.pos_row = row_index;
                        monster.entity.pos_col = col_index;

                        self.enemies.push(monster);
                    },
                    _ => (),
                }

                col_index += 1;
            }

            row_index += 1;
        }
    }

    fn handle_game_over_state(&self, input: Input) -> Action {
        match input {
            _ => { Action::GameOver }
        }
    }

    fn handle_menu_state(&self, input: Input) -> Action {
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

    fn handle_inventory_state(&mut self, input: Input) -> Action {
        match self.inventory_pointer {
            InventoryPointer::Backpack => {
                match input {
                    Input::MoveUp => {
                        if self.backpack_index > 0 {
                            self.backpack_index -= 1;
                        }
                    },
                    Input::MoveDown => {
                        if !self.player.backpack.empty_slot(self.backpack_index + 1) {
                            self.backpack_index += 1;
                        }
                    },
                    Input::Use => {
                        let new_item: Item = self.player.backpack.items[self.backpack_index].clone();

                        if new_item.item_type == Type::Potion {
                            let max_life = self.player.calculate_max_life();
                            let heal_percentage = new_item.get_heal_percentage();
                            let actual_heal = max_life / heal_percentage;

                            self.player.current_life = self.player.current_life + actual_heal;
                            if self.player.current_life > max_life {
                                self.player.current_life = max_life;
                            }

                            self.player.backpack.remove_item(self.backpack_index);
                            self.log.add_message(format!("Player {} have been healed.", self.player.name));
                        } else {
                            let name_clone = new_item.name.clone();
                            self.player.backpack.remove_item(self.backpack_index);
                            let old_item = self.player.equip(new_item);

                            if old_item.item_type != Type::Nothing {
                                self.player.backpack.insert_item(self.backpack_index, old_item);
                            }

                            self.log.add_message(format!("Player {} equipped {}", self.player.name, name_clone));
                        }
                    },
                    Input::Drop => {
                        let new_item: Item = self.player.backpack.items[self.backpack_index].clone();

                        if new_item.item_type != Type::Nothing {
                            self.player.backpack.remove_item(self.backpack_index);
                            self.log.add_message(format!("Player {} dropped {}.", self.player.name, new_item.name));
                        }
                    }

                    Input::MoveLeft => {
                        self.inventory_pointer = InventoryPointer::Character;
                    }

                    Input::Quit => { return Action::Game },
                    _ => {},
                };
            },
            InventoryPointer::Character => {
                match input {
                    Input::MoveUp => {
                        match self.character_pointer {
                            Type::Chest => {
                                self.character_pointer = Type::Head;
                            },
                            Type::Legs => {
                                self.character_pointer = Type::Chest;
                            },
                            Type::Weapon => {
                                self.character_pointer = Type::Legs;
                            },
                            _ => {},
                        }
                    },
                    Input::MoveDown => {
                        match self.character_pointer {
                            Type::Head => {
                                self.character_pointer = Type::Chest;
                            }
                            Type::Chest => {
                                self.character_pointer = Type::Legs;
                            },
                            Type::Legs => {
                                self.character_pointer = Type::Weapon;
                            },
                            _ => {},
                        }
                    }

                    Input::MoveRight => {
                        self.inventory_pointer = InventoryPointer::Backpack;
                    },

                    Input::Quit => { return Action::Game },
                    _ => {},
                }
            },
        }

        Action::Inventory
    }

    fn handle_loot_state(&mut self, input: Input) -> Action {
        match input {
            Input::MoveUp => {
                if self.backpack_index > 0 {
                    self.backpack_index -= 1;
                }
            },
            Input::MoveDown => {
                if !self.enemies[self.enemy_loot_index].entity.backpack.empty_slot(self.backpack_index + 1) {
                    self.backpack_index += 1;
                }
            },

            Input::Use => {
                if self.player.backpack.has_space() {
                    let item = self.enemies[self.enemy_loot_index].entity.backpack.items[self.backpack_index].clone();
                    self.log.add_message(format!("Item {} added to Backpack", item.name));

                    self.enemies[self.enemy_loot_index].entity.backpack.remove_item(self.backpack_index);
                    match self.player.backpack.add_item(item) {
                        Result::Err(..) => { panic!("Error") },
                        _ => {},
                    }
                } else {
                    self.log.add_message("Backpack ist full!".to_string());
                }
            },

            Input::Quit => { return Action::Game; },
            _ => {},
        }


        Action::Loot
    }

    fn handle_game_state(&mut self, input: Input) -> Action {
        self.effect_list.clear();

        if self.player.is_death() {
            return Action::GameOver;
        }

        match input {
            Input::MoveUp | Input::MoveDown | Input::MoveLeft | Input::MoveRight => {
                self.handle_move(input);
            },

            Input::AttackUp | Input::AttackDown | Input::AttackLeft | Input::AttackRight => {
                self.handle_attack(input);
            },

            Input::Use => {
                let enemy = self.enemies.iter().find(|x| x.entity.pos_row == self.player.pos_row && x.entity.pos_col == self.player.pos_col);

                match enemy {
                    Option::Some(..) => {
                        let enemy_with_loot = self.enemies.iter().position(|x| x.entity.backpack.size() > 0 && x.entity.pos_row == self.player.pos_row && x.entity.pos_col == self.player.pos_col);

                        match enemy_with_loot {
                            Option::Some(value) => {
                                self.enemy_loot_index = value;
                                return Action::Loot;
                            },
                            _ => {
                                self.log.add_message("Nothing to loot here.".to_string());
                                return Action::Game;
                            }
                        }
                    },
                    _ => {
                        if self.map.meta[self.player.pos_row as usize][self.player.pos_col as usize] == Tile::Next {
                            return Action::NextLevel;
                        }
                        return Action::Inventory;
                    },
                }
            },

            Input::Quit => { return Action::Menu },

            Input::Nothing | Input::Drop => {},
        }

        handle_ki(&mut self.log, &self.map, &mut self.player, &mut self.enemies, &mut self.effect_list);

        Action::Game
    }

    fn handle_attack(&mut self, direction: Input) {
        let attack_direction = match direction {
            Input::AttackUp => AttackDirection::North,
            Input::AttackDown => AttackDirection::South,
            Input::AttackLeft => AttackDirection::West,
            Input::AttackRight => AttackDirection::East,
            _ => unreachable!(),
        };

        let attack = WeaponAttack::new(&self.player, attack_direction);

        for enemy in &mut self.enemies {
            for &(row, col) in &attack.area {
                if enemy.entity.pos_row == row && enemy.entity.pos_col == col {
                    Fight::weapon_hit(&mut self.log, RndGenerator, &self.player, &mut enemy.entity);
                }
            }
        }

        self.effect_list.push(attack);
    }

    fn handle_move(&mut self, direction: Input) {
        let mut row_diff = self.player.pos_row;
        let mut col_diff = self.player.pos_col;

        match direction {
            Input::MoveUp => row_diff -= 1,
            Input::MoveDown => row_diff += 1,
            Input::MoveLeft => col_diff -= 1,
            Input::MoveRight => col_diff += 1,

            _ => unreachable!(),
        }

        //Collision with Wall uncool.
        if self.map.level[row_diff as usize][col_diff as usize] == Tile::Wall {
            return;
        }

        //Collision with alive entity uncool.
        for enemy in &self.enemies {
            if !enemy.entity.is_death() && row_diff == enemy.entity.pos_row && col_diff == enemy.entity.pos_col {
                return;
            }
        }

        self.player.pos_row = row_diff as i32;
        self.player.pos_col = col_diff as i32;
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
    GameOver,
    Loot,
    Inventory,
    NextLevel,
    Menu,
    Quit,
}