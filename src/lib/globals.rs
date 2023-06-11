use macroquad::prelude::*;

// screen dimensions
pub const HEIGHT: i32 = 720;
pub const WIDTH: i32 = 1280;

// game variables
pub const GRAVITY: f32 = 20f32;
pub const GROUND_FRICTION: f32 = 20.0;

// player variables
pub const PLAYER: [f32; 4] = [100f32, 100f32, 100f32, 75f32];
pub const PLAYER_SPEED: Vec2 = Vec2::from_array([50f32, 100f32]);
pub const MAX_PLAYER_SPEED: Vec2 = Vec2::from_array([1000f32, 0f32]); // y speed doesn't matter
pub const PLAYER_JUMP_THRESHOLD: f32 = 50f32; // pixels

// obstacle variables
pub const OBSTACLE: [f32; 4] = [WIDTH as f32, HEIGHT as f32 - 200f32, 50f32, 100f32];
pub const OBSTACLE_SPAWN_TIME: f64 = 1f64;
pub const OBSTACLE_SPEED: f32 = 500f32;


pub enum GameState {
    Menu,
    Game,
    Pause,
    Dead,
}