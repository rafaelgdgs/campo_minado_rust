use macroquad::prelude::*;

const GRID_SIZE: usize = 10;
const TILE_SIZE: f32 = 40.0;

fn draw_grid() {
    for i in 0..GRID_SIZE {
        for j in 0..GRID_SIZE {
            let x = i as f32 * TILE_SIZE;
            let y = j as f32 * TILE_SIZE;

            draw_rectangle(x, y, TILE_SIZE - 2.0, TILE_SIZE - 2.0, GRAY);
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
    loop {
        clear_background(WHITE);

        draw_grid();
        let mouse_pos = mouse_position();
        draw_circle(mouse_pos.0, mouse_pos.1, 100.0, GREEN);

        next_frame().await;
    }
}
