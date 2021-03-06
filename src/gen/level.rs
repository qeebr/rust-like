extern crate rand;

use std;
use rand::{Rng, ThreadRng};

use super::super::level::*;

struct Room {
    row: usize,
    col: usize,

    width: usize,
    height: usize,
}

impl Room {
    fn intersect(&self, other: &Room) -> bool {
        let x_min = self.col - self.width;
        let x_max = self.col + self.width;

        let other_x_min = other.col - other.width;
        let other_x_max = other.col + other.width;


        let y_min = self.row - self.height;
        let y_max = self.row + self.height;

        let other_y_min = other.row - other.height;
        let other_y_max = other.row + other.height;


        return (x_min <= other_x_min && other_x_min <= x_max || other_x_min <= x_min && x_min <= other_x_max) &&
            (y_min <= other_y_min && other_y_min <= y_max || other_y_min <= y_min && y_min <= other_y_max);
    }

    fn distance(&self, row: usize, col: usize) -> usize {
        let diff_row = self.row as i32 - row as i32;
        let diff_col = self.col as i32 - col as i32;

        let q_sum = diff_row * diff_row + diff_col * diff_col;

        (q_sum as f32).sqrt().abs() as usize
    }

    fn room_distance(&self, other_room: &Room) -> usize {
        self.distance(other_room.row, other_room.col)
    }
}

pub struct LevelGenerator {
    rng: ThreadRng,
}

impl LevelGenerator {
    pub fn new() -> LevelGenerator {
        LevelGenerator { rng: rand::thread_rng()}
    }

    pub fn generate_level(&mut self, level: i32) -> Level {
        if level != 0 && level % 10 == 0 {
            return generate_boss_level();
        }

        let size_rows = 18;
        let size_cols = 78;
        let min_room_count = 3;
        let max_room_count = 10;

        let room_count = self.rng.gen_range(min_room_count, max_room_count);
        let mut rooms: Vec<Room> = Vec::new();

        //Distribute rooms.
        let mut room_counter = 0;
        while room_counter < room_count {
            room_counter += 1;

            let room_height = self.rng.gen_range(2, 7);
            let room_width = self.rng.gen_range(2, 7);

            let mut room = Room {
                width: room_width,
                height: room_height,

                row: self.rng.gen_range(room_height, size_rows - (room_height)),
                col: self.rng.gen_range(room_width, size_cols - (room_width))
            };

            let mut retries = 0;
            let max_retries = 50;
            let mut position_found = false;

            while retries < max_retries {
                let mut collision = false;
                for index in 0..rooms.len() {
                    if rooms[index as usize].intersect(&room) {
                        collision = true;
                    }
                }

                if collision {
                    retries += 1;
                } else {
                    position_found = true;
                    break;
                }

                room.row = self.rng.gen_range(room_height, size_rows - (room_height));
                room.col = self.rng.gen_range(room_width, size_cols - (room_width));
            }

            if position_found {
                rooms.push(room);
            }
        }

        //Find room, which is nearest to the middle.
        let mut middle_room = 0 as usize;
        let mut min_distance = std::usize::MAX;

        let mut current_room = 0 as usize;
        for room in &rooms {
            let current_distance = room.distance(size_rows / 2, size_cols / 2);

            if current_distance < min_distance {
                middle_room = current_room;
                min_distance = current_distance;
            }

            current_room += 1;
        }

        //Move every other room to the middle room.
        let mut moved_room = true;
        let target_row = rooms[middle_room].row;
        let target_col = rooms[middle_room].col;

        while moved_room {
            moved_room = false;

            for index in 0..rooms.len() {
                if index == middle_room {
                    continue;
                }

                let mut new_row = rooms[index].row;
                let mut new_col = rooms[index].col;

                let row_diff = target_row as i32 - new_row as i32;
                let col_diff = target_col as i32 - new_col as i32;

                if row_diff >= 0 && col_diff >= 0 {
                    if row_diff > col_diff {
                        new_row += 1;
                    } else {
                        new_col += 1;
                    }
                } else if row_diff > 0 && col_diff < 0 {
                    if row_diff > -1 * col_diff {
                        new_row += 1;
                    } else {
                        new_col -= 1;
                    }
                } else if row_diff <= 0 && col_diff <= 0 {
                    if row_diff < col_diff {
                        new_row -= 1;
                    } else {
                        new_col -= 1;
                    }
                } else if row_diff < 0 && col_diff > 0 {
                    if -1 * row_diff > col_diff {
                        new_row -= 1;
                    } else {
                        new_col += 1;
                    }
                }

                let new_room = Room { row: new_row, col: new_col, height: rooms[index].height, width: rooms[index].width };
                let mut collision = false;
                for second_index in 0..rooms.len() {
                    if index == second_index {
                        continue;
                    }

                    if rooms[second_index].intersect(&new_room) {
                        collision = true;
                        break;
                    }
                }

                if !collision {
                    rooms[index].row = new_row;
                    rooms[index].col = new_col;

                    moved_room = true;
                }
            }
        }

        let mut level = create_level(&rooms, size_rows, size_cols, level);

        //connect all rooms with corridors
        connect_rooms(&mut rooms, &mut level);

        //set meta-information.
        self.add_meta_information(&rooms, &mut level);

        level
    }

