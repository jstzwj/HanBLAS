use num_traits::{Zero, Float};
use crate::{c32, c64};

pub fn sign<T:Float>(a:&T, b:&T) -> T {
    if *b >= Zero::zero() {
        return a.abs();
    } else {
        return -a.abs();
    }
}

#[inline]
pub fn lsame(i: u8, c:char) -> bool{
    (i as char).to_lowercase().to_string() == c.to_lowercase().to_string()
}

#[inline]
pub fn scabs1(z:c32) -> f32 {
    z.re.abs() + z.im.abs()
}

#[inline]
pub fn dcabs1(z:c64) -> f64 {
    z.re.abs() + z.im.abs()
}

#[inline]
pub fn cconjg(c:c32) -> c32 {
    c32::new(c.re, -c.im)
}

#[inline]
pub fn zconjg(z:c64) -> c64 {
    c64::new(z.re, -z.im)
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