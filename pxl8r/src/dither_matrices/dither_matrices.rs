use std::collections::HashMap;
pub fn get_dithering_matrices() -> HashMap<&'static str, [[f32; 3]; 2]> {
    let mut matrices = HashMap::new();
    matrices.insert("floyd", [
        [0.0, 7.0 / 16.0, 0.0],
        [3.0 / 16.0, 5.0 / 16.0, 1.0 / 16.0]
    ]);
    matrices.insert("jarvis", [
        [0.0, 7.0 / 48.0, 5.0 / 48.0],
        [3.0 / 48.0, 5.0 / 48.0, 7.0 / 48.0]
    ]);
    matrices.insert("stucki", [
        [0.0, 8.0 / 42.0, 4.0 / 42.0],
        [2.0 / 42.0, 4.0 / 42.0, 8.0 / 42.0]
    ]);
    matrices.insert("burkes", [
        [0.0, 8.0 / 32.0, 4.0 / 32.0],
        [2.0 / 32.0, 4.0 / 32.0, 8.0 / 32.0]
    ]);
    matrices.insert("sierra", [
        [0.0, 5.0 / 32.0, 3.0 / 32.0],
        [2.0 / 32.0, 4.0 / 32.0, 5.0 / 32.0]
    ]);
    matrices
}