    fn add_meta_information(&mut self, rooms: &Vec<Room>, level: &mut Level) {
        //First Room is the Start.
        level.meta[rooms[0].row][rooms[0].col] = Tile::PlSpawn;

        for index in 1..rooms.len() - 1 {
            let spawn_chance = self.rng.gen_range(1, 11);

            if spawn_chance <= 9 {
                self.add_monster(&rooms[index], level);
            }
        }

        level.meta[rooms[rooms.len() - 1].row][rooms[rooms.len() - 1].col] = Tile::Next;
    }

    fn add_monster(&mut self, room: &Room, level: &mut Level) {
        //1 Easy-Many
        //2 Normal-Few Easy-Few
        //3 Hard-One, Normal-Few, Easy-Few

        match self.rng.gen_range(1, 4) {
            1 => {
                let monster_type = self.rng.gen_range(1, 4);

                let row = room.row;
                let col = room.col;
                if inside_level(row, col, level) {
                    level.meta[row][col] = Tile::MnSpawn { difficulty: 1, mn_type: monster_type };
                }

                let row = room.row + 1;
                let col = room.col;
                if inside_level(row, col, level) {
                    level.meta[row][col] = Tile::MnSpawn { difficulty: 1, mn_type: monster_type };
                }

                let row = room.row - 1;
                let col = room.col;
                if inside_level(row, col, level) {
                    level.meta[row][col] = Tile::MnSpawn { difficulty: 1, mn_type: monster_type };
                }

                let row = room.row;
                let col = room.col + 1;
                if inside_level(row, col, level) {
                    level.meta[row][col] = Tile::MnSpawn { difficulty: 1, mn_type: monster_type };
                }

                let row = room.row;
                let col = room.col - 1;
                if inside_level(row, col, level) {
                    level.meta[row][col] = Tile::MnSpawn { difficulty: 1, mn_type: monster_type };
                }
            },
            2 => {
                let easy_monster_type = self.rng.gen_range(1, 4);
                let normal_monster_type = self.rng.gen_range(1, 4);

                let row = room.row;
                let col = room.col;
                if inside_level(row, col, level) {
                    level.meta[row][col] = Tile::MnSpawn { difficulty: 2, mn_type: normal_monster_type };
                }

                let row = room.row + 1;
                let col = room.col;
                if inside_level(row, col, level) {
                    level.meta[row][col] = Tile::MnSpawn { difficulty: 2, mn_type: normal_monster_type };
                }

                let row = room.row - 1;
                let col = room.col;
                if inside_level(row, col, level) {
                    level.meta[row][col] = Tile::MnSpawn { difficulty: 1, mn_type: easy_monster_type };
                }

                let row = room.row;
                let col = room.col + 1;
                if inside_level(row, col, level) {
                    level.meta[row][col] = Tile::MnSpawn { difficulty: 1, mn_type: easy_monster_type };
                }

                let row = room.row;
                let col = room.col - 1;
                if inside_level(row, col, level) {
                    level.meta[row][col] = Tile::MnSpawn { difficulty: 1, mn_type: easy_monster_type };
                }
            },
            3 => {
                let easy_monster_type = self.rng.gen_range(1, 4);
                let normal_monster_type = self.rng.gen_range(1, 4);
                let hard_monster_type = self.rng.gen_range(1, 4);

                let row = room.row;
                let col = room.col;
                if inside_level(row, col, level) {
                    level.meta[row][col] = Tile::MnSpawn { difficulty: 3, mn_type: hard_monster_type };
                }

                let row = room.row + 1;
                let col = room.col;
                if inside_level(row, col, level) {
                    level.meta[row][col] = Tile::MnSpawn { difficulty: 2, mn_type: normal_monster_type };
                }

                let row = room.row - 1;
                let col = room.col;
                if inside_level(row, col, level) {
                    level.meta[row][col] = Tile::MnSpawn { difficulty: 2, mn_type: normal_monster_type };
                }

                let row = room.row;
                let col = room.col + 1;
                if inside_level(row, col, level) {
                    level.meta[row][col] = Tile::MnSpawn { difficulty: 1, mn_type: easy_monster_type };
                }

                let row = room.row;
                let col = room.col - 1;
                if inside_level(row, col, level) {
                    level.meta[row][col] = Tile::MnSpawn { difficulty: 1, mn_type: easy_monster_type };
                }
            },
            _ => {},
        }
    }
}

