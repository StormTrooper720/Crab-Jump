use macroquad::prelude::*;

mod lib {
    pub mod globals;
    pub mod player;
    pub mod obstacle;
}

fn window_config() -> Conf {
    Conf {
        window_title: "Crab Jump".to_owned(),
        window_width: lib::globals::WIDTH,
        window_height: lib::globals::HEIGHT,
        fullscreen: false,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_config)]
async fn main() {
    let mut game_state = lib::globals::GameState::Game;

    let mut player = lib::player::Player::new().await;
    let mut obstacles: Vec<lib::obstacle::Obstacle> = Vec::new();
    let mut time_until_obstacle = get_time();

    let mut score: i32 = 0;

    loop {
        clear_background(WHITE);

        match game_state {
            lib::globals::GameState::Menu  => menu_loop(),
            lib::globals::GameState::Game  => game_loop(&mut game_state, &mut player, &mut obstacles, &mut time_until_obstacle, &mut score).await,
            lib::globals::GameState::Pause => pause_loop(),
            lib::globals::GameState::Dead  => dead_loop(&mut game_state, &mut player, &mut obstacles),
        }

        next_frame().await;
    }
}

fn menu_loop() {

}

async fn game_loop(
    game_state: &mut lib::globals::GameState,
    player: &mut lib::player::Player,
    obstacles: &mut Vec<lib::obstacle::Obstacle>,
    time_until_obstacle: &mut f64,
    score: &mut i32,
) {
    player.collision();
    player.update(get_frame_time());

    for i in 1..obstacles.len() {
        let result = &obstacles[i - 1].update(get_frame_time());
        if *result {
            obstacles.remove(i - 1);
            *score += 1;
        }
        
        let result = obstacles[i - 1].collision(player);
        if result {
            player.update(get_frame_time());
            *game_state = lib::globals::GameState::Dead;
        }
    }

    if get_time() >= *time_until_obstacle {
        let obstacle = lib::obstacle::Obstacle::new().await;
        obstacles.push(obstacle);
        *time_until_obstacle = get_time() + lib::globals::OBSTACLE_SPAWN_TIME + rand::gen_range(0, 2) as f64;
    }

    let score_string: String = score.to_string();
    draw_text(&score_string, 0.0, 25.0, 50.0, BLACK);

    for obstacle in obstacles {
        obstacle.draw();
    }
    player.draw(game_state);
    draw_rectangle(0f32, lib::globals::HEIGHT as f32 - 100f32, lib::globals::WIDTH as f32, 100f32, BLACK);
}

fn pause_loop() {

}

fn dead_loop(
    game_state: &mut lib::globals::GameState,
    player: &mut lib::player::Player,
    obstacles: &mut Vec<lib::obstacle::Obstacle>,
) {
    for obstacle in obstacles {
        obstacle.draw();
    }
    player.draw(game_state);
    draw_rectangle(0f32, lib::globals::HEIGHT as f32 - 100f32, lib::globals::WIDTH as f32, 100f32, BLACK);
}