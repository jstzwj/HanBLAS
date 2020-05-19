use crate::{HanInt, c32, c64};

pub fn sdot_generic(n: HanInt, x: &[f32], incx: HanInt, y: &[f32], incy: HanInt) -> f32 {
    let mut ret = 0.0f32;
    if n <= 0 {return ret;}
    if incx == 1 && incy == 1 {
        let m = n % 4;
        if m != 0 {
            for i in 0..m as usize {
                ret = ret + x[i]*y[i];
            }
            if n < 4 {
                return ret;
            }
        }
        for i in (m as usize..n as usize).step_by(4) {
            ret = ret
                + x[i]*y[i]
                + x[i+1]*y[i+1]
                + x[i+2]*y[i+2]
                + x[i+3]*y[i+3];
        }
    } else {
        let mut ix = 0;
        let mut iy = 0;
        if incx < 0 {
            ix = (-n+1)*incx + 1;
        }
        if incy < 0 {
            iy = (-n+1)*incy + 1;
        }
        for _ in 0..n {
            ret = ret + x[ix as usize]*y[iy as usize];
            ix = ix + incx;
            iy = iy + incy;
        }
    }
    return ret;
}


pub fn ddot_generic(n: HanInt, x: &[f64], incx: HanInt, y: &[f64], incy: HanInt) -> f64 {
    let mut ret = 0.0f64;
    if n <= 0 {return ret;}
    if incx == 1 && incy == 1 {
        let m = n % 4;
        if m != 0 {
            for i in 0..m as usize {
                ret = ret + x[i]*y[i];
            }
            if n < 4 {
                return ret;
            }
        }
        for i in (m as usize..n as usize).step_by(4) {
            ret = ret
                + x[i]*y[i]
                + x[i+1]*y[i+1]
                + x[i+2]*y[i+2]
                + x[i+3]*y[i+3];
        }
    } else {
        let mut ix = 0;
        let mut iy = 0;
        if incx < 0 {
            ix = (-n+1)*incx + 1;
        }
        if incy < 0 {
            iy = (-n+1)*incy + 1;
        }
        for _ in 0..n {
            ret = ret + x[ix as usize]*y[iy as usize];
            ix = ix + incx;
            iy = iy + incy;
        }
    }
    return ret;
}
