use crate::state::*;
use macroquad::prelude::*;

pub fn draw(state: &GameState, input: &InputState) {
    clear_background(BLACK);
    draw_player(&state.player, input.thrust);
}

fn draw_player(player: &Player, thrust: bool) {
    const PLAYER_SHAPE: [Vec2; 4] = [
        Vec2 { x: 1.0, y: 0.0 },
        Vec2 { x: -0.5, y: -0.6 },
        Vec2 { x: -0.2, y: 0.0 },
        Vec2 { x: -0.5, y: 0.6 },
    ];
    draw_shape(&PLAYER_SHAPE, player.position, player.angle, 30.0, WHITE);

    const PLUME_SHAPE: [Vec2; 4] = [
        Vec2 { x: -0.3, y: 0.0 },
        Vec2 { x: -1.0, y: -0.3 },
        Vec2 { x: -0.8, y: 0.0 },
        Vec2 { x: -1.0, y: 0.3 },
    ];
    if thrust {
        draw_filled_shape(&PLUME_SHAPE, player.position, player.angle, 30.0, WHITE);
    }
}

fn draw_shape(shape: &[Vec2], pos: Vec2, angle: f32, scale: f32, color: Color) {
    let points = transform_points(shape, pos, angle, scale);
    for i in 0..points.len() {
        let start = points[i];
        let end = points[(i + 1) % points.len()];
        draw_line(start.x, start.y, end.x, end.y, 1.0, color);
    }
}

fn draw_filled_shape(shape: &[Vec2], pos: Vec2, angle: f32, scale: f32, color: Color) {
    let points = transform_points(shape, pos, angle, scale);
    for i in 1..(points.len() - 1) {
        draw_triangle(points[0], points[i], points[i + 1], color);
    }
}

fn transform_point(pos: Vec2, offset: Vec2, angle: f32, scale: f32) -> Vec2 {
    Vec2::from_angle(angle.to_radians()).rotate(pos) * scale + offset
}

fn transform_points(points: &[Vec2], offset: Vec2, angle: f32, scale: f32) -> Vec<Vec2> {
    points
        .iter()
        .map(|pt| transform_point(*pt, offset, angle, scale))
        .collect()
}
