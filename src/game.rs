use super::character::entity::*;
use super::character::item::*;
use super::level::*;
use super::gen::level::*;
use super::gen::monster::*;
use super::ui::*;
use super::effect::{AttackDirection, WeaponHit, Storm, RoundHouse, Effect};
use super::log::*;
use super::ki::*;

/*
    Was kann ich verbessern:
    * Evtl noch nen Defense/ Angriff Boni, der Prozentual berechnet wird
    * Rendern der special attacken.

    UI
    * Die einzelnen Fenster für Loot und bla überschneiden sich, String ausgabe finden die um chars verschiebt -> Anzeige Karte blendet in die Spieler anzeige.
    * Zucker: Anzeigen ob Item besser ist.

    Game
    * Monster-Generierung balancieren.
    * Das wegwerfen von Items bringt Etwas, Etwas gegen Etwas-Anderes eintauschen.
*/

pub struct Game {
    log: Log,
    map: Level,
    player: Entity,

    entity_count: u32,
    enemies: Vec<Entity>,
    effects: Vec<Box<Effect>>,

    game_state: Action,
    backpack_index: usize,
    inventory_pointer: InventoryPointer,
    character_pointer: Type,
    enemy_loot_index: usize,

    player_special_one: bool,
    player_special_two: bool,
    player_special_three: bool,

    level_generator: LevelGenerator,
    window: Window,
}

impl Game {
    pub fn new() -> Game {
        Game {
            log: Log::new(),
            map: Level::new(),
            player: Entity::new(0),

            entity_count: 1,
            enemies: Vec::new(),
            effects: Vec::new(),

            game_state: Action::Game,
            backpack_index: 0,
            inventory_pointer: InventoryPointer::Backpack,
            character_pointer: Type::Head,
            enemy_loot_index: 0,

            player_special_one: false,
            player_special_two: false,
            player_special_three: false,

            level_generator: LevelGenerator::new(),
            window: Window::new(),
        }
    }

    pub fn init(&mut self) {
        self.player.name = "TamaNu".to_string();

        self.map = self.level_generator.generate_level(0);

        self.set_player_and_monsters();
    }

    pub fn run(&mut self) {
        self.window.draw(&mut self.log, &self.map, &self.player, &self.enemies, false, false);

        loop {
            let input = self.window.get_input();

            let next_game_state = match self.game_state {
                Action::Game => {
                    self.handle_game_state(input)
                },
                Action::GameOver => {
                    self.handle_game_over_state(input)
                },
                Action::Loot => {
                    self.handle_loot_state(input)
                },
                Action::Inventory => {
                    self.handle_inventory_state(input)
                },
                Action::Menu => {
                    self.handle_menu_state(input)
                },
                Action::Quit => {
                    break;
                },
            };

            if self.game_state == Action::Game && next_game_state == Action::Loot {
                self.backpack_index = 0;
            }

            if next_game_state == Action::Quit {
                break;
            } else {
                self.game_state = next_game_state;
            }

            let storm_cooldown = self.effects.iter().position(|x| x.actor_id() == self.player.id && x.effect_id() == 2).is_some();
            let kick_cooldown = self.effects.iter().position(|x| x.actor_id() == self.player.id && x.effect_id() == 3).is_some();

            self.window.draw(&mut self.log, &self.map, &self.player, &self.enemies, storm_cooldown, kick_cooldown);

            if self.game_state == Action::Loot {
                let enemy = &self.enemies[self.enemy_loot_index];

                self.window.draw_loot(&enemy.backpack, self.backpack_index, true, &enemy.name)
            } else if self.game_state == Action::Inventory {
                self.window.draw_loot(&self.player.backpack, self.backpack_index, self.inventory_pointer == InventoryPointer::Backpack, &"".to_string());
                self.window.draw_entity(&self.player, self.character_pointer, self.inventory_pointer == InventoryPointer::Character);
            } else if self.game_state == Action::Menu {
                self.window.draw_menu();
            } else if self.game_state == Action::GameOver {
                self.window.draw_game_over();
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
                        let mut monster = Entity::new(self.entity_count);
                        self.entity_count += 1;

                        create_monster(&self.player, &mut monster, mn_type, difficulty);

                        monster.pos_row = row_index;
                        monster.pos_col = col_index;

                        self.enemies.push(monster);
                    },
                    _ => (),
                }

                col_index += 1;
            }