fn inside_level(row: usize, col: usize, level: &Level) -> bool {
    level.map[row][col] == Tile::Floor
}

fn connect_rooms(rooms: &mut Vec<Room>, level: &mut Level) {
    for current_room_index in 0..rooms.len() {
        let mut first_smallest_distance = std::usize::MAX;
        let mut min_distance = std::usize::MAX;
        let mut distanced_rooms: Vec<usize> = Vec::new();

        for other_room_index in 0..rooms.len() {
            if current_room_index == other_room_index {
                continue;
            }

            let new_distance = rooms[current_room_index].room_distance(&rooms[other_room_index]);

            if new_distance < min_distance {
                first_smallest_distance = other_room_index;
                distanced_rooms.clear();
                min_distance = new_distance;
            } else if new_distance == min_distance {
                distanced_rooms.push(other_room_index);
            }
        }

        connect(level, &rooms[current_room_index], &rooms[first_smallest_distance]);

        for other_rooms_index in distanced_rooms {
            connect(level, &rooms[current_room_index], &rooms[other_rooms_index]);
        }
    }

    rooms.sort_by(|a, b| a.col.cmp(&b.col));

    for current_room_index in 1..rooms.len() {
        connect(level, &rooms[current_room_index - 1], &rooms[current_room_index])
    }

    assure_walls_everywhere(level);
}

fn assure_walls_everywhere(level: &mut Level) {
    for row in 1..level.map.len() - 1 {
        for col in 1..level.map[row].len() - 1 {
            if level.map[row][col] == Tile::Floor {
                if level.map[row - 1][col] == Tile::Nothing {
                    level.map[row - 1][col] = Tile::Wall;
                }

                if level.map[row][col - 1] == Tile::Nothing {
                    level.map[row][col - 1] = Tile::Wall;
                }

                if level.map[row - 1][col - 1] == Tile::Nothing {
                    level.map[row - 1][col - 1] = Tile::Wall;
                }

                if level.map[row + 1][col] == Tile::Nothing {
                    level.map[row + 1][col] = Tile::Wall;
                }

                if level.map[row][col + 1] == Tile::Nothing {
                    level.map[row][col + 1] = Tile::Wall;
                }

                if level.map[row + 1][col + 1] == Tile::Nothing {
                    level.map[row + 1][col + 1] = Tile::Wall;
                }
            }
        }
    }
}

fn connect(level: &mut Level, room_a: &Room, room_b: &Room) {
    let mut current_row = room_a.row;
    let mut current_col = room_a.col;

    loop {
        let row_diff = room_b.row as i32 - current_row as i32;
        let col_diff = room_b.col as i32 - current_col as i32;

        if row_diff >= 0 && col_diff >= 0 {
            if row_diff > col_diff {
                current_row += 1;
            } else {
                current_col += 1;
            }
        } else if row_diff > 0 && col_diff < 0 {
            if row_diff > -1 * col_diff {
                current_row += 1;
            } else {
                current_col -= 1;
            }
        } else if row_diff <= 0 && col_diff <= 0 {
            if row_diff < col_diff {
                current_row -= 1;
            } else {
                current_col -= 1;
            }
        } else if row_diff < 0 && col_diff > 0 {
            if -1 * row_diff > col_diff {
                current_row -= 1;
            } else {
                current_col += 1;
            }
        }

        if current_row == room_b.row && current_col == room_b.col {
            break;
        } else {
            level.map[current_row][current_col] = Tile::Floor;
        }
    }
}

