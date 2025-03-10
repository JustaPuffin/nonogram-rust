use std::sync::LazyLock;
use macroquad::prelude::*;

#[path ="level-00/0-data.rs"]
mod level0000;

pub struct LevelContent<'a> {
    pub name: &'a str,
    pub grid: LazyLock<Vec<Vec<i16>>>,
}

// gets data for a nonogram
pub fn get_nonogram(difficulty: &str, level: &str) -> LevelContent<'static> {
    let puzzle = LevelContent {
        name: level0000::DATA.name,
        grid: level0000::DATA.grid,
    };
    return puzzle;
}
