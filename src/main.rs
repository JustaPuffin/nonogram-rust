use macroquad::prelude::*;
use macroquad::miniquad::window;

#[path ="nonograms/level.rs"] mod level;
#[path ="modes/nonogram.rs"] mod nonogram;

type ModeType = i16;

//const MENU_MAIN : ModeType = 0;
//const MENU_PACK : ModeType = 1;
//const MENU_LEVEL : ModeType = 2;
//const MENU_OPTIONS : ModeType = 3;
const NONOGRAM_PLAY: ModeType = 4;
const NONOGRAM_FINISHED: ModeType = 5;

const WINDOW_WIDTH: u32 = 1980;
const WINDOW_HEIGHT: u32 = 1080;


static mut PACK: usize = 3;
static mut LEVEL: usize = 0;

static mut ROSTER: Vec<Vec<Field>> = vec![];
static mut HINTS_ROWS: Vec<Vec<Hint>> = vec![];
static mut HINTS_COLOUMNS: Vec<Vec<Hint>> = vec![];

static mut TIMER_START: f64 = 0.0;
static mut TIMER_END: f64 = 0.0;
static mut TIMER_PENALTY_TIME: f64 = 0.0;
static mut TIMER_PENALTY_COUNT: i32 = 0;



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
    let mut timer_text = String::new();
    let grid = unsafe {level::get_data(PACK, LEVEL).grid};

    miniquad::window::set_window_size(WINDOW_WIDTH,WINDOW_HEIGHT);



    unsafe {
        ROSTER = nonogram::get_nonogram_field(grid.clone());
        HINTS_ROWS = nonogram::get_nonogram_hint_rows(grid.clone());
        HINTS_COLOUMNS = nonogram::get_nonogram_hint_coloums(grid.clone());
    }
    
    loop {
        match mode {
            NONOGRAM_PLAY => {
                unsafe {
                    TIMER_START = get_time();
                    TIMER_PENALTY_COUNT = 0;
                };
                loop {
                unsafe{
                    nonogram::nonogram_play();
                    TIMER_PENALTY_TIME = 0.0;
                    for i in 1..TIMER_PENALTY_COUNT + 1 {TIMER_PENALTY_TIME += clamp(f64::powi(2.0, i), 2.0, 8.0) * 60.0}
                    TIMER_END = get_time() + TIMER_PENALTY_TIME;


                    timer_text = "Time: ".to_owned()
                        + &((TIMER_END - TIMER_START) as i32 / 3600 % 60).to_string() + ":"
                        + &((TIMER_END - TIMER_START) as i32 / 60 % 60).to_string() + ":"
                        + &((TIMER_END - TIMER_START) as i32 % 60).to_string();
                }
                draw_text(&timer_text, 20.0, window::screen_size().1 - 20.0, window::screen_size().0 / 20.0, WHITE);
                if unsafe {nonogram::check_roster_state(ROSTER.clone(), PACK, LEVEL)} {
                    mode = NONOGRAM_FINISHED;
                    break;
                }
                next_frame().await}}
            NONOGRAM_FINISHED => {
                loop {
                    unsafe {nonogram::nonogram_finished(PACK, LEVEL, TIMER_END).await;};
                    draw_text(&timer_text, 20.0, window::screen_size().1 - 20.0, window::screen_size().0 / 20.0, WHITE);
                    next_frame().await}}

            _ => todo!("Mode {:?} doesn't exist or isn't implemented yet", mode),
        }
        next_frame().await;
    }
}








