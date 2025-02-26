use crate::dither_matrices::dither_matrices::get_dithering_matrices;
use crate::palette_mapping::{palette_mapping::*, palettes::get_palette};
use image::{DynamicImage, RgbImage};
use image::imageops::FilterType::Nearest;
use rayon::prelude::*;
use std::io::Write;
use anyhow::Context;
pub enum DitherMode {
    Basic,
    Full
}
fn scanlines(mut image: RgbImage, width: u32) -> RgbImage {
    let buffer = image.as_mut();
    buffer
        .par_chunks_exact_mut((width * 3) as usize * 2) // Process 2 rows at a time
        .for_each(|rows| {
            for x in (0..(width * 3) as usize).step_by(3) {
                rows[x] = (rows[x] as f32 * 0.5) as u8;
                rows[x + 1] = (rows[x + 1] as f32 * 0.5) as u8;
                rows[x + 2] = (rows[x + 2] as f32 * 0.5) as u8;
            }
        });
    image
}
fn apply_chromatic_aberration(image: &mut RgbImage, shift: i32, width: u32, height: u32) -> RgbImage {
    let temp = image.clone();
    for y in 0..height {
        for x in 0..width {
            let r_offset = (x as i32).saturating_sub(shift).max(0) as u32;
            let b_offset = (x as i32).saturating_add(shift).min(width as i32 - 1) as u32;
            let r_pixel = temp.get_pixel(r_offset, y);
            let b_pixel = temp.get_pixel(b_offset, y);
            let g_pixel = temp.get_pixel(x, y);
            image.put_pixel(x, y, image::Rgb([r_pixel[0], g_pixel[1], b_pixel[2]]));
        }
    }
    image.clone() // Return modified image
}
fn apply_crt_warp(image: &mut RgbImage, warp_strength: f32, width: u32, height: u32) -> RgbImage {
    let temp = image.clone();
    for y in 0..height {
        for x in 0..width {
            let offset_x = ((x as f32 - width as f32 / 2.0) * warp_strength).sin() * 5.0;
            let offset_y = ((y as f32 - height as f32 / 2.0) * warp_strength).sin() * 5.0;
            let src_x = (x as f32 + offset_x).clamp(0.0, width as f32 - 1.0) as u32;
            let src_y = (y as f32 + offset_y).clamp(0.0, height as f32 - 1.0) as u32;
            *image.get_pixel_mut(x, y) = *temp.get_pixel(src_x, src_y);
        }
    }
    image.clone() // Return a new image
}
pub fn crt_mode(image: &RgbImage, width: u32, height: u32) -> anyhow::Result<RgbImage> {
    let rgb_image = scanlines(image.clone(), width);
    let shift = get_input("Enter the shift value: ").parse::<i32>()
        .context("Failed to parse shift value as integer")?;
    let mut rgb_image = apply_chromatic_aberration(&mut rgb_image.clone(), shift, width, height);
    let warp_strength = get_input("Choose warp strength: ").parse::<f32>()
        .context("Failed to parse warp strength as float")?;
    let final_image = apply_crt_warp(&mut rgb_image, warp_strength, width, height);
    Ok(final_image)
}
pub fn dither(image: &RgbImage, mode: DitherMode, width: u32, height: u32) -> anyhow::Result<RgbImage> {
    let mut buffer = image.clone().into_raw();
    let palette_option = get_palette(get_input("Choose a palette: ").trim());
    let result = match mode {
        DitherMode::Basic => {
            buffer.par_chunks_mut(width as usize * 3).enumerate().for_each(|(_y, row)| {
                for x in 0..width as usize {
                    let i = x * 3;
                    let pixel = Color {
                        r: row[i],
                        g: row[i + 1],
                        b: row[i + 2],
                    };
                    let new_color = map_to_palette1(pixel, &palette_option);
                    row[i] = new_color.r;
                    row[i + 1] = new_color.g;
                    row[i + 2] = new_color.b;
                }
            });
            RgbImage::from_raw(width, height, buffer)
                .ok_or_else(|| anyhow::anyhow!("Failed to create image from buffer"))?
        }
        DitherMode::Full => {
            let dithering_matrices = get_dithering_matrices();
            let selected_dither = get_input("Choose dithering type: ");
            let dither_matrix = dithering_matrices.get(selected_dither.trim())
                .unwrap_or(&dithering_matrices["floyd"]); // Default to Floyd-Steinberg
            for y in 0..height {
                for x in 0..width {
                    let i = ((y * width + x) * 3) as usize;
                    let pixel = Color {
                        r: buffer[i],
                        g: buffer[i + 1],
                        b: buffer[i + 2],
                    };
                    let (new_color, qe) = map_to_palette2(pixel, &palette_option);
                    buffer[i] = new_color.r;
                    buffer[i + 1] = new_color.g;
                    buffer[i + 2] = new_color.b;
                    // Dithering using dither_matrix
                    for dy in 0..2 {
                        for dx in -1_i32..=1 {
                            let nx = x as i32 + dx;
                            let ny = y as i32 + dy;
                            if nx < 0 || nx >= width as i32 || ny < 0 || ny >= height as i32 {
                                continue;
                            }
                            let ni = ((ny as u32 * width + nx as u32) * 3) as usize;
                            let diffusion_factor = dither_matrix[dy as usize][(dx + 1) as usize];
                            buffer[ni] = (buffer[ni] as f32 + qe.r * diffusion_factor)
                                .round().clamp(0.0, 255.0) as u8;
                            buffer[ni + 1] = (buffer[ni + 1] as f32 + qe.g * diffusion_factor)
                                .round().clamp(0.0, 255.0) as u8;
                            buffer[ni + 2] = (buffer[ni + 2] as f32 + qe.b * diffusion_factor)
                                .round().clamp(0.0, 255.0) as u8;
                        }
                    }
                }
            }
            RgbImage::from_raw(width, height, buffer)
                .ok_or_else(|| anyhow::anyhow!("Failed to create image from buffer"))?
        }
    };
    Ok(result)
}
pub fn get_input(prompt: &str) -> String {
    print!("{}", prompt);
    std::io::stdout().flush().unwrap();
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}
pub fn resize(image: &RgbImage, width: u32, height: u32) -> anyhow::Result<RgbImage> {
    let temp_width = get_input("Enter the width: ").trim().parse()?;
    let temp_height = get_input("Enter the height: ").trim().parse()?;
    let dynamic_image = DynamicImage::ImageRgb8(image.clone());
    let resized = dynamic_image.resize(temp_width, temp_height, Nearest);
    let back_to_original = resized.resize(width, height, Nearest);
    let new_image = back_to_original.to_rgb8();
    Ok(new_image)
}
