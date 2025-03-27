use macroquad::prelude::*;

pub const BALL_SIZE: Vec2 = Vec2::new(20.0, 20.0);
const BALL_SPEED: f32 = 400.0;
// Defines the ball
pub struct Ball {
    pub rect: Rect,
    pub speed: Vec2,
}
impl Ball {
    pub fn new(pos: Vec2) -> Self {
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
