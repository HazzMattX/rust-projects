use image::{DynamicImage, GenericImageView, ImageReader};
use image::imageops::FilterType::Nearest;
use std::path::Path;
use std::io::Write;
use std::f32::INFINITY;
use anyhow::Context;
use image::ExtendedColorType;
struct Color {
    r: u8,
    g: u8,
    b: u8,
}
struct QuantizationError {
    r: f32,
    g: f32,
    b: f32,
}
impl From<u32> for Color {
    fn from(value: u32) -> Self {
        Color {
            b: (value & 0xFF) as u8,
            g: ((value >> 8) & 0xFF) as u8,
            r: ((value >> 16) & 0xFF) as u8,
        }
    }
}
impl From<&[u8; 3]> for Color {
    fn from(value: &[u8; 3]) -> Self {
        Color {
            b: value[0],
            g: value[1],
            r: value[2],
        }
    }
}
fn map_to_palette(orig: Color, palette: &Vec<Color>) -> (Color, QuantizationError) {
    let mut closest = &palette[0];
    let mut min_distance = INFINITY;
    for color in palette {
        let distance = (orig.r as f32 - color.r as f32).powi(2) +
                       (orig.g as f32 - color.g as f32).powi(2) +
                       (orig.b as f32 - color.b as f32).powi(2);
        if distance < min_distance {
            closest = color;
            min_distance = distance;
        }
    }
    let qe = QuantizationError {
        r: orig.r as f32 - closest.r as f32,
        g: orig.g as f32 - closest.g as f32,
        b: orig.b as f32 - closest.b as f32,
    };
    (Color { r: closest.r, g: closest.g, b: closest.b }, qe)
}
fn dither(image: &image::DynamicImage) -> anyhow::Result<DynamicImage> {
    let image = image.to_rgb8();
    let (width, height) = image.dimensions();
    let mut buffer = image.into_raw();
    let palette = vec![
        Color::from(0x000000),
        Color::from(0xf50f27),
        Color::from(0xf56c3f),
        Color::from(0xffffff),
        Color::from(0x004785),
        Color::from(0x0395b1),
        Color::from(0x36eb8f),
        Color::from(0xabffa8),
        Color::from(0x8f2700),
        Color::from(0xff8f15),
        Color::from(0xffec3c),
        Color::from(0xe3fdaf),
        Color::from(0x2d006a),
        Color::from(0x0003f5),
        Color::from(0x6ed1f5),
        Color::from(0xadffca),
    ];
    let floyd_steinberg: [[f32; 3]; 2] = [
        [0.0, 7.0 / 16.0, 0.0],
        [3.0 / 16.0, 5.0 / 16.0, 1.0 / 16.0]
    ];
    for y in 0..height {
        for x in 0..width {
            let i = ((y * width + x) * 3) as usize;
            let pixel = Color {
                r: buffer[i],
                g: buffer[i + 1],
                b: buffer[i + 2],
            };
            let (new_color, qe) = map_to_palette(pixel, &palette);
            buffer[i] = new_color.r;
            buffer[i + 1] = new_color.g;
            buffer[i + 2] = new_color.b;
            // Floyd-Steinberg Dithering
            for dy in 0..2 {
                for dx in -1..=1 {
                    let nx = x as isize + dx;
                    let ny = y as isize + dy;
                    if nx < 0 || nx >= width as isize || ny < 0 || ny >= height as isize {
                        continue;
                    }
                    let ni = ((ny as u32 * width + nx as u32) * 3) as usize;
                    let diffusion_factor = floyd_steinberg[dy as usize][(dx + 1) as usize];
                    buffer[ni] = ((buffer[ni] as f32 + qe.r * diffusion_factor).round().clamp(0.0, 255.0)) as u8;
                    buffer[ni + 1] = ((buffer[ni + 1] as f32 + qe.g * diffusion_factor).round().clamp(0.0, 255.0)) as u8;
                    buffer[ni + 2] = ((buffer[ni + 2] as f32 + qe.b * diffusion_factor).round().clamp(0.0, 255.0)) as u8;
                }
            }
        }
    }

    let output = get_input("Enter the output path: ");
    image::save_buffer(&output, &buffer, width, height, ExtendedColorType::Rgb8)
        .context(format!("Failed to save output image: {}", output))?;

    let img = DynamicImage::ImageRgb8(
        image::RgbImage::from_raw(width, height, buffer)
            .expect("Failed to create image from buffer")
    );
    Ok(img)
}
fn get_input(prompt: &str) -> String {
    print!("{}", prompt);
    std::io::stdout().flush().unwrap();
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}
fn resize(image: &image::DynamicImage) -> anyhow::Result<image::DynamicImage> {
    let (original_width, original_height) = image.dimensions();
    let width = get_input("Enter the width: ").trim().parse().unwrap();
    let height = get_input("Enter the height: ").trim().parse().unwrap();
    let image = image.resize(width, height, Nearest);
    let new_image = image.resize(original_width, original_height, Nearest);
    let new_path = get_input("Enter the path to save the new image: ");
    new_image.save(&new_path)
        .context(format!("Failed to save resized image: {}", new_path))?;
    Ok(new_image)
}
fn main() -> anyhow::Result<()> {
    let image_path = get_input("Enter the path to the image: ");
    let image = ImageReader::open(&image_path)
        .context(format!("Failed to open image: {}", image_path))?
        .decode()
        .context(format!("Failed to decode image: {}", image_path))?;

    let edit_options = get_input("Enter the edit options: ");
    match edit_options.as_str() {
        "resize" => resize(&image),
        "dither" => dither(&image),
        _ => Ok(image)
    }?;
    println!("New image successfully created");
    Ok(())
}
