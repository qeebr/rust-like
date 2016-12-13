pub struct Level {
    pub level: Vec<Vec<Tile>>,

    pub meta: Vec<Vec<Tile>>,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Tile {
    Nothing,
    Floor,
    Wall,
    PlSpawn,
    MnSpawn { mn_type : u32, difficulty : u32 },
}

impl Level {
    pub fn new() -> Level {
        let mut rows = Vec::new();

        let row = vec![Tile::Nothing, Tile::Nothing, Tile::Nothing, Tile::Nothing, Tile::Nothing, Tile::Nothing, Tile::Nothing, Tile::Nothing, Tile::Nothing];
        rows.push(row);
        let row = vec![Tile::Nothing, Tile::Wall, Tile::Wall, Tile::Wall, Tile::Wall, Tile::Wall, Tile::Wall, Tile::Wall, Tile::Nothing];
        rows.push(row);
        let row = vec![Tile::Nothing, Tile::Wall, Tile::Floor, Tile::Floor, Tile::Floor, Tile::Floor, Tile::Floor, Tile::Wall, Tile::Nothing];
        rows.push(row);
        let row = vec![Tile::Nothing, Tile::Wall, Tile::Floor, Tile::Floor, Tile::Floor, Tile::Floor, Tile::Floor, Tile::Wall, Tile::Nothing];
        rows.push(row);
        let row = vec![Tile::Nothing, Tile::Wall, Tile::Floor, Tile::Floor, Tile::Floor, Tile::Floor, Tile::Floor, Tile::Wall, Tile::Nothing];
        rows.push(row);
        let row = vec![Tile::Nothing, Tile::Wall, Tile::Floor, Tile::Floor, Tile::Floor, Tile::Floor, Tile::Floor, Tile::Wall, Tile::Nothing];
        rows.push(row);
        let row = vec![Tile::Nothing, Tile::Wall, Tile::Floor, Tile::Floor, Tile::Floor, Tile::Floor, Tile::Floor, Tile::Wall, Tile::Nothing];
        rows.push(row);
        let row = vec![Tile::Nothing, Tile::Wall, Tile::Wall, Tile::Wall, Tile::Wall, Tile::Wall, Tile::Wall, Tile::Wall, Tile::Nothing];
        rows.push(row);
        let row = vec![Tile::Nothing, Tile::Nothing, Tile::Nothing, Tile::Nothing, Tile::Nothing, Tile::Nothing, Tile::Nothing, Tile::Nothing, Tile::Nothing];
        rows.push(row);

        let mut meta = Vec::new();
        let row = vec![Tile::Nothing, Tile::Nothing, Tile::Nothing, Tile::Nothing, Tile::Nothing, Tile::Nothing, Tile::Nothing, Tile::Nothing, Tile::Nothing];
        meta.push(row);
        let row = vec![Tile::Nothing, Tile::Nothing, Tile::Nothing, Tile::Nothing, Tile::Nothing, Tile::Nothing, Tile::Nothing, Tile::Nothing, Tile::Nothing];
        meta.push(row);
        let row = vec![Tile::Nothing, Tile::Nothing, Tile::PlSpawn, Tile::Nothing, Tile::Nothing, Tile::Nothing, Tile::Nothing, Tile::Nothing, Tile::Nothing];
        meta.push(row);
        let row = vec![Tile::Nothing, Tile::Nothing, Tile::Nothing, Tile::Nothing, Tile::Nothing, Tile::Nothing, Tile::Nothing, Tile::Nothing, Tile::Nothing];
        meta.push(row);
        let row = vec![Tile::Nothing, Tile::Nothing, Tile::Nothing, Tile::Nothing, Tile::Nothing, Tile::Nothing, Tile::Nothing, Tile::Nothing, Tile::Nothing];
        meta.push(row);
        let row = vec![Tile::Nothing, Tile::Nothing, Tile::Nothing, Tile::Nothing, Tile::Nothing, Tile::Nothing, Tile::Nothing, Tile::Nothing, Tile::Nothing];
        meta.push(row);
        let row = vec![Tile::Nothing, Tile::Nothing, Tile::Nothing, Tile::Nothing, Tile::Nothing, Tile::Nothing, Tile::MnSpawn{mn_type:1,difficulty:1}, Tile::Nothing, Tile::Nothing];
        meta.push(row);
        let row = vec![Tile::Nothing, Tile::Nothing, Tile::Nothing, Tile::Nothing, Tile::Nothing, Tile::Nothing, Tile::Nothing, Tile::Nothing, Tile::Nothing];
        meta.push(row);
        let row = vec![Tile::Nothing, Tile::Nothing, Tile::Nothing, Tile::Nothing, Tile::Nothing, Tile::Nothing, Tile::Nothing, Tile::Nothing, Tile::Nothing];
        meta.push(row);

        Level { level: rows, meta: meta }
    }
}
