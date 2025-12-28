mod render;

use macroquad::prelude::*;
use macroquad::window;

struct Player {
    position: Vec2,
    angle: f32,
}

struct GameState {
    player: Player,
}

struct InputState {
    thrust: bool,
    left: bool,
    right: bool,
    shoot: bool,
}

fn init_game() -> GameState {
    GameState {
        player: Player {
            position: Vec2 {
                x: screen_width() * 0.5,
                y: screen_height() * 0.5,
            },
            angle: 0.0,
        },
    }
}

fn draw_player(player: &Player) {
    const PLAYER_SHAPE: [Vec2; 4] = [
        Vec2 { x: 1.0, y: 0.0 },
        Vec2 { x: -0.5, y: -0.6 },
        Vec2 { x: -0.2, y: 0.0 },
        Vec2 { x: -0.5, y: 0.6 },
    ];
    render::draw_shape(&PLAYER_SHAPE, player.position, player.angle, 30.0);
}

fn draw(state: &GameState) {
    clear_background(BLACK);
    draw_player(&state.player);
}

fn get_input() -> InputState {
    InputState {
        thrust: is_key_down(KeyCode::W),
        left: is_key_down(KeyCode::A),
        right: is_key_down(KeyCode::D),
        shoot: is_key_down(KeyCode::Space),
    }
}

fn update(state: GameState, _input: InputState, _dt: f32) -> GameState {
    state
}

#[macroquad::main("Asteroids")]
async fn main() {
    let mut state = init_game();
    loop {
        let dt = get_frame_time();
        let input = get_input();
        state = update(state, input, dt);
        draw(&state);
        next_frame().await
    }
}
