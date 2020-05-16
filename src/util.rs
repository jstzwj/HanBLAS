use num_traits::{Zero, Float};

pub fn sign<T:Float>(a:&T, b:&T) -> T {
    if *b >= Zero::zero() {
        return a.abs();
    } else {
        return -a.abs();
    }
}


pub fn lsame(i: u8, c:char) -> bool{
    return (i as char).to_lowercase().to_string() == c.to_lowercase().to_string();
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