use macroquad::prelude::*;
use macroquad::miniquad::window;
use std::cmp;
use crate::{Field, Hint, HintPosition, HINTS_COLOUMNS, HINTS_ROWS, LEVEL, PACK, ROSTER};
use crate::level::DATA;


const BACKGROUND        : Color     = Color { // Background colour
    r:  75.0 / 255.0,
    g:  91.0 / 255.0,
    b: 171.0 / 255.0,
    a:   1.0,
};
const EMPTY             : Color     = Color { // Empty Square
    r: 255.0 / 255.0,
    g: 255.0 / 255.0,
    b: 235.0 / 255.0,
    a:   1.0,
};
const FILLED            : Color     = Color { // Filled Square
    r:  77.0 / 255.0,
    g: 166.0 / 255.0,
    b: 255.0 / 255.0,
    a:   1.0,
};
const CROSSED           : Color     = Color { // Crossed out Square
    r: 126.0 / 255.0,
    g: 126.0 / 255.0,
    b: 143.0 / 255.0,
    a: 1.0,
};


// function used for the mode NONOGRAM_PLAY
pub unsafe fn nonogram_play() {

    clear_background(BACKGROUND);

    if is_mouse_button_pressed(MouseButton::Left) {
        ROSTER = check_roster_input(MouseButton::Left, ROSTER.clone(), PACK, LEVEL);
        
    }
    else if is_mouse_button_pressed(MouseButton::Right) {
        ROSTER = check_roster_input(MouseButton::Right, ROSTER.clone(), PACK, LEVEL);
    }
    
    draw_roster(ROSTER.clone());
    draw_hint_rows(HINTS_ROWS.clone());
    draw_hint_coloumns(HINTS_COLOUMNS.clone());
}

// gets a clear nonogram field
pub fn get_nonogram_field(grid: Vec<Vec<i8>>) -> Vec<Vec<Field>> {
    let mut roster: Vec<Vec<Field>> = vec![];
    let max = cmp::max(grid.len(), grid[0].len()) as f32;
    let size = window::screen_size().0 / 4.0 / max;
    

    for y in 0..grid.len() {
        roster.push(vec![]);
        for x in 0..grid[y].len() {
            roster[y].push(
                Field {
                    x       : x as f32 * size + (x as f32 * 2.0) + size * (max + 1.0),
                    y       : y as f32 * size + (y as f32 * 2.0) + size * (max + 1.0),
                    size    : size,
                    colour  : EMPTY,
                    filled  : 0,
                    crossed : 0,
                }
            )
        }
        
    }

    return roster;
}

// gets num for rows of nonogram field
pub fn get_nonogram_hint_rows(grid: Vec<Vec<i8>>) -> Vec<Vec<Hint>> {
    let mut hint_rows: Vec<Vec<Hint>> = vec![];
    let mut count: i8;
    let mut temp: usize;
    let max = cmp::max(grid.len(), grid[0].len()) as f32;
    let size = window::screen_size().0 / 4.0 / max;

    for y in 0..grid.len() {
        hint_rows.push(vec![]);
        count = 0;
        for x in 0..grid[y].len() {
            if grid[y][x] == 1 {count += 1}
            else if count > 0 {
                temp = hint_rows[y].len();
                hint_rows[y].push(get_hint(grid[y].len() - temp, y, size, HintPosition::Row, count, max));
                count = 0;
            }
        }
        if count > 0 || hint_rows[y].len() == 0 {
            temp = hint_rows[y].len();
            hint_rows[y].push(get_hint(grid[y].len() - temp, y, size, HintPosition::Row, count, max));
        }
    }
    return hint_rows;
}

// gets num for coloumns of nonogram field
pub fn get_nonogram_hint_coloums(grid: Vec<Vec<i8>>) -> Vec<Vec<Hint>> {
    let mut hint_coloumns: Vec<Vec<Hint>> = vec![];
    let mut count: i8;
    let mut temp: usize;
    let max = cmp::max(grid.len(), grid[0].len()) as f32;
    let size = window::screen_size().0 / 4.0 / max;

    for x in 0..grid.len() {
        hint_coloumns.push(vec![]);
        count = 0;
        for y in 0..grid.len() {
            if grid[y][x] == 1 {count += 1}
            else if count > 0 {
                temp = grid[x].len() - hint_coloumns[x].len();
                hint_coloumns[x].push(get_hint(x, temp, size, HintPosition::Coloumn, count, max));
                count = 0;
            }
        }
        if count > 0 || hint_coloumns.len() == 0 {
            temp = grid[x].len() - hint_coloumns[x].len();
            hint_coloumns[x].push(get_hint(x, temp, size, HintPosition::Coloumn, count, max));
        }
    }

    /*for y in 0..grid.len() {
        if hint_coloumns[y].len() > 1 {
            println!("{:?}", hint_coloumns[y].len());
            for x in 0..grid[y].len() / 2 {
                temp = hint_coloumns[y].len()-1-x;
                count = hint_coloumns[y][x].value;
                println!("count: {:?}, val l: {:?}, val r: {:?}", count, hint_coloumns[y][x].value, hint_coloumns[y][temp].value);
                hint_coloumns[y][x].value = hint_coloumns[y][temp].value;
                println!("count: {:?}, val l: {:?}, val r: {:?}", count, hint_coloumns[y][x].value, hint_coloumns[y][temp].value);
                hint_coloumns[y][temp].value = count;
                println!("count: {:?}, val l: {:?}, val r: {:?}", count, hint_coloumns[y][x].value, hint_coloumns[y][temp].value);
            }
        }
    }*/
    println!("{hint_coloumns:?}");
    return hint_coloumns;
}

