use csv::Reader;
use image::ImageReader;
use macroquad::prelude::*;
#[macroquad::main("States Guessing Game")]
async fn main() {
    let image_reader = ImageReader::open("blank_states_img.png").unwrap();
    let (width, height) = image_reader.into_dimensions().unwrap();
    let image = load_texture("blank_states_img.png").await.unwrap();
    request_new_screen_size(width as f32, height as f32 + 29.0);
    loop {
        clear_background(WHITE);
        draw_texture(&image, 0.0, 0.0, WHITE);
        next_frame().await;
    }
}
