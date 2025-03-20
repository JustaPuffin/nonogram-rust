use std::sync::LazyLock;

pub struct LevelContent<'a> {
    pub name: &'a str,
    pub grid: Vec<Vec<i8>>,
    pub frames: i16,
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
                frames: 16,
            },
        ],
        vec![               // Level Pack   01
            LevelContent {  // Level        00
                name: "Phone",
                grid: vec![
                    vec![1,1,0,0,0,0,1],
                    vec![1,1,0,0,0,0,1],
                    vec![1,0,1,1,1,0,1],
                    vec![0,0,1,1,0,0,1],
                    vec![0,0,0,0,0,0,1],
                    vec![0,1,0,0,0,0,1],
                    vec![1,1,0,0,0,0,1],
                ],
                frames: 16,
            },
        ],
        vec![               // Level Pack   02
            LevelContent {  // Level        00
                name: "Table",
                grid: vec![
                    vec![1,1,1,1,1,1,1,1,1,1],
                    vec![1,1,1,1,1,1,1,1,1,1],
                    vec![1,0,0,0,0,0,0,0,0,1],
                    vec![0,0,0,0,0,0,0,0,0,0],
                    vec![0,0,0,0,0,0,0,0,0,0],
                    vec![1,1,1,0,0,0,0,0,0,1],
                    vec![1,1,1,1,1,1,1,1,1,1],
                    vec![1,0,0,0,1,1,0,0,0,1],
                    vec![1,0,0,1,1,1,0,0,1,1],
                    vec![1,0,0,1,1,1,0,0,1,1],
                ],
                frames: 16,
            },
        ],
        vec![               // Level Pack   03
            LevelContent {  // Level        00
                name: "Table",
                grid: vec![
                    vec![0,1,0,0,0,0,0,0,0,0,0,0,0,0,0],
                    vec![0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
                    vec![0,0,0,0,0,0,0,0,1,0,0,0,0,0,0],
                    vec![0,0,0,0,0,0,0,1,1,1,0,0,0,0,0],
                    vec![0,0,0,1,0,0,0,1,1,1,1,0,0,0,0],
                    vec![0,0,1,1,1,0,1,1,1,1,1,0,0,0,0],
                    vec![0,0,1,1,1,1,1,1,0,1,1,0,0,0,0],
                    vec![0,0,1,1,1,1,1,0,0,0,1,0,0,0,0],
                    vec![0,0,0,1,0,0,1,0,1,0,1,0,0,0,0],
                    vec![0,0,0,1,0,0,0,1,1,0,1,0,0,0,0],
                    vec![0,0,0,0,1,0,1,1,0,1,0,0,0,0,0],
                    vec![0,0,1,1,1,1,1,1,1,1,1,1,0,0,0],
                    vec![0,0,0,0,1,1,1,1,1,1,1,1,1,0,0],
                    vec![0,0,0,0,0,0,1,0,0,0,1,1,0,0,0],
                    vec![0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
                ],
                frames: 4,
            },
        ],
    ]);

// gets data for a nonogram
pub unsafe fn get_data(pack: usize, level: usize) -> LevelContent<'static> {
    let puzzle = LevelContent {
        name: DATA[pack][level].name,
        grid: DATA[pack][level].grid.clone(),
        frames: DATA[pack][level].frames,
    };
    println!("Level name: {}", DATA[pack][level].name);
    return puzzle;
}

