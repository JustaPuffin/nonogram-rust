use macroquad::prelude::*;

#[path ="nonograms/level.rs"]
mod level;

//type ModeType: = i16;


const BACKGROUND: Color = BLACK; // Background colour
const EMPTY: Color = WHITE; // Empty Square
const FILLED: Color = BLUE; // Filled Square
const CROSSED: Color = GRAY; // Crossed out Square

// Used for squares in the nonogram roster
struct Field {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    size: f32,
    colour: Color,
    filled: bool,
    crossed: bool,
}



#[macroquad::main("Nonogram")]
async fn main() {
    let mut roster= get_roster(25, 20, 20);
    let test = level::get_nonogram("00", "00");
    loop {
        clear_background(BACKGROUND);

        if is_mouse_button_pressed(MouseButton::Left) {roster = check_roster_input(MouseButton::Left, roster)}
        else if is_mouse_button_pressed(MouseButton::Right) {roster = check_roster_input(MouseButton::Right, roster)}

        draw_roster(&roster);
        next_frame().await
    }
}

// gets a clear roster
fn get_roster(width: usize, height: usize, size: usize)  -> Vec<Vec<Field>> {
    let mut roster: Vec<Vec<Field>> = vec![];

    

    for y in 0..height {
        roster.push(vec![]);
        for x in 0..width {
            roster[y].push(
                Field {
                    x: x as f32 * 20.0 + (x as f32 * 2.0),
                    y: y as f32 * 20.0 + (y as f32 * 2.0),
                    w: size as f32,
                    h: size as f32,
                    size: 20.0,
                    colour: EMPTY,
                    filled: false,
                    crossed: false,
                }
            )
        }
        
    }

    return roster;
}

// draws a square matrix
fn draw_roster(roster: &Vec<Vec<Field>>) {
    for y in 0..roster.len() {
        for x in 0..roster[y].len() {
            draw_rectangle(
                roster[y][x].x,
                roster[y][x].y,
                roster[y][x].w,
                roster[y][x].h,
                roster[y][x].colour,
            );
        }
    }
}

// checks if the mouse is hovering above a square 
fn check_roster_input(input: MouseButton, roster: Vec<Vec<Field>>) -> Vec<Vec<Field>> {
    let mut temp: Vec<Vec<Field>> = roster;
    for y in 0..temp.len() {
        for x in 0..temp[y].len() {
            if mouse_position().0 > temp[y][x].x && mouse_position().0 < temp[y][x].x + temp[y][x].size {
                if mouse_position().1 > temp[y][x].y && mouse_position().1 < temp[y][x].y + temp[y][x].size {
                    temp = update_roster(input, temp, x, y);
                    break;
                }
            }
        }
    }
    return temp;
}

// updates the roster
fn update_roster(input: MouseButton, roster: Vec<Vec<Field>>, x: usize, y: usize) -> Vec<Vec<Field>> {
    let mut temp: Vec<Vec<Field>> = roster;
    match input {
        MouseButton::Left => if temp[y][x].filled {
            temp[y][x].colour = EMPTY;
            temp[y][x].filled = false;
        } else {
            temp[y][x].colour = FILLED;
            temp[y][x].filled = true;
            temp[y][x].crossed = false;
        }
        MouseButton::Right => if temp[y][x].crossed {
            temp[y][x].colour = EMPTY;
            temp[y][x].crossed = false;
        } else {
            temp[y][x].colour = CROSSED;
            temp[y][x].filled = false;
            temp[y][x].crossed = true;
        }
        _ => todo!()
    }
    return temp
}

