
pub fn sign(a:&f32, b:&f32) -> f32 {
    if *b >= 0.0f32 {
        return a.abs();
    } else {
        return -a.abs();
    }
}