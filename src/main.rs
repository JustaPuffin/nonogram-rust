use macroquad::prelude::*;
use nonogram_play::{check_roster_state, get_nonogram_field, nonogram_play};


#[path ="nonograms/level.rs"]       mod level;
#[path ="modes/nonogram_play.rs"]   mod nonogram_play;

type ModeType                       = i16;

const NONOGRAM_PLAY     : ModeType  = 0;
const NONOGRAM_FINISHED : ModeType  = 1;

 

static mut PACK: usize = 0;
static mut LEVEL: usize = 0;
static mut ROSTER: Vec<Vec<Field>> = vec![];

// Used for squares in the nonogram roster
#[derive(Clone)] struct Field {
    x       : f32,
    y       : f32,
    size    : f32,
    colour  : Color,
    filled  : i8,
    crossed : i8,
}


#[macroquad::main("Nonogram")]
async fn main() {
    let mut mode    : ModeType  = NONOGRAM_PLAY;
    unsafe {ROSTER = get_nonogram_field(level::get_data(PACK, LEVEL))}
    
    loop {
        match mode {
            NONOGRAM_PLAY => loop {
                unsafe{nonogram_play()};
                if unsafe {check_roster_state(ROSTER.clone(), PACK, LEVEL)} {
                    mode = NONOGRAM_FINISHED;
                    break;
                }
                next_frame().await;
            },
            NONOGRAM_FINISHED => loop {
                next_frame().await;
            },
            _ => todo!(),
        }
        next_frame().await;
    }
}








