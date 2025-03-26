use macroquad::prelude::*;
const PLAYER_SIZE: Vec2 = Vec2::new(150.0, 40.0);
const PLAYER_SPEED: f32 = 700.0;
const BLOCK_SIZE: Vec2 = Vec2::new(120.0, 40.0);
const BALL_SIZE: Vec2 = Vec2::new(20.0, 20.0);
const BALL_SPEED: f32 = 400.0;
pub enum GameState {
    Menu,
    Game,
    Completed,
    GameOver,
}
// Defines the Player
struct Player {
    rect: Rect,
}
impl Player {
    pub fn new() -> Self {
        Self {
            rect: Rect::new(screen_width() / 2.0, screen_height() - 100.0, PLAYER_SIZE.x, PLAYER_SIZE.y),
        }
    }
    pub fn update(&mut self, dt: f32) {
        let move_x = match (is_key_down(KeyCode::Left), is_key_down(KeyCode::Right)) {
            (true, false) => -10.0,
            (false, true) => 10.0,
            _ => 0.0,
        };
        self.rect.x += move_x * dt * PLAYER_SPEED;
        if self.rect.x < 0.0 {
            self.rect.x = 0.0;
        }
        if self.rect.x + self.rect.w > screen_width() {
            self.rect.x = screen_width() - self.rect.w;
        }
    }
    pub fn draw(&self) {
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, BLACK);
    }
}
#[derive(PartialEq)]
pub enum BlockType {
    Regular,
    SpawnBallOnDeath, // Block spawns a ball when removed
}
struct Block {
    rect: Rect,
    lives: u8,
    block_type: BlockType
}
impl Block {
    pub fn new(pos: Vec2, block_type: BlockType) -> Self {
            Self {
                rect: Rect::new(pos.x, pos.y, BLOCK_SIZE.x, BLOCK_SIZE.y),
                lives: 2,
                block_type,
            }
        }
    pub fn draw(&self) {
        let color = match self.block_type {
            BlockType::Regular =>
                match self.lives {
                2 => RED,
                _ => ORANGE
            },
            BlockType::SpawnBallOnDeath => GREEN,
        };
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, color);
    }
}
fn init_blocks(blocks: &mut Vec<Block>) {
    let (width, height) = (6, 6);
    let padding = 5.0;
    let total_block_size = BLOCK_SIZE + vec2(padding, padding);
    let board_start_pos = vec2((screen_width() - total_block_size.x * width as f32) / 2.0 , 50.0);
    for i in 0..width * height {
        let block_x = (i % width) as f32 * total_block_size.x;
        let block_y = (i / width) as f32 * total_block_size.y;
        blocks.push(Block::new(board_start_pos + vec2(block_x, block_y), BlockType::Regular));
    }
    for _ in 0..3 {
        let rand_index = rand::gen_range(0, blocks.len());
        blocks[rand_index].block_type = BlockType::SpawnBallOnDeath;
    }
}
// Defines the ball
struct Ball {
    rect: Rect,
    speed: Vec2,
}
impl Ball {
    fn new(pos: Vec2) -> Self {
        Self {
            rect: Rect::new(pos.x, pos.y, BALL_SIZE.x, BALL_SIZE.y),
            speed: vec2(rand::gen_range(-1.0, 1.0), 1.0).normalize(),
        }
    }
    pub fn update(&mut self, dt: f32) {
        self.rect.x += self.speed.x * dt * BALL_SPEED;
        self.rect.y += self.speed.y * dt * BALL_SPEED;
        if self.rect.x < 0.0 {
            self.speed.x = 1.0
        }
        if self.rect.x > screen_width() - self.rect.w {
            self.speed.x = -1.0
        }
        if self.rect.y < 0.0 {
            self.speed.y = 1.0
        }
    }
    pub fn draw(&self) {
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, DARKGRAY);
    }
}
fn collision_detection(a: &mut Rect, speed: &mut Vec2, b: Rect) -> bool {
    let intersection = match a.intersect(b) {
        Some(intersection) => intersection,
        None => return false,
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
    blocks: &mut Vec<Block>,
    balls: &mut Vec<Ball>,
    player: &mut Player,
) {
    *player = Player::new();
    *score = 0;
    *player_lives = 5;
    blocks.clear();
    balls.clear();
    init_blocks(blocks);
}
#[macroquad::main("Breakout")]
async fn main() {
    let font = load_ttf_font("font/Brexon-Regular.ttf").await.unwrap();
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
                let text = "Press SPACE to start";
                let dims = measure_text(text, Some(&font), 30, 1.0);
                draw_text_ex(
                    text,
                    screen_width() / 2.0 - dims.width / 2.0,
                    screen_height() / 2.0 - dims.height / 2.0,
                    TextParams { font: Some(&font), font_size: 30, color: BLACK, ..Default::default() }
                );
                if is_key_pressed(KeyCode::Space) {
                    game_state = GameState::Game;
                    reset_game(&mut score, &mut player_lives, &mut blocks, &mut balls, &mut player);
                }
            },
            GameState::Game => {
                if is_key_pressed(KeyCode::Space) {
                    balls.push(Ball::new(vec2(player.rect.x + player.rect.w/2.0 - BALL_SIZE.x/2.0, player.rect.y - BALL_SIZE.y)));
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
                            let spawn_ball = block.lives == 1 && matches!(block.block_type, BlockType::SpawnBallOnDeath);
                            let ball_position = vec2(block.rect.x + block.rect.w/2.0 - BALL_SIZE.x/2.0, block.rect.y + block.rect.h/2.0);
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
                let text = "You won! Press ESC to go to menu or SPACE to restart";
                let dims = measure_text(text, Some(&font), 30, 1.0);
                draw_text_ex(
                    text,
                    screen_width() / 2.0 - dims.width / 2.0,
                    screen_height() / 2.0 - dims.height / 2.0,
                    TextParams { font: Some(&font), font_size: 30, color: BLACK, ..Default::default() }
                );
                if is_key_pressed(KeyCode::Space) {
                    game_state = GameState::Game;
                    reset_game(&mut score, &mut player_lives, &mut blocks, &mut balls, &mut player);
                }
                if is_key_pressed(KeyCode::Escape) {
                    game_state = GameState::Menu;
                }
            },
            GameState::GameOver => {
                let text = "Game Over!!";
                let dims = measure_text(text, Some(&font), 30, 1.0);
                draw_text_ex(
                    text,
                    screen_width() / 2.0 - dims.width / 2.0,
                    screen_height() / 2.0 - dims.height / 2.0,
                    TextParams { font: Some(&font), font_size: 30, color: BLACK, ..Default::default() }
                );
                if is_key_pressed(KeyCode::Space) {
                    game_state = GameState::Game;
                    reset_game(&mut score, &mut player_lives, &mut blocks, &mut balls, &mut player);
                }
                if is_key_pressed(KeyCode::Escape) {
                    game_state = GameState::Menu;
                }
            },
        }
        next_frame().await;
    }
}
