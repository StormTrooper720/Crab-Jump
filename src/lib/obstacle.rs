use macroquad::prelude::*;

use super::globals::*;
use super::player::Player;

pub struct Obstacle {
    rect: Rect,
    // texture: Texture2D,
}

impl Obstacle {
    pub async fn new() -> Self {
        let height: f32 = rand::gen_range(0, 50) as f32;
        Self {
            rect: Rect::new(
                OBSTACLE[0],
                OBSTACLE[1] - height,
                OBSTACLE[2],
                OBSTACLE[3] + height
            ),
            // texture: load_texture("res/normal.png").await.unwrap(),
        }
    }

    pub fn update(&mut self, dt: f32) -> bool {
        self.rect.x -= OBSTACLE_SPEED * dt;

        if self.rect.x + self.rect.w < 0f32 {
            return true;
        }
        return false;
    }

    pub fn collision(&mut self, player: &mut Player) -> bool {
        if self.rect.overlaps(&player.rect) {
            player.is_dead = true;
            return true;
        } else {
            return false;
        }
    }

    pub fn draw(&self) {
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, BLACK);
    }
}