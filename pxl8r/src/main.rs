use image::{GenericImageView, ImageReader};
use image::imageops::FilterType;
use std::path::Path;
use std::io::Write;
fn get_input(prompt: &str) -> String {
    print!("{}", prompt);
    std::io::stdout().flush().unwrap();
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}
fn main() {
    let image_path = get_input("Enter the path to the image: ");
    let image = ImageReader::open(image_path).unwrap().decode().unwrap();
    let (original_width, original_height) = image.dimensions();
    let width = get_input("Enter the width: ").trim().parse().unwrap();
    let height = get_input("Enter the height: ").trim().parse().unwrap();
    let image = image.resize(width, height, FilterType::Nearest);
    let new_path = get_input("Enter the path to save the new image: ");
    let new_image = image.resize(original_width, original_height, FilterType::Nearest);
    new_image.save(new_path).unwrap();
    println!("New image successfully created");
}
