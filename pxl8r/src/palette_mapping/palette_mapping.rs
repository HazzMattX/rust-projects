use std::f32::INFINITY;
use once_cell::sync::Lazy;
pub static PALETTE1: Lazy<Vec<Color>> = Lazy::new(|| vec![
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
]);
pub static PALETTE2: Lazy<Vec<Color>> = Lazy::new(|| vec![
    Color::from(0xf2c0a2),
    Color::from(0xe98472),
    Color::from(0xd82323),
    Color::from(0x98183c),
    Color::from(0x1fcb23),
    Color::from(0x126d30),
    Color::from(0x26dddd),
    Color::from(0x1867a0),
    Color::from(0x934226),
    Color::from(0x6c251e),
    Color::from(0xf7e26c),
    Color::from(0xedb329),
    Color::from(0xe76d14),
    Color::from(0xf2f2f9),
    Color::from(0x6a5fa0),
    Color::from(0x161423),
]);

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
impl From<&[u8; 3]> for Color {
    fn from(value: &[u8; 3]) -> Self {
        Color {
            b: value[0],
            g: value[1],
            r: value[2],
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
