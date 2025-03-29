mod player;
mod blocks;
mod ball;
mod button_ui;
use ball::{Ball, BALL_SIZE};
use blocks::*;
use button_ui::CustomButton;
use player::Player;
use macroquad::prelude::*;
enum GameState {
    Menu,
    Game,
    Completed,
    GameOver,
}
fn collision_detection(a: &mut Rect, speed: &mut Vec2, b: Rect) -> bool {
    let intersection = match a.intersect(b) {
        Some(intersection) => intersection,
        None => {
            return false
        },
    };
    let a_center = a.center();
    let b_center = b.center();
    let to = a_center - b_center;
    let to_signum = to.signum();
    match intersection.w > intersection.h {
        true => {
            // Fix the y-position adjustment
            if to_signum.y > 0.0 {
                a.y = b.y + b.h;
                speed.y = 1.0;
            } else {
                a.y = b.y - a.h;
                speed.y = -1.0;
            }
        }
        false => {
            // Fix the x-position adjustment
            if to_signum.x > 0.0 {
                a.x = b.x + b.w;
                speed.x = 1.0;
            } else {
                a.x = b.x - a.w;
                speed.x = -1.0;
            }
        }
    }
    true
}
fn reset_game(
    score: &mut i32,
    player_lives: &mut i32,
    blocks: &mut Vec<blocks::Block>,
    balls: &mut Vec<Ball>,
    player: &mut player::Player,
) {
    *player = player::Player::new();
    *score = 0;
    *player_lives = 5;
    blocks.clear();
    balls.clear();
    blocks::init_blocks(blocks);
}
#[macroquad::main("Breakout")]
async fn main() {
    let font = load_ttf_font("font/Brexon-Regular.ttf").await.unwrap();
    const BUTTON_SIZE: Vec2 = vec2(200.0, 50.0);
    request_new_screen_size(1000.0, 600.0);
    let mut game_state = GameState::Menu;
    let mut score = 0;
    let mut player_lives = 5;
    let mut player = Player::new();
    let mut blocks: Vec<Block> = Vec::new();
    let mut balls: Vec<Ball> = Vec::new();
    init_blocks(&mut blocks);
    balls.push(Ball::new(vec2(player.rect.x + player.rect.w/2.0 - BALL_SIZE.x/2.0, player.rect.y - BALL_SIZE.y)));
    loop {
        clear_background(WHITE);
        match game_state {
            GameState::Menu => {
                let text = "BREAKOUT";
                let dims = measure_text(text, Some(&font), 30, 1.0);
                draw_text_ex(
                    text,
                    screen_width() / 2.0 - dims.width / 2.0,
                    100.0,
                    TextParams { font: Some(&font), font_size: 70, color: BLACK, ..Default::default() }
                );
                let mut start_button = CustomButton::new(
                    350.0,
                    250.0,
                    BUTTON_SIZE.x,
                    BUTTON_SIZE.y,
                    "Start Game",
                    font.clone()
                );
                start_button.set_colors(Color::new(0.2, 0.6, 0.8, 1.0), // Normal color (blue)
                                        Color::new(0.3, 0.7, 0.9, 1.0), // Hover color (lighter blue)
                                        WHITE); // Text color
                start_button.update();
                start_button.draw();
                if start_button.is_clicked() {
                    game_state = GameState::Game;
                    reset_game(&mut score, &mut player_lives, &mut blocks, &mut balls, &mut player);
                }
            },
            GameState::Game => {
                if is_key_pressed(KeyCode::Space) {
                    balls.push(Ball::new(
                        vec2(
                            player.rect.x + player.rect.w/2.0 - BALL_SIZE.x/2.0, player.rect.y - BALL_SIZE.y)));
                }
                player.update(get_frame_time() / 4.0);
                for ball in balls.iter_mut() {
                    ball.update(get_frame_time());
                }
                let mut extra_balls: Vec<Ball> = vec![];
                for ball in balls.iter_mut() {
                    collision_detection(&mut ball.rect, &mut ball.speed, player.rect);
                    for block in blocks.iter_mut() {
                        if collision_detection(&mut ball.rect, &mut ball.speed, block.rect) {
                            let spawn_ball = block.lives == 1 && matches!(
                                block.block_type, BlockType::SpawnBallOnDeath);
                            let ball_position = vec2(
                                block.rect.x + block.rect.w/2.0 - BALL_SIZE.x/2.0, block.rect.y + block.rect.h/2.0);
                            block.lives -= 1;
                            score += 10;
                            if spawn_ball {
                                extra_balls.push(Ball::new(ball_position));
                            }
                        };
                    }
                }
                balls.append(&mut extra_balls);
                let balls_len = balls.len();
                let was_last_ball = balls_len == 1;
                balls.retain(|ball| ball.rect.y < screen_height() - 100.0);
                let removed_balls = balls_len - balls.len();
                if removed_balls > 0 && was_last_ball {
                    player_lives -= 1;
                    if player_lives == 0 {
                        game_state = GameState::GameOver;
                    }
                }
                if blocks.is_empty() {
                    game_state = GameState::Completed;
                }
                blocks.retain(|block| block.lives > 0);
                player.draw();
                for block in blocks.iter() {
                    block.draw();
                }
                for ball in balls.iter() {
                    ball.draw();
                }
                draw_text_ex(
                    &format!("Score: {}", score),
                    screen_width() / 2.0,
                    40.0,
                    TextParams { font: Some(&font), font_size: 30, color: BLACK, ..Default::default() }
                );
                draw_text_ex(
                    &format!("Lives: {}", player_lives),
                    30.0,
                    40.0,
                    TextParams { font: Some(&font), font_size: 30, color: BLACK, ..Default::default() }
                );
            },
            GameState::Completed => {
                let mut restart_button = CustomButton::new(
                    350.0,
                    250.0,
                    BUTTON_SIZE.x,
                    BUTTON_SIZE.y,
                    "Restart Game",
                    font.clone()
                );
                restart_button.set_colors(Color::new(0.2, 0.6, 0.8, 1.0), // Normal color (blue)
                                        Color::new(0.3, 0.7, 0.9, 1.0), // Hover color (lighter blue)
                                        WHITE); // Text color
                restart_button.update();
                restart_button.draw();
                if restart_button.is_clicked() {
                    game_state = GameState::Game;
                    reset_game(&mut score, &mut player_lives, &mut blocks, &mut balls, &mut player);
                }
                let mut quit_button = CustomButton::new(
                    350.0,
                    250.0,
                    BUTTON_SIZE.x,
                    BUTTON_SIZE.y,
                    "Quit Game",
                    font.clone()
                );
                quit_button.set_colors(Color::new(0.2, 0.6, 0.8, 1.0), // Normal color (blue)
                                        Color::new(0.3, 0.7, 0.9, 1.0), // Hover color (lighter blue)
                                        WHITE); // Text color
                quit_button.update();
                quit_button.draw();
                if quit_button.is_clicked() {
                    game_state = GameState::Game;
                    reset_game(&mut score, &mut player_lives, &mut blocks, &mut balls, &mut player);
                }
            },
            GameState::GameOver => {
                let text = "Game Over!!";
                let dims = measure_text(text, Some(&font), 30, 1.0);
                draw_text_ex(
                    text,
                    screen_width() / 2.0 - dims.width / 2.0,
                    100.0,
                    TextParams { font: Some(&font), font_size: 30, color: BLACK, ..Default::default() }
                );
                let mut restart_button = CustomButton::new(
                    350.0,
                    250.0,
                    BUTTON_SIZE.x,
                    BUTTON_SIZE.y,
                    "Restart Game",
                    font.clone()
                );
                restart_button.set_colors(Color::new(0.2, 0.6, 0.8, 1.0), // Normal color (blue)
                                        Color::new(0.3, 0.7, 0.9, 1.0), // Hover color (lighter blue)
                                        WHITE); // Text color
                restart_button.update();
                restart_button.draw();
                if restart_button.is_clicked() {
                    game_state = GameState::Game;
                    reset_game(&mut score, &mut player_lives, &mut blocks, &mut balls, &mut player);
                }
                let mut quit_button = CustomButton::new(
                    350.0,
                    250.0,
                    BUTTON_SIZE.x,
                    BUTTON_SIZE.y,
                    "Quit Game",
                    font.clone()
                );
                quit_button.set_colors(Color::new(0.2, 0.6, 0.8, 1.0), // Normal color (blue)
                                        Color::new(0.3, 0.7, 0.9, 1.0), // Hover color (lighter blue)
                                        WHITE); // Text color
                quit_button.update();
                quit_button.draw();
                if quit_button.is_clicked() {
                    game_state = GameState::Game;
                    reset_game(&mut score, &mut player_lives, &mut blocks, &mut balls, &mut player);
                }
            },
        }
        next_frame().await;
    }
}
