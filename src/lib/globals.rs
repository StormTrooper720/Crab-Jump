use macroquad::prelude::*;
use image;

// screen dimensions
pub const HEIGHT: i32 = 720;
pub const WIDTH: i32 = 1280;

// images
const NORMAL: &[u8] = include_bytes!("../../res/normal.png");
const JUMP: &[u8] = include_bytes!("../../res/jump.png");
const DIED: &[u8] = include_bytes!("../../res/died.png");

// game variables
pub const GRAVITY: f32 = 20f32;
// pub const GROUND_FRICTION: f32 = 20.0;

// player variables
pub const PLAYER: [f32; 4] = [100f32, 100f32, 100f32, 75f32];
pub const PLAYER_SPEED: Vec2 = Vec2::from_array([50f32, 100f32]);
// pub const MAX_PLAYER_SPEED: Vec2 = Vec2::from_array([1000f32, 0f32]); // y speed doesn't matter
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

pub fn get_image(name: String) -> Texture2D {
    let mut image;
    if name == "normal" {
        image = image::load_from_memory(NORMAL).expect("Failed to load image").to_rgba8();
    } else if name == "jump" {
        image = image::load_from_memory(JUMP).expect("Failed to load image").to_rgba8();
    } else if name == "died" {
        image = image::load_from_memory(DIED).expect("Failed to load image").to_rgba8();
    } else {
        image = image::load_from_memory(DIED).expect("Failed to load image").to_rgba8();
    }

    let (width, height) = image.dimensions();
    let width = width as u16;
    let height = height as u16;
    let texture = Texture2D::from_rgba8(width, height, &image);

    return texture;
}