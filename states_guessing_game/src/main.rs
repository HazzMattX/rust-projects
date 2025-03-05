use image::ImageReader;
use csv::Reader;
use piston_window::{PistonWindow, WindowSettings, Texture, TextureSettings, clear, image as draw_image, Button, MouseButton, MouseCursorEvent};

fn main() {
    let image = ImageReader::open("blank_states_image.gif").unwrap().decode().unwrap();
    let rdr = Reader::from_path("50_states.csv")
        .expect("Could not read CSV file");

    let mut window: PistonWindow = WindowSettings::new("States Guessing Game", [800, 600])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let texture = Texture::from_image(
        &mut window.create_texture_context(),
        &image,
        &TextureSettings::new()
    ).unwrap();

    let mut cursor_position = [0.0, 0.0];

    while let Some(event) = window.next() {
        if let Some(pos) = event.mouse_cursor_args() {
            cursor_position = pos;
        }

        if let Some(Button::Mouse(MouseButton::Left)) = event.press_args() {
            println!("Mouse clicked at: {:?}", cursor_position);
            // Handle interaction logic here
        }

        window.draw_2d(&event, |c, g, _| {
            clear([1.0; 4], g);
            draw_image(&texture, c.transform, g);
        });
    }
}
