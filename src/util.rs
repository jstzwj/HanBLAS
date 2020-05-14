
pub fn sign(a:&f32, b:&f32) -> f32 {
    if *b >= 0.0f32 {
        return a.abs();
    } else {
        return -a.abs();
    }
}


pub fn scomparea(a:&[f32], b:&[f32]) -> f32 {
    let mut max_diff: f32 = 0.0;
    for i in 0..a.len() {
        let diff = (a[i] - b[i]).abs();
        if diff > max_diff {
            max_diff = diff;
        }
    }
    return max_diff;
}