use macroquad::prelude::*;

const BACKGROUND: Color = BLACK;
const EMPTY: Color = WHITE;
const FILLED: Color = BLUE;

struct Field {
    colour: Color,
    filled: bool,
    crossed: bool,
}

#[macroquad::main("Nonogram")]
async fn main() {
    
    loop {
        clear_background(BACKGROUND);
        draw_fields(get_fields(5));
        next_frame().await
    }
}

fn get_fields(size: usize) -> Vec<Vec<Field>> {
    let mut roster: Vec<Vec<Field>> = vec![];

    for y in 0..size {
        roster.push(vec![]);
        for _x in 0..size {
            roster[y].push(
                Field {
                    colour: EMPTY,
                    filled: false,
                    crossed: false
                }
            );
        }
        
    }
    
    return roster;
}

fn draw_fields(roster: Vec<Vec<Field>>) {
    for y in 0..roster.len() {
        for x in 0..roster[y].len() {
            draw_rectangle(
                x as f32 * 20.0 + (x as f32 * 2.0),
                y as f32 * 20.0 + (y as f32 * 2.0),
                20.0, 20.0,
                roster[y][x].colour,
            );
        }
    }
}