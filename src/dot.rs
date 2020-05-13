use self::super::{c16, c32};

pub fn sdot(n: i32, x: &[f32], incx: i32, y: &[f32], incy: i32) -> f32 {
    let mut stemp = 0.0e0;
    if n <= 0 {return stemp;}
    if incx == 1 && incy == 1 {
        let m = n % 4;
        if m != 0 {
            for i in 0..m as usize {
                stemp = stemp + x[i]*y[i];
            }
            if n < 4 {
                return stemp;
            }
        }
        for i in (m as usize..n as usize).step_by(4) {
            stemp = stemp
                + x[i]*y[i]
                + x[i+1]*y[i+1]
                + x[i+2]*y[i+2]
                + x[i+3]*y[i+3];
        }
    } else {
        let mut ix = 1;
        let mut iy = 1;
        if (incx < 0) {
            ix = (-n+1)*incx + 1;
        }
        if (incy < 0) {
            iy = (-n+1)*incy + 1;
        }
        for i in 0..n {
            stemp = stemp + x[ix as usize]*y[iy as usize];
            ix = ix + incx;
            iy = iy + incy;
        }
    }
    return stemp;
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