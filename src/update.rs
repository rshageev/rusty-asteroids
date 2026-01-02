use macroquad::math::Vec2;

use crate::state::*;

pub fn update(state: GameState, input: &InputState, dt: f32) -> GameState {
    GameState {
        player: update_player(&state.player, &input, dt),
        ..state
    }
}

fn update_player(player: &Player, input: &InputState, dt: f32) -> Player {
    const ROT_SPEED: f32 = 150.0;
    const ACCELERATION: f32 = 60.0;

    let new_position = player.position + player.velocity * dt;

    let new_velocity = if input.thrust {
        player.velocity + Vec2::from_angle(player.angle.to_radians()) * ACCELERATION * dt
    } else {
        player.velocity
    };

    let mut new_angle = player.angle;
    if input.left {
        new_angle -= ROT_SPEED * dt
    };
    if input.right {
        new_angle += ROT_SPEED * dt
    };

    Player {
        position: new_position,
        angle: new_angle,
        velocity: new_velocity,
    }
}
