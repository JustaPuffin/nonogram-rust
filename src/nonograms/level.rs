use std::{sync::LazyLock, time::{self, Duration}};

pub struct LevelContent<'a> {
    pub name: &'a str,
    pub grid: Vec<Vec<i8>>,
    pub time: time::Duration,
}

pub static mut DATA: LazyLock<Vec<Vec<LevelContent>>> = LazyLock::new(||
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
                time: Duration::from_secs(0),
            },
        ],
    ]);

// gets data for a nonogram
pub unsafe fn get_data(pack: usize, level: usize) -> LevelContent<'static> {
    let puzzle = LevelContent {
        name: DATA[pack][level].name,
        grid: DATA[pack][level].grid.clone(),
        time: DATA[pack][level].time,
    };
    println!("Level name: {}", DATA[pack][level].name);
    return puzzle;
}

