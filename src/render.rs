use macroquad::prelude::*;

pub fn draw_shape(shape: &[Vec2], pos: Vec2, angle: f32, scale: f32) {
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

fn transform_point(pos: Vec2, offset:Vec2, angle: f32, scale: f32) -> Vec2 {
    Vec2::from_angle(angle).rotate(pos) * scale + offset
}
