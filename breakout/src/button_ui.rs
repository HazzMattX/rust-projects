use macroquad::prelude::*;
// ui.rs
pub struct CustomButton {
    rect: Rect,
    text: String,
    font: Font,
    is_hovered: bool,
    is_clicked: bool,
    normal_color: Color,
    hover_color: Color,
    text_color: Color,
    font_size: u16,
}
impl CustomButton {
    pub fn new(x: f32, y: f32, width: f32, height: f32, text: &str, font: Font) -> Self {
        Self {
            rect: Rect::new(x, y, width, height),
            text: text.to_string(),
            font,
            is_hovered: false,
            is_clicked: false,
            normal_color: GRAY,
            hover_color: LIGHTGRAY,
            text_color: BLACK,
            font_size: 30,
        }
    }
    pub fn set_colors(&mut self, normal: Color, hover: Color, text: Color) {
        self.normal_color = normal;
        self.hover_color = hover;
        self.text_color = text;
    }
    pub fn update(&mut self) {
        let mouse_pos = mouse_position();
        let mouse_point = Vec2::new(mouse_pos.0, mouse_pos.1);

        self.is_hovered = self.rect.contains(mouse_point);
        self.is_clicked = self.is_hovered && is_mouse_button_pressed(MouseButton::Left);
    }
    pub fn draw(&self) {
        // Draw button background
        let color = if self.is_hovered { self.hover_color } else { self.normal_color };
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, color);
        // Center and draw text
        let text_dims = measure_text(&self.text, Some(&self.font), self.font_size, 1.0);
        let text_x = self.rect.x + (self.rect.w - text_dims.width) / 2.0;
        let text_y = self.rect.y + (self.rect.h + text_dims.height) / 2.0;
        draw_text_ex(
            &self.text,
            text_x,
            text_y,
            TextParams {
                font: Some(&self.font),
                font_size: self.font_size,
                color: self.text_color,
                ..Default::default()
            }
        );
    }
    pub fn is_clicked(&self) -> bool {
        self.is_clicked
    }
}
