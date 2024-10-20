use ::rand::seq::SliceRandom;
use macroquad::prelude::*;

const GRID_SIZE: usize = 10;
const TILE_SIZE: f32 = 40.0;

#[derive(Clone, Copy, PartialEq)]
enum CellStates {
    Hidden,
    Clicked,
}

#[derive(Clone, Copy, PartialEq)]
enum CellContent {
    Bomb,
    Number(u8),
    Empty,
}

#[derive(Clone, Copy)]
struct Cell {
    state: CellStates,
    content: CellContent,
}

fn create_grid() -> [Cell; GRID_SIZE * GRID_SIZE] {
    [Cell {
        state: CellStates::Hidden,
        content: CellContent::Empty,
    }; GRID_SIZE * GRID_SIZE]
}

fn fill_grid(grid: &mut [Cell; GRID_SIZE * GRID_SIZE]) {
    let num_bombs: usize = GRID_SIZE * 2;
    let mut numbers: Vec<usize> = (0..(GRID_SIZE * GRID_SIZE))
        .collect::<Vec<usize>>()
        .iter()
        .copied()
        .collect();
    numbers.shuffle(&mut ::rand::thread_rng());
    let usable_numbers = &numbers[0..num_bombs];
    for i in 0..num_bombs {
        grid[usable_numbers[i]].content = CellContent::Bomb;
        grid[usable_numbers[i]].state = CellStates::Clicked;
    }
}

fn draw_grid(grid: [Cell; GRID_SIZE * GRID_SIZE]) {
    for i in 0..GRID_SIZE {
        for j in 0..GRID_SIZE {
            let x = i as f32 * TILE_SIZE;
            let y = j as f32 * TILE_SIZE;

            match grid[GRID_SIZE * i + j].state {
                CellStates::Hidden => draw_rectangle(x, y, TILE_SIZE - 2.0, TILE_SIZE - 2.0, GRAY),
                CellStates::Clicked => match grid[GRID_SIZE * i + j].content {
                    CellContent::Empty => {
                        draw_rectangle(x, y, TILE_SIZE - 2.0, TILE_SIZE - 2.0, WHITE)
                    }
                    CellContent::Bomb => draw_circle(
                        x + TILE_SIZE / 2.0,
                        y + TILE_SIZE / 2.0,
                        TILE_SIZE / 2.0,
                        RED,
                    ),
                    CellContent::Number(val) => {
                        draw_text(&val.to_string(), x, y, 5.0, BLACK);
                    }
                },
            }

            //draw_rectangle(x, y, TILE_SIZE - 2.0, TILE_SIZE - 2.0, color);
        }
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Sudoku do rafael".to_owned(),
        window_width: 800,
        window_height: 600,
        window_resizable: true,
        fullscreen: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut base_grid = create_grid();
    fill_grid(&mut base_grid);

    loop {
        clear_background(WHITE);

        draw_grid(base_grid);
        let mouse_pos = mouse_position();
        draw_circle(mouse_pos.0, mouse_pos.1, 10.0, GREEN);

        next_frame().await;
    }
}
