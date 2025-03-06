use macroquad::prelude::*;

const BACKGROUND: Color = BLACK;
const EMPTY: Color = WHITE;
const FILLED: Color = BLUE;

struct Field {
    x: f32,
    y: f32,
    size: f32,
    colour: Color,
    filled: bool,
    crossed: bool,
}

#[macroquad::main("Nonogram")]
async fn main() {
    let mut roster: Vec<Vec<Field>> = get_roster(5);
    loop {
        clear_background(BACKGROUND);
        draw_roster(&roster);
        next_frame().await
    }
}

// gets a clear roster
fn get_roster(size: usize) -> Vec<Vec<Field>> {
    let mut roster: Vec<Vec<Field>> = vec![];

    for y in 0..size {
        roster.push(vec![]);
        for x in 0..size {
            roster[y].push(
                Field {
                    x: x as f32 * 20.0 + (x as f32 * 2.0),
                    y: y as f32 * 20.0 + (y as f32 * 2.0),
                    size: 20.0,
                    colour: EMPTY,
                    filled: false,
                    crossed: false
                }
            );
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
                roster[y][x].size,
                roster[y][x].size,
                roster[y][x].colour,
            );
        }
    }
}