fn create_level(rooms: &Vec<Room>, size_rows: usize, size_cols: usize, level : i32) -> Level {
    let mut level = Level { map: Vec::new(), meta: Vec::new(), level : level};
    //Create empty map.
    for row in 0..size_rows {
        level.map.push(Vec::new());
        level.meta.push(Vec::new());

        for col in 0..size_cols {
            level.map[row].push(Tile::Nothing);
            level.meta[row].push(Tile::Nothing);

            assert!(level.map[row][col] == Tile::Nothing);
        }
    }

    for room in rooms {
        for test_row in 0..(room.height) + 1 {
            let row_calc = room.row + test_row;

            for test_col in 0..(room.width) + 1 {
                let col_calc = room.col + test_col;

                level.map[row_calc][col_calc] = Tile::Floor;
                level.map[room.row + room.height][col_calc] = Tile::Wall;
            }
            for test_col in 1..(room.width) + 1 {
                let col_calc = room.col - test_col;

                level.map[row_calc][col_calc] = Tile::Floor;
                level.map[room.row + room.height][col_calc] = Tile::Wall;
            }

            level.map[room.row + test_row][room.col + room.width] = Tile::Wall;
            level.map[room.row + test_row][room.col - room.width] = Tile::Wall;
        }

        for test_row in 1..(room.height) + 1 {
            let row_calc = room.row - test_row;
            for test_col in 0..(room.width) + 1 {
                let col_calc = room.col + test_col;

                level.map[row_calc][col_calc] = Tile::Floor;
                level.map[room.row - room.height][col_calc] = Tile::Wall;
            }
            for test_col in 1..(room.width) + 1 {
                let col_calc = room.col - test_col;

                level.map[row_calc][col_calc] = Tile::Floor;
                level.map[room.row - room.height][col_calc] = Tile::Wall;
            }

            level.map[row_calc][room.col + room.width] = Tile::Wall;
            level.map[row_calc][room.col - room.width] = Tile::Wall;
        }
    }

    level
}

fn generate_boss_level() -> Level {
    let mut rows = Vec::new();

    let mut str_map = Vec::new();
    str_map.push("                               ".to_string());
    str_map.push(" xxxxxxxxxxxxxxxxxxxxxxx       ".to_string());
    str_map.push(" x_____________________x       ".to_string());
    str_map.push(" x_____________________x xxxxx ".to_string());
    str_map.push(" x_____________________xxx___x ".to_string());
    str_map.push(" x___________________________x ".to_string());
    str_map.push(" x_____________________xxx___x ".to_string());
    str_map.push(" x_____________________x xxxxx ".to_string());
    str_map.push(" x_____________________x       ".to_string());
    str_map.push(" xxxxxxxxxxxxxxxxxxxxxxx       ".to_string());
    str_map.push("                               ".to_string());

    for str_row in str_map {
        let mut chars = str_row.chars();
        let mut row = Vec::new();

        loop {
            match chars.next() {
                Option::Some(char) => {
                    match char {
                        ' ' => row.push(Tile::Nothing),
                        'x' => row.push(Tile::Wall),
                        '_' => row.push(Tile::Floor),
                        _ => {},
                    }
                }
                Option::None => break
            }
        }

        rows.push(row);
    }

    let mut meta = Vec::new();

    let mut str_map = Vec::new();
    str_map.push("                               ".to_string());
    str_map.push("                               ".to_string());
    str_map.push("                               ".to_string());
    str_map.push("                               ".to_string());
    str_map.push("                               ".to_string());
    str_map.push("    P             B    D   U   ".to_string());
    str_map.push("                               ".to_string());
    str_map.push("                               ".to_string());
    str_map.push("                               ".to_string());
    str_map.push("                               ".to_string());
    str_map.push("                               ".to_string());

    for str_row in str_map {
        let mut chars = str_row.chars();
        let mut row = Vec::new();

        loop {
            match chars.next() {
                Option::Some(char) => {
                    match char {
                        ' ' => row.push(Tile::Nothing),
                        'P' => row.push(Tile::PlSpawn),
                        'B' => row.push(Tile::MnSpawn {mn_type: 4, difficulty: 4}),
                        'D' => row.push(Tile::Nothing),
                        'U' => row.push(Tile::Next),
                        _ => {},
                    }
                }
                Option::None => break
            }
        }

        meta.push(row);
    }

    Level { map: rows, meta: meta, level: 0 }
}

#[test]
fn test_intersect() {
    let a = Room { row: 20, col: 10, height: 5, width: 10 };
    let b = Room { row: 20, col: 10, height: 5, width: 10 };

    assert!(a.intersect(&b));
}

#[test]
fn test_not_intersect() {
    let a = Room { row: 20, col: 10, height: 5, width: 10 };
    let b = Room { row: 31, col: 10, height: 5, width: 10 };

    assert!(!a.intersect(&b));
}
