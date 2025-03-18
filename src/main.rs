use macroquad::prelude::*;
use macroquad::miniquad::window;

#[path ="nonograms/level.rs"] mod level;
#[path ="modes/nonogram.rs"] mod nonogram;

type ModeType = i16;

//const MENU_MAIN : ModeType = 0;
//const MENU_PACK : ModeType = 1;
//const MENU_LEVEL : ModeType = 2;
//const MENU_OPTIONS : ModeType = 3;
const NONOGRAM_PLAY : ModeType = 4;
const NONOGRAM_FINISHED : ModeType = 5;


static mut PACK : usize = 0;
static mut LEVEL : usize = 0;
static mut ROSTER : Vec<Vec<Field>> = vec![];
static mut HINTS_ROWS : Vec<Vec<Hint>> = vec![];
static mut HINTS_COLOUMNS : Vec<Vec<Hint>> = vec![];




#[derive(Clone, Debug)] enum HintPosition {
    Row,
    Coloumn,
}

// Used for hints for the nonogram roster
#[derive(Clone, Debug)] struct Hint {
    data    : Field,
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

#[macroquad::main("Nonogram")] async fn main() {
    let mut mode: ModeType  = NONOGRAM_PLAY;
    let mut nonogram_time = String::new();
    let nonogram_start = get_time();
    let nonogram_end: f64;
    let grid = unsafe {level::get_data(PACK, LEVEL).grid};

    unsafe {
        HINTS_ROWS = nonogram::get_nonogram_hint_rows(grid.clone());
        HINTS_COLOUMNS = nonogram::get_nonogram_hint_coloums(grid.clone());
        ROSTER = nonogram::get_nonogram_field(grid.clone());
    }
    
    loop {
        match mode {
            NONOGRAM_PLAY => {
                loop {
                unsafe{nonogram::nonogram_play()};
                nonogram_time = "Time: ".to_owned()
                + &((get_time() - nonogram_start) as i32 / 3600 % 60).to_string() + ":"
                + &((get_time() - nonogram_start) as i32 / 60 % 60).to_string() + ":"
                + &((get_time() - nonogram_start) as i32 % 60).to_string();
                draw_text(&nonogram_time, 20.0, window::screen_size().1 - 20.0, window::screen_size().0 / 20.0, WHITE);
                if unsafe {nonogram::check_roster_state(ROSTER.clone(), PACK, LEVEL)} {
                    mode = NONOGRAM_FINISHED;
                    break;
                }
                next_frame().await}}
            NONOGRAM_FINISHED => {
                nonogram_end = get_time();
                loop {
                    unsafe {nonogram::nonogram_finished(PACK, LEVEL, nonogram_end).await;};
                    draw_text(&nonogram_time, 20.0, window::screen_size().1 - 20.0, window::screen_size().0 / 20.0, WHITE);
                    next_frame().await}}

            _ => todo!("Mode {:?} doesn't exist or isn't implemented yet", mode),
        }
        next_frame().await;
    }
}








