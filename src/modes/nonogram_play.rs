use macroquad::prelude::*;
use macroquad::miniquad::window;
use std::cmp;
use crate::{PACK, LEVEL, ROSTER, Field};
use crate::level::{LevelContent, DATA};

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
    
}

// gets a clear nonogram field
pub fn get_nonogram_field(data: LevelContent<'static>)  -> Vec<Vec<Field>> {
    let mut roster: Vec<Vec<Field>> = vec![];
    let size = window::screen_size().0 / 2.0 / cmp::max(data.grid.len(), data.grid[0].len()) as f32;
    

    for y in 0..data.grid.len() {
        roster.push(vec![]);
        for x in 0..data.grid[y].len() {
            roster[y].push(
                Field {
                    x       : x as f32 * size + (x as f32 * 2.0),
                    y       : y as f32 * size + (y as f32 * 2.0),
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
                //window::screen_size().0 / 2.0 / cmp::max(roster.len(), roster[y].len()) as f32,
                //window::screen_size().1 / 2.0 / cmp::max(roster.len(), roster[y].len()) as f32,
                roster[y][x].colour,
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