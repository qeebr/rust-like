use super::entity::*;

pub enum MonsterType {
    Crab,
    Zombie,
    Goblin,
}

pub enum Difficulty {
    Easy,
    Normal,
    Hard,
}

pub struct Monster {
    pub monster_type: MonsterType,
    pub monster_difficulty: Difficulty,
    pub entity: Entity,
}


impl Monster {
    pub fn new(mon_type : MonsterType, mon_diff : Difficulty, entity : Entity) -> Monster {
        Monster {monster_type : mon_type, monster_difficulty : mon_diff, entity : entity}
    }
}