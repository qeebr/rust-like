use super::log::Log;
use super::character::entity::Entity;
use super::combat::fight::{Fight, RndGenerator};
use super::level::{Level, Tile};

pub trait Effect {

    fn execute(&mut self, log: &mut Log, map: &mut Level, me: &mut Entity, other: &mut Entity);

    // Bool -> true delete from list.
    fn done(&mut self, me: &mut Entity, map: &mut Level) -> bool;

    fn actor_id(&self) -> u32;
}

#[derive(Clone)]
pub enum AttackDirection {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

pub struct WeaponHit {
    pub direction: AttackDirection,
    pub id:u32,
}

impl Effect for WeaponHit {
    fn execute(&mut self, log: &mut Log, map: &mut Level, me: &mut Entity, mut other: &mut Entity) {
        simple_attack(&self.direction, log, me.pos_row, me.pos_col, me, other);
    }

    fn done(&mut self, me: &mut Entity, map: &mut Level) -> bool {
        true
    }
    fn actor_id(&self) -> u32 {
        self.id
    }
}

fn simple_attack(direction: &AttackDirection, log: &mut Log, pos_row:i32, pos_col:i32, me: &mut Entity, mut other: &mut Entity) {
    let attack_area = resolve_attack_area(direction, pos_row, pos_col);

    for (row, col) in attack_area {
        if other.pos_row == row && other.pos_col == col {
            Fight::weapon_hit(log, RndGenerator, me, &mut other);
        }
    }
}

fn resolve_attack_area(dir: &AttackDirection, pos_row:i32, pos_col:i32) -> Vec<(i32, i32)> {
    let mut attack_area = Vec::new();

    match dir {
        &AttackDirection::North => {
            //---
            // @
            attack_area.push((pos_row - 1, pos_col - 1));
            attack_area.push((pos_row - 1, pos_col));
            attack_area.push((pos_row - 1, pos_col + 1));
        },
        &AttackDirection::NorthEast => {
            // --
            // @-
            attack_area.push((pos_row - 1, pos_col));
            attack_area.push((pos_row - 1, pos_col + 1));
            attack_area.push((pos_row, pos_col + 1));
        },
        &AttackDirection::East => {
            //  -
            // @-
            //  -
            attack_area.push((pos_row - 1, pos_col + 1));
            attack_area.push((pos_row, pos_col + 1));
            attack_area.push((pos_row + 1, pos_col + 1));
        },
        &AttackDirection::SouthEast => {
            // @-
            // --
            attack_area.push((pos_row, pos_col + 1));
            attack_area.push((pos_row + 1, pos_col + 1));
            attack_area.push((pos_row + 1, pos_col));
        },
        &AttackDirection::South => {
            // @
            //---
            attack_area.push((pos_row + 1, pos_col + 1));
            attack_area.push((pos_row + 1, pos_col));
            attack_area.push((pos_row + 1, pos_col - 1));
        },
        &AttackDirection::SouthWest => {
            //-@
            //--
            attack_area.push((pos_row + 1, pos_col));
            attack_area.push((pos_row + 1, pos_col - 1));
            attack_area.push((pos_row, pos_col - 1));
        },
        &AttackDirection::West => {
            //-
            //-@
            //-
            attack_area.push((pos_row + 1, pos_col - 1));
            attack_area.push((pos_row, pos_col - 1));
            attack_area.push((pos_row - 1, pos_col - 1));
        },
        &AttackDirection::NorthWest => {
            //--
            //-@
            attack_area.push((pos_row, pos_col - 1));
            attack_area.push((pos_row - 1, pos_col - 1));
            attack_area.push((pos_row - 1, pos_col));
        },
    };

    attack_area
}

pub struct Storm {
    pub direction: AttackDirection,
    pub id: u32,
}

impl Effect for Storm {

    fn execute(&mut self, log: &mut Log, map: &mut Level, me: &mut Entity, other: &mut Entity) {
        let (row, col) = resolve_direction(&self.direction);
        let mut steps = 5;

        let mut pos_row = me.pos_row;
        let mut pos_col = me.pos_col;

        while steps > 0 {
            steps -= 1;

            simple_attack(&self.direction, log, pos_row, pos_col, me, other);

            if map.map[(pos_row + row) as usize][(pos_col + col) as usize] == Tile::Floor {
                pos_row += row;
                pos_col += col;
            }
        }

        simple_attack(&self.direction, log, pos_row, pos_col, me, other);
    }

    fn done(&mut self, me: &mut Entity, map: &mut Level) -> bool {
        let (row, col) = resolve_direction(&self.direction);
        let mut steps = 5;

        while steps > 0 {
            steps -= 1;

            if map.map[(me.pos_row + row) as usize][(me.pos_col + col) as usize] == Tile::Floor {
                me.pos_row += row;
                me.pos_col += col;
            }
        }

        true
    }

    fn actor_id(&self) -> u32 {
        self.id
    }
}

fn resolve_direction(direction: &AttackDirection) -> (i32, i32) {
    match direction {
        &AttackDirection::North => { (-1, 0) },
        &AttackDirection::South => { (1, 0) },
        &AttackDirection::East => { (0, 1) },
        &AttackDirection::West => { (0, -1) },
        _ => { panic!("not supported.") }
    }
}