use std::sync::LazyLock;
use crate::level::LevelContent;



pub const DATA: LevelContent = LevelContent {
    name: "Clock",
    grid: LazyLock::new(||vec![
        [0,1,1,1,0].to_vec(),
        [1,0,1,0,1].to_vec(),
        [1,0,1,1,1].to_vec(),
        [1,0,0,0,1].to_vec(),
        [0,1,1,1,0].to_vec(),
    ]),
};