use macroquad::prelude::*;
use crate::state::*;

pub fn draw(state: &GameState) {
    clear_background(BLACK);
    draw_player(&state.player);
}


fn draw_player(player: &Player) {
    const PLAYER_SHAPE: [Vec2; 4] = [
        Vec2 { x: 1.0, y: 0.0 },
        Vec2 { x: -0.5, y: -0.6 },
        Vec2 { x: -0.2, y: 0.0 },
        Vec2 { x: -0.5, y: 0.6 },
    ];
    draw_shape(&PLAYER_SHAPE, player.position, player.angle, 30.0);
}

fn draw_shape(shape: &[Vec2], pos: Vec2, angle: f32, scale: f32) {
    let points: Vec<Vec2> = shape
        .iter()
        .map(|pt| transform_point(*pt, pos, angle, scale))
        .collect();

    for i in 0..points.len() {
        let start = points[i];
        let end = points[(i + 1) % points.len()];
        draw_line(start.x, start.y, end.x, end.y, 1.0, WHITE);
    }
}

fn transform_point(pos: Vec2, offset: Vec2, angle: f32, scale: f32) -> Vec2 {
    Vec2::from_angle(angle.to_radians()).rotate(pos) * scale + offset
}
