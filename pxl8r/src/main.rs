mod edit_options;
mod palettes;
use edit_options::{get_input, resize, dither1, dither2};
use image::ImageReader;
use anyhow::Context;

fn main() -> anyhow::Result<()> {
    let image_path = get_input("Enter the path to the image: ");
    let image = ImageReader::open(&image_path)
        .context(format!("Failed to open image: {}", image_path))?
        .decode()
        .context(format!("Failed to decode image: {}", image_path))?;

    let edit_options = get_input("Enter the edit options: ");
    match edit_options.as_str() {
        "resize" => { resize(&image)?; },
        "dither1" => { dither1(&image)?; },
        "dither2" => { dither2(&image)?; },
        _ => { }
    };
    println!("New image successfully created");
    Ok(())
}
