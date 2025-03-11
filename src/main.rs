use macroquad::prelude::*;
use nonogram_play::{check_roster_state, get_nonogram_field, get_nonogram_hint_coloums, get_nonogram_hint_rows, nonogram_play};


#[path ="nonograms/level.rs"]       mod level;
#[path ="modes/nonogram_play.rs"]   mod nonogram_play;

type ModeType                       = i16;

const MENU_MAIN         : ModeType  = 0;
const MENU_PACK         : ModeType  = 1;
const MENU_LEVEL        : ModeType  = 2;
const MENU_OPTIONS      : ModeType  = 3;
const NONOGRAM_PLAY     : ModeType  = 4;
const NONOGRAM_FINISHED : ModeType  = 5;

static mut PACK             : usize = 0;
static mut LEVEL            : usize = 0;
static mut ROSTER           : Vec<Vec<Field>> = vec![];
static mut HINTS_ROWS       : Vec<Vec<Hint>> = vec![];
static mut HINTS_COLOUMNS   : Vec<Vec<Hint>> = vec![];

#[derive(Clone, Debug)] enum HintPosition {
    Row,
    Coloumn,
}

// Used for hints for the nonogram roster
#[derive(Clone, Debug)] struct Hint {
    data    : Field,
    position: HintPosition,
    value   : i8,
}

// Used for squares in the nonogram roster
#[derive(Clone, Debug)] struct Field {
    x       : f32,
    y       : f32,
    size    : f32,
    colour  : Color,
    filled  : i8,
    crossed : i8,
}

#[macroquad::main("Nonogram")]
async fn main() {
    let mut mode: ModeType  = NONOGRAM_PLAY;
    let grid = unsafe {level::get_data(PACK, LEVEL).grid};
    unsafe {
        HINTS_ROWS = get_nonogram_hint_rows(grid.clone());
        HINTS_COLOUMNS = get_nonogram_hint_coloums(grid.clone());
        ROSTER = get_nonogram_field(grid.clone());
    }
    
    
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








