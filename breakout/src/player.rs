use macroquad::prelude::*;
const PLAYER_SIZE: Vec2 = Vec2::new(150.0, 40.0);
const PLAYER_SPEED: f32 = 700.0;
// Defines the Player
pub struct Player {
    pub rect: Rect,
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
