use image::ImageReader;
use image::imageops::FilterType;
use std::path::Path;
use std::io::Write;
enum ImageOptions {
    Resize,
} impl ImageOptions {
    fn edit_option(&self) -> ImageOptions {
            use ImageOptions::*;
            match self {
                Resize => Resize,
            }
        }
}
fn get_input(prompt: &str) -> String {
    print!("{}", prompt);
    std::io::stdout().flush().unwrap();
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}
fn main() {
    let image = ImageReader::open(Path::new("dnd_profile_pic.webp")).unwrap().decode().unwrap();
    let image = image.resize(200, 200, FilterType::Nearest);
    let new_image = image.resize(1024, 1024, FilterType::Nearest);
    new_image.save("dnd_profile_pic_resized.png").unwrap();
}