            row_index += 1;
        }
    }

    fn handle_game_over_state(&mut self, input: Input) -> Action {
        match input {
            Input::Use => {
                let player_name = self.player.name.clone();

                self.player = Entity::new(0);
                self.player.name = player_name;

                self.enemies.clear();
                self.map = self.level_generator.generate_level(0);
                self.set_player_and_monsters();

                Action::Game
            }
            Input::Quit => {
                Action::Quit
            }
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
                            let heal_percentage = new_item.get_heal_percentage() as f32;
                            let actual_heal = ((max_life as f32) * (heal_percentage / 100.0f32)).round() as i32;

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
                if !self.enemies[self.enemy_loot_index].backpack.empty_slot(self.backpack_index + 1) {
                    self.backpack_index += 1;
                }
            },

            Input::Use => {
                if self.player.backpack.has_space() {
                    let item = self.enemies[self.enemy_loot_index].backpack.items[self.backpack_index].clone();
                    self.log.add_message(format!("Item {} added to Backpack", item.name));

                    self.enemies[self.enemy_loot_index].backpack.remove_item(self.backpack_index);
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
        if self.player.is_death() {
            return Action::GameOver;
        }

        match input {
            Input::MoveUp | Input::MoveDown | Input::MoveLeft | Input::MoveRight => {
                self.handle_move(input);
            },

            Input::AttackUp | Input::AttackDown | Input::AttackLeft | Input::AttackRight |
            Input::SpecialOne | Input::SpecialTwo | Input::SpecialThree => {
                self.handle_attack(input);
            },

            Input::Use => {
                let player_on_enemy;
                {
                    let enemy = self.enemies.iter().find(|x| x.pos_row == self.player.pos_row && x.pos_col == self.player.pos_col);
                    player_on_enemy = match enemy {
                        Option::Some(..) => true,
                        Option::None => false,
                    }
                }

                if player_on_enemy {
                    let enemy_with_loot = self.enemies.iter().position(|x| x.backpack.size() > 0 && x.pos_row == self.player.pos_row && x.pos_col == self.player.pos_col);

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
                } else {
                    if self.map.meta[self.player.pos_row as usize][self.player.pos_col as usize] == Tile::Next {
                        self.enemies.clear();
                        self.map = self.level_generator.generate_level(self.map.level + 1);
                        self.set_player_and_monsters();

                        return Action::Game;
                    } else if self.map.meta[self.player.pos_row as usize][self.player.pos_col as usize] == Tile::PlSpawn {
                        self.log.add_message("There is no way up.".to_string());

                        return Action::Game;
                    }

                    return Action::Inventory;
                };
            },

            Input::Quit => { return Action::Menu },

            Input::Nothing | Input::Drop => {},
        }

        handle_ki(&self.map, &mut self.player, &mut self.enemies, &mut self.effects);

        self.handle_player_effects();
        self.handle_enemy_effects();

        Action::Game
    }

    fn handle_player_effects(&mut self) {
        let mut player_effects: Vec<usize> = Vec::new();
        let mut index: usize = 0;

        for effect in self.effects.iter() {
            if effect.actor_id() == self.player.id {
                player_effects.push(index);
            }

            index += 1;
        }

        for effect_index in player_effects.iter() {
            for enemy_index in 0..self.enemies.len() {
                self.effects[*effect_index].execute(&mut self.log, &mut self.map, &mut self.player, &mut self.enemies[enemy_index]);
            }
        }

        player_effects.reverse();
        for effect_index in player_effects.iter() {
            if self.effects[*effect_index].done(&mut self.player, &mut self.map) {
                self.effects.remove(*effect_index);
            }
        }
    }

    fn handle_enemy_effects(&mut self) {
        let mut enemy_effects: Vec<usize> = Vec::new();
        let mut index: usize = 0;

        for effect in self.effects.iter() {
            if effect.actor_id() != self.player.id {
                enemy_effects.push(index);
            }

            index += 1;
        }

        for effect_index in enemy_effects.iter() {
            let enemy_index = self.enemies.iter().position(|enemy| enemy.id == self.effects[*effect_index].actor_id()).unwrap();

            self.effects[*effect_index].execute(&mut self.log, &mut self.map, &mut self.enemies[enemy_index], &mut self.player);
        }

        enemy_effects.reverse();
        for effect_index in enemy_effects.iter() {
            let enemy_index = self.enemies.iter().position(|enemy| enemy.id == self.effects[*effect_index].actor_id()).unwrap();

            if self.effects[*effect_index].done(&mut self.enemies[enemy_index], &mut self.map) {
                self.effects.remove(*effect_index);
            }
        }
    }

    fn handle_attack(&mut self, direction: Input) {
        //AttackDirection will not be used.
        //effect is direction_less.
        let effect: Box<Effect> = match direction {
            Input::SpecialOne => Box::new(Storm::new(self.player.id, AttackDirection::North)),
            Input::SpecialTwo => Box::new(RoundHouse::new(self.player.id)),
            Input::SpecialThree => Box::new(WeaponHit::new(self.player.id, AttackDirection::North)),
            _ => Box::new(WeaponHit::new(self.player.id, AttackDirection::North)),
        };

        if effect.needs_direction() {
            match direction {
                Input::SpecialOne => self.player_special_one = true,
                Input::SpecialTwo => self.player_special_two = true,
                Input::SpecialThree => self.player_special_three = true,

                _ => {
                    let attack_direction = match direction {
                        Input::AttackUp => AttackDirection::North,
                        Input::AttackDown => AttackDirection::South,
                        Input::AttackLeft => AttackDirection::West,
                        Input::AttackRight => AttackDirection::East,

                        _ => unreachable!(),
                    };

                    //Here are the correct AttackDirections.
                    let hit: Box<Effect> = if self.player_special_one {
                        Box::new(Storm::new(self.player.id, attack_direction))
                    } else if self.player_special_two {
                        Box::new(RoundHouse::new(self.player.id))
                    } else if self.player_special_three {
                        Box::new(WeaponHit::new(self.player.id, attack_direction))
                    } else {
                        Box::new(WeaponHit::new(self.player.id, attack_direction))
                    };

                    if hit.valid(&self.effects) {
                        self.effects.push(hit);
                    }

                    self.player_special_one = false;
                    self.player_special_two = false;
                    self.player_special_three = false;
                }
            }
        } else {
            if effect.valid(&self.effects) {
                self.effects.push(effect);
            }
        }
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
        if self.map.map[row_diff as usize][col_diff as usize] == Tile::Wall {
            return;
        }

        //Collision with alive entity uncool.
        for enemy in &self.enemies {
            if !enemy.is_death() && row_diff == enemy.pos_row && col_diff == enemy.pos_col {
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
    Menu,
    Quit,
}