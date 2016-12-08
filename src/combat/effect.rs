use super::super::character::entity::*;

pub struct WeaponAttack {
    pub area: Vec<(i32, i32)>,
}

impl WeaponAttack {
    pub fn new(actor: &Entity, dir: AttackDirection) -> WeaponAttack {
        let mut attack_area = Vec::new();

        match dir {
            AttackDirection::North => {
                //---
                // @
                attack_area.push((actor.pos_row - 1, actor.pos_col - 1));
                attack_area.push((actor.pos_row - 1, actor.pos_col));
                attack_area.push((actor.pos_row - 1, actor.pos_col + 1));
            },
            AttackDirection::NorthEast => {
                // --
                // @-
                attack_area.push((actor.pos_row - 1, actor.pos_col));
                attack_area.push((actor.pos_row - 1, actor.pos_col + 1));
                attack_area.push((actor.pos_row, actor.pos_col + 1));
            },
            AttackDirection::East => {
                //  -
                // @-
                //  -
                attack_area.push((actor.pos_row - 1, actor.pos_col + 1));
                attack_area.push((actor.pos_row, actor.pos_col + 1));
                attack_area.push((actor.pos_row + 1, actor.pos_col + 1));
            },
            AttackDirection::SouthEast => {
                // @-
                // --
                attack_area.push((actor.pos_row, actor.pos_col + 1));
                attack_area.push((actor.pos_row + 1, actor.pos_col + 1));
                attack_area.push((actor.pos_row + 1, actor.pos_col));
            },
            AttackDirection::South => {
                // @
                //---
                attack_area.push((actor.pos_row + 1, actor.pos_col + 1));
                attack_area.push((actor.pos_row + 1, actor.pos_col));
                attack_area.push((actor.pos_row + 1, actor.pos_col - 1));
            },
            AttackDirection::SouthWest => {
                //-@
                //--
                attack_area.push((actor.pos_row + 1, actor.pos_col));
                attack_area.push((actor.pos_row + 1, actor.pos_col - 1));
                attack_area.push((actor.pos_row, actor.pos_col - 1));
            },
            AttackDirection::West => {
                //-
                //-@
                //-
                attack_area.push((actor.pos_row + 1, actor.pos_col - 1));
                attack_area.push((actor.pos_row, actor.pos_col - 1));
                attack_area.push((actor.pos_row - 1, actor.pos_col - 1));
            },
            AttackDirection::NorthWest => {
                //--
                //-@
                attack_area.push((actor.pos_row, actor.pos_col - 1));
                attack_area.push((actor.pos_row - 1, actor.pos_col - 1));
                attack_area.push((actor.pos_row - 1, actor.pos_col));
            },
        };

        WeaponAttack{ area: attack_area}
    }
}

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