mod edit_options;
mod palette_mapping;
use edit_options::*;
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
                    dither(&image, dither_mode)?;
                },
        _ => { }
    };
    println!("New image successfully created");
    Ok(())
}
