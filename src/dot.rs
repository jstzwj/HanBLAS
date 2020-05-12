use self::super::{c16, c32};

pub fn sdot(n: i32, x: &[f32], incx: i32, y: &[f32], incy: i32) -> f32 {
    return 0.0;
}


pub fn ddot(n: i32, x: &[f64], incx: i32, y: &[f64], incy: i32) -> f64 {
    return 0.0;
}

pub fn cdot(n: i32, x: &[c32], incx: i32, y: &[c32], incy: i32) -> c32 {
    return c32::new(0.0, 0.0);
}

pub fn zdot(n: i32, x: &[c16], incx: i32, y: &[c16], incy: i32) -> c16 {
    return c16::new(half::f16::from_f32(0.0), half::f16::from_f32(0.0));
}