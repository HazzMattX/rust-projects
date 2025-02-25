use std::collections::HashMap;
use once_cell::sync::Lazy;
use super::palette_mapping::Color;
static PALETTE: Lazy<HashMap<&'static str, &'static [Color]>> = Lazy::new(|| {
    let mut palette_library = HashMap::new();
    palette_library.insert("p1", PALETTE1.as_slice());
    palette_library.insert("p2", PALETTE2.as_slice());
    palette_library.insert("p3", PALETTE3.as_slice());
    palette_library
});
pub fn get_palette(name: &str) -> &'static [Color] {
    PALETTE.get(name).unwrap_or(&PALETTE1.as_slice())
}
static PALETTE1: Lazy<Vec<Color>> = Lazy::new(|| vec![
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
static PALETTE2: Lazy<Vec<Color>> = Lazy::new(|| vec![
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
static PALETTE3: Lazy<Vec<Color>> = Lazy::new(|| vec![
    Color::from(0x533c43),
    Color::from(0x3b262a),
    Color::from(0x22161b),
    Color::from(0x120a0c),
    Color::from(0x3f2324),
    Color::from(0x5d3f3b),
    Color::from(0x6d5a54),
    Color::from(0x4c4740),
    Color::from(0x302e2e),
    Color::from(0x756b6b),
    Color::from(0x585c61),
    Color::from(0x453e4a),
    Color::from(0x382832),
]);
