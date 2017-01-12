extern crate rand;
extern crate ncurses;

pub mod character;
pub mod combat;
pub mod level;
pub mod ui;
pub mod log;
pub mod game;
pub mod gen;
pub mod ki;

fn main() {
    game::game();
}