// returns a Hint struct from provided contents
fn get_hint(x: usize, y: usize, size: f32, position: HintPosition, value: i8, max: f32) -> Hint {
    match position {
        HintPosition::Row       => return Hint {
            data    : Field {
                x       : x as f32 * size + (x as f32 * 2.0),
                y       : y as f32 * size + (y as f32 * 2.0) + size * (max + 1.75),
                size    : size,
                colour  : EMPTY,
                filled  : 0,
                crossed : 0,
            },
            position: position,
            value   : value,
        },
        HintPosition::Coloumn   => return Hint {
            data    : Field {
                x       : x as f32 * size + (x as f32 * 2.0) + size * (max + 1.25),
                y       : y as f32 * size + (y as f32 * 2.0),
                size    : size,
                colour  : EMPTY,
                filled  : 0,
                crossed : 0,
            },
            position: position,
            value   : value,
        },
    }
    
}

// draws a square matrix
pub fn draw_roster(roster: Vec<Vec<Field>>) {
    //let size = window::screen_size().0 / 2.0 / cmp::max(roster.len(), roster[0].len()) as f32;

    for y in 0..roster.len() {
        for x in 0..roster[y].len() {
            draw_rectangle(
                roster[y][x].x,
                roster[y][x].y,
                roster[y][x].size,
                roster[y][x].size,
                roster[y][x].colour,
            );
        }
    }
}

// draws hints for each row
pub fn draw_hint_rows(hints: Vec<Vec<Hint>>) {
    for y in 0..hints.len() {
        for x in 0..hints[y].len() {
            draw_text(
                &hints[y][x].value.to_string(),
                hints[y][x].data.x,
                hints[y][x].data.y,
                hints[y][x].data.size,
                hints[y][x].data.colour,
            );
        }
    }
}

// draws hints for each coloumn
pub fn draw_hint_coloumns(hints: Vec<Vec<Hint>>) {
    for y in 0..hints.len() {
        for x in 0..hints[y].len() {
            draw_text(
                &hints[y][x].value.to_string(),
                hints[y][x].data.x,
                hints[y][x].data.y,
                hints[y][x].data.size,
                hints[y][x].data.colour,
            );
        }
    }
}

// checks if the mouse is hovering above a square 
pub fn check_roster_input(input: MouseButton, roster: Vec<Vec<Field>>, pack: usize, level: usize) -> Vec<Vec<Field>> {
    let mut temp: Vec<Vec<Field>> = roster;
    for y in 0..temp.len() {
        for x in 0..temp[y].len() {
            if mouse_position().0 > temp[y][x].x && mouse_position().0 < temp[y][x].x + temp[y][x].size {
                if mouse_position().1 > temp[y][x].y && mouse_position().1 < temp[y][x].y + temp[y][x].size {
                    temp = unsafe {update_roster(input, temp, x, y, pack, level)};
                    break;
                }
            }
        }
    }
    return temp;
}

// updates the roster
pub unsafe fn update_roster(input: MouseButton, roster: Vec<Vec<Field>>, x: usize, y: usize, pack: usize, level: usize) -> Vec<Vec<Field>> {
    let mut temp: Vec<Vec<Field>> = roster;
    match input {
        MouseButton::Left => if temp[y][x].filled == 1 {
            temp[y][x].colour = EMPTY;
            temp[y][x].filled = 0;
        } else if DATA[pack][level].grid[y][x] == 1{
            temp[y][x].colour = FILLED;
            temp[y][x].filled = 1;
            temp[y][x].crossed = 0;
        } else {
            temp[y][x].colour = CROSSED;
            temp[y][x].filled = 0;
            temp[y][x].crossed = 1;
            todo!("Add Time Penalty!");
        }
        MouseButton::Right => if temp[y][x].crossed == 1 {
            temp[y][x].colour = EMPTY;
            temp[y][x].crossed = 0;
        } else {
            temp[y][x].colour = CROSSED;
            temp[y][x].filled = 0;
            temp[y][x].crossed = 1;
        }
        _ => todo!()
    }
    return temp
}

// checks to see if the nonogram has been solved
pub unsafe fn check_roster_state(roster: Vec<Vec<Field>>, pack: usize, level: usize) -> bool {
    for y in 0..roster.len() {
        for x in 0..roster[y].len() {
            if roster[y][x].filled != DATA[pack][level].grid[y][x] {
                return false;
            }
        }
    }
    return true
}

