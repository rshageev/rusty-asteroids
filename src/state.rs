use macroquad::prelude::Vec2;
pub struct Player {
    pub position: Vec2,
    pub angle: f32,
    pub velocity: Vec2,
}

pub struct GameState {
    pub player: Player,
}

pub struct InputState {
    pub thrust: bool,
    pub left: bool,
    pub right: bool,
    pub shoot: bool,
}
