mod render;
mod state;
mod update;

use macroquad::prelude::*;
use state::*;

fn init_game() -> GameState {
    GameState {
        player: Player {
            position: Vec2 {
                x: screen_width() * 0.5,
                y: screen_height() * 0.5,
            },
            angle: 0.0,
            velocity: Vec2::ZERO,
        },
    }
}

fn get_input() -> InputState {
    InputState {
        thrust: is_key_down(KeyCode::W),
        left: is_key_down(KeyCode::A),
        right: is_key_down(KeyCode::D),
        shoot: is_key_down(KeyCode::Space),
    }
}

#[macroquad::main("Asteroids")]
async fn main() {
    let mut state = init_game();
    loop {
        let input = get_input();
        let dt = get_frame_time();
        state = update::update(state, &input, dt);
        render::draw(&state, &input);
        next_frame().await
    }
}
