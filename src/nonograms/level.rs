use std::sync::LazyLock;
use macroquad::prelude::*;

pub struct LevelContent<'a> {
    pub name: &'a str,
    pub grid: Vec<Vec<i32>>,
}

pub const DATA: LazyLock<Vec<Vec<LevelContent>>> = LazyLock::new(||
    vec![                   // Level Collection
        vec![               // Level Pack   00
            LevelContent {  // Level        00
                name: "Clock",
                grid: vec![
                    vec![0,1,1,1,0],
                    vec![1,0,1,0,1],
                    vec![1,0,1,1,1],
                    vec![1,0,0,0,1],
                    vec![0,1,1,1,0],
                ],
            },
        ],
    ]);

// gets data for a nonogram
pub fn get_data(pack: usize, level: usize) -> LevelContent<'static> {
    let puzzle = LevelContent {
        name: DATA[pack][level].name,
        grid: DATA[pack][level].grid.clone(),
    };
    println!("Level name: {}", DATA[pack][level].name);
    return puzzle;
}

