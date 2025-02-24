use std::f32::INFINITY;
pub struct Color {
    pub(crate) r: u8,
    pub(crate) g: u8,
    pub(crate) b: u8,
}
pub struct QuantizationError {
    pub(crate) r: f32,
    pub(crate) g: f32,
    pub(crate) b: f32,
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
pub fn map_to_palette2(orig: Color, palette: &Vec<Color>) -> (Color, QuantizationError) {
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
pub fn map_to_palette1(orig: Color, palette: &Vec<Color>) -> Color {
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
    Color { r: closest.r, g: closest.g, b: closest.b }
}
