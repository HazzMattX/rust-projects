use image::ImageReader;
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
    let width = get_input("Enter the width: ");
    let height = get_input("Enter the height: ");
    let image = image.resize(width.trim().parse().unwrap(), height.trim().parse().unwrap(), FilterType::Nearest);
    let new_path = get_input("Enter the path to save the new image: ");
    let new_image = image.resize(1024, 1024, FilterType::Nearest);
    new_image.save(new_path).unwrap();
    println!("New image successfully created");
}
