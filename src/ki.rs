use super::level::*;
use super::character::entity::*;
use super::effect::{WeaponHit, Effect, AttackDirection};
use super::ui::*;

pub fn handle_ki(map: &Level, player: &mut Entity, enemies: &mut Vec<Entity>, effects: &mut Vec<Box<Effect>>) {
    let size = enemies.len();
    for index in 0..size {
        if enemies[index].is_death() {
            continue;
        }

        let row_diff = player.pos_row - enemies[index].pos_row;
        let col_diff = player.pos_col - enemies[index].pos_col;

        let distance = ((row_diff * row_diff + col_diff * col_diff) as f32).sqrt();

        //GameCode!
        if distance == 1f32 {
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

            let hit = WeaponHit::new(enemies[index].id, direction);
            if hit.valid(&effects) {
                effects.push(Box::new(hit));
            }
        } else if distance <= 4f32 {
            let direction = if row_diff >= 0 && col_diff >= 0 {
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
            } else if row_diff <= 0 && col_diff <= 0 {
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

            let mut row_diff = enemies[index].pos_row;
            let mut col_diff = enemies[index].pos_col;

            match direction {
                Input::MoveUp => row_diff -= 1,
                Input::MoveDown => row_diff += 1,
                Input::MoveLeft => col_diff -= 1,
                Input::MoveRight => col_diff += 1,

                _ => { continue; },
            }

            //Collision with Wall uncool.
            if map.map[row_diff as usize][col_diff as usize] == Tile::Wall {
                continue;
            }

            if player.pos_row == row_diff && player.pos_col == col_diff {
                continue;
            }

            let mut collision_with_other_enemy = false;

            for inner_index in 0..size {
                if inner_index == index {
                    continue;
                }

                if !enemies[inner_index].is_death() && row_diff == enemies[inner_index].pos_row && col_diff == enemies[inner_index].pos_col {
                    collision_with_other_enemy = true;
                    break;
                }
            }

            if !collision_with_other_enemy {
                let mut mut_enemy = &mut enemies[index];
                mut_enemy.pos_row = row_diff as i32;
                mut_enemy.pos_col = col_diff as i32;
            }
        }
    }
}