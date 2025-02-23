use crate::palette_mapping::palette_mapping::*;
use crate::palette_mapping::palettes::*;
use image::{DynamicImage, GenericImageView, ExtendedColorType};
use image::imageops::FilterType::Nearest;
use std::io::Write;
use anyhow::Context;
pub enum DitherMode {
    Basic,
    Full
}
pub fn dither(image: &image::DynamicImage, mode: DitherMode) -> anyhow::Result<DynamicImage> {
    let image = image.to_rgb8();
    let (width, height) = image.dimensions();
    let mut buffer = image.into_raw();
    let floyd_steinberg: [[f32; 3]; 2] = [
        [0.0, 7.0 / 16.0, 0.0],
        [3.0 / 16.0, 5.0 / 16.0, 1.0 / 16.0]
    ];
    let palette_choice = get_input("Choose palette: ");
    let palette = match palette_choice.trim() {
        "1" => &PALETTE1,
        "2" => &PALETTE2,
        "3" => &PALETTE3,
        _ => &PALETTE1,
    };
    for y in 0..height {
        for x in 0..width {
            let i = ((y * width + x) * 3) as usize;
            let pixel = Color {
                r: buffer[i],
                g: buffer[i + 1],
                b: buffer[i + 2],
            };
            match mode {
                DitherMode::Basic => {
                                let new_color = map_to_palette1(pixel, &palette);
                                buffer[i] = new_color.r;
                                buffer[i + 1] = new_color.g;
                                buffer[i + 2] = new_color.b;
                            },
                DitherMode::Full => {
                                let (new_color, qe) = map_to_palette2(pixel, &palette);
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
                                        buffer[ni] = ((buffer[ni] as f32 + qe.r * diffusion_factor)
                                            .round().clamp(0.0, 255.0)) as u8;
                                        buffer[ni + 1] = ((buffer[ni + 1] as f32 + qe.g * diffusion_factor)
                                            .round().clamp(0.0, 255.0)) as u8;
                                        buffer[ni + 2] = ((buffer[ni + 2] as f32 + qe.b * diffusion_factor)
                                            .round().clamp(0.0, 255.0)) as u8;
                                    }
                                }
                            }
            }
        }
    }
    let output = get_input("Enter the output path: ");
    image::save_buffer(&output, &buffer, width, height, ExtendedColorType::Rgb8)
        .context(format!("Failed to save output image: {}", output))?;

    Ok(DynamicImage::ImageRgb8(image::RgbImage::from_raw(width, height, buffer).unwrap()))
}
pub fn get_input(prompt: &str) -> String {
    print!("{}", prompt);
    std::io::stdout().flush().unwrap();
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}
pub fn resize(image: &image::DynamicImage) -> anyhow::Result<DynamicImage> {
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
