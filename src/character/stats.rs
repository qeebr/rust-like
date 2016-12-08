#[derive(Debug, Copy, Clone)]
pub struct Stats {
    pub vitality: i32,
    pub strength: i32,
    pub speed: i32,
    pub defense: i32,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Stat {
    Vitality(i32),
    Strength(i32),
    Speed(i32),
    Defense(i32),
}
