use macroquad::prelude::*;

use super::globals::*;

pub struct Player {
    pub rect: Rect,
    current_texture: Texture2D,
    alive_texture: Texture2D,
    dead_texture: Texture2D,
    jump_texture: Texture2D,
    velocity: Vec2,
    touching_ground: bool,
    in_threshold: bool,
    allow_jump: bool,
    inital_jump_boost: bool,
    is_dead: bool,
}

impl Player {
    pub async fn new() -> Self {
        Self {
            rect: Rect::new(
                PLAYER[0],
                PLAYER[1],
                PLAYER[2],
                PLAYER[3]
            ),
            current_texture: load_texture("res/normal.png").await.unwrap(),
            alive_texture: load_texture("res/normal.png").await.unwrap(),
            dead_texture: load_texture("res/died.png").await.unwrap(),
            jump_texture: load_texture("res/jump.png").await.unwrap(),
            velocity: Vec2::from_array([0f32, 0f32]),
            touching_ground: false,
            in_threshold: false,
            allow_jump: false,
            inital_jump_boost: false,
            is_dead: true,
        }
    }

    pub fn update(&mut self, dt: f32) {
        // x movement
        /*
        if is_key_down(KeyCode::A) {
            if self.velocity.x > -MAX_PLAYER_SPEED.x {
                self.velocity.x += -PLAYER_SPEED.x;
            }
        } else if is_key_down(KeyCode::D) {
            if self.velocity.x < MAX_PLAYER_SPEED.x {
                self.velocity.x += PLAYER_SPEED.x;
            }
        } else {
            if self.velocity.x > 0f32 {
                self.velocity.x -= GROUND_FRICTION;
            } else if self.velocity.x < 0f32 {
                self.velocity.x += GROUND_FRICTION;
            }
        }
        */

        // y movement
        if self.touching_ground {
            self.velocity.y = 0f32;
        } else {
            self.velocity.y += GRAVITY;
        }

        if self.in_threshold {
            if (is_key_down(KeyCode::W) || is_key_down(KeyCode::Space)) && self.allow_jump {
                if self.inital_jump_boost {
                    self.velocity.y = -(PLAYER_SPEED.y * 3f32);
                    self.inital_jump_boost = false;
                }
                self.velocity.y += -PLAYER_SPEED.y;
            } else {
                self.allow_jump = false;
            }
        }

        // update play position
        self.rect.x += self.velocity.x * dt;
        self.rect.y += self.velocity.y * dt;

        // fix positions off screen
        if self.rect.x + self.rect.w > WIDTH as f32 {
            self.rect.x = WIDTH as f32 - self.rect.w;
        } else if self.rect.x < 0f32 {
            self.rect.x = 0f32;
        }
    }

    pub fn collision(&mut self) {
        // replace with ground sprite
        if self.rect.y + self.rect.h > HEIGHT as f32 - 100f32 {
            self.touching_ground = true;
            self.in_threshold = true;
            self.allow_jump = true;
            self.inital_jump_boost = true;
        } else if self.rect.y + self.rect.h > HEIGHT as f32 - 100f32 - PLAYER_JUMP_THRESHOLD {
            self.touching_ground = false;
            self.in_threshold = true;
        } else {
            self.touching_ground = false;
            self.in_threshold = false;
        }
    }

    pub fn draw(&mut self, game_state: &mut GameState) {
        // draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, BLACK);
        let size: Vec2 = Vec2::from_array([self.rect.w, self.rect.h]);
        
        match *game_state {
            GameState::Dead => self.current_texture = self.dead_texture,
            _=> self.current_texture = self.alive_texture,
        }

        if !self.touching_ground {
            self.current_texture = self.jump_texture;
        }

        draw_texture_ex(self.current_texture, self.rect.x, self.rect.y, WHITE, DrawTextureParams {dest_size: Some(size), ..Default::default()});
    }
}