mod edit_options;
mod palette_mapping;
mod dither_matrices;
use edit_options::*;
use image::ImageReader;
use anyhow::Context;
fn main() -> anyhow::Result<()> {
    let image_path = get_input("Enter the path to the image: ");
    let image = ImageReader::open(&image_path)
        .context(format!("Failed to open image: {}", image_path))?
        .decode()
        .context(format!("Failed to decode image: {}", image_path))?;
    let rgb_image = image.to_rgb8();
    let (width, height) = rgb_image.dimensions();
    let edit_options = get_input("Enter the edit options: ");
    let rgb_image = match edit_options.as_str() {
        "resize" => resize(&rgb_image, width, height)?,
        "dither" => {
                    let mode = get_input("Choose dither mode: (basic/full): ");
                    let dither_mode = match mode.as_str() {
                        "basic" => DitherMode::Basic,
                        "full" => DitherMode::Full,
                        _ => {
                            println!("Invalid option. Defaulting to 'basic'.");
                            DitherMode::Basic
                        }
                    };
                    dither(&rgb_image, dither_mode, width, height)?
                },
        "crt" => crt_mode(&rgb_image, width, height)?,
        _ => rgb_image
    };
    let output_path = get_input("Enter the output path: ");
    let output_image = image::RgbImage::from_vec(width, height, rgb_image.into_vec())
        .context("Failed to create output image")?;
    output_image.save(&output_path).context(format!("Failed to save image: {}", output_path))?;
    println!("New image successfully created");
    Ok(())
}
