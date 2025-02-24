use crate::dither_matrices::dither_matrices::get_dithering_matrices;
use crate::palette_mapping::{palette_mapping::*, palettes::*,};
use image::{DynamicImage, ExtendedColorType, GenericImageView};
use image::imageops::FilterType::Nearest;
use rayon::prelude::*;
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
    let palette_choice = get_input("Choose palette: ");
    let palette = match palette_choice.trim() {
        "1" => &PALETTE1,
        "2" => &PALETTE2,
        "3" => &PALETTE3,
        _ => &PALETTE1,
    };
    let dithering_matrices = get_dithering_matrices();
    let selected_dither = get_input("Choose dithering type: ");
    let dither_matrix = dithering_matrices.get(selected_dither.trim())
        .unwrap_or(&dithering_matrices["floyd"]); // Default to Floyd-Steinberg
    if let DitherMode::Basic = mode {
        buffer.par_chunks_mut(width as usize * 3).enumerate().for_each(|(_y, row)| {
            for x in 0..width as usize {
                let i = x * 3;
                let pixel = Color {
                    r: row[i],
                    g: row[i + 1],
                    b: row[i + 2],
                };
                let new_color = map_to_palette1(pixel, &palette);
                row[i] = new_color.r;
                row[i + 1] = new_color.g;
                row[i + 2] = new_color.b;
            }
        });
    } else {
        for y in 0..height {
            for x in 0..width {
                let i = ((y * width + x) * 3) as usize;
                let pixel = Color {
                    r: buffer[i],
                    g: buffer[i + 1],
                    b: buffer[i + 2],
                };
                match mode {
                    DitherMode::Full => {
                        let dithering_matrices = get_dithering_matrices();
                        let selected_dither = get_input("Choose dithering type: ");
                        let dither_matrix = dithering_matrices.get(selected_dither.trim())
                            .unwrap_or(&dithering_matrices["floyd"]); // Default to Floyd-Steinberg
                        let (new_color, qe) = map_to_palette2(pixel, &palette);
                        buffer[i] = new_color.r;
                        buffer[i + 1] = new_color.g;
                        buffer[i + 2] = new_color.b;
                        // Floyd-Steinberg Dithering
                        for dy in 0..2 {
                            for dx in -1_i32..=1 {
                                let nx = x as i32 + dx;
                                let ny = y as i32 + dy as i32;
                                if nx < 0 || nx >= width as i32 || ny < 0 || ny >= height as i32 {
                                    continue;
                                }
                                let ni = ((ny as u32 * width + nx as u32) * 3) as usize;
                                let diffusion_factor = dither_matrix[dy as usize][(dx + 1) as usize];
                                buffer[ni] = ((buffer[ni] as f32 + qe.r * diffusion_factor)
                                    .round().clamp(0.0, 255.0)) as u8;
                                buffer[ni + 1] = ((buffer[ni + 1] as f32 + qe.g * diffusion_factor)
                                    .round().clamp(0.0, 255.0)) as u8;
                                buffer[ni + 2] = ((buffer[ni + 2] as f32 + qe.b * diffusion_factor)
                                    .round().clamp(0.0, 255.0)) as u8;
                            }
                        }
                    }
                    _ => {} // Basic mode is handled above
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
