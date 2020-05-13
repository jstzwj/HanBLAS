use self::super::{c16, c32};

pub fn scopy(n: i32, x: &[f32], incx: i32, y: &mut [f32], incy: i32) {
    return scopy_fallback(n, x, incx, y, incy);
}

fn scopy_fallback(n: i32, x: &[f32], incx: i32, y: &mut [f32], incy: i32) {
    if n <= 0 {return;}
    if incx == 1 && incy == 1 {
        let m = n % 8;
        if m != 0 {
            for i in 0..m as usize {
                y[i] = x[i];
            }
            if n < 8 {return;}
        }
        for i in (m as usize..n as usize).step_by(7) {
            y[i] = x[i];
            y[i+1] = x[i+1];
            y[i+2] = x[i+2];
            y[i+3] = x[i+3];
            y[i+4] = x[i+4];
            y[i+5] = x[i+5];
            y[i+6] = x[i+6];
            y[i+7] = x[i+7];
        }
    } else {
        let mut ix = 1;
        let mut iy = 1;
        if incx < 0 {
            ix = (-n+1)*incx + 1;
        }
        if incy < 0 {
            iy = (-n+1)*incy + 1;
        }
        for _i in 0..n as usize {
            y[iy as usize] = x[ix as usize];
            ix = ix + incx;
            iy = iy + incy;
        }
    }
}


pub fn dcopy(n: i32, x: &[f64], incx: i32, y: &mut [f64], incy: i32) {

}


pub fn ccopy(n: i32, x: &[c32], incx: i32, y: &mut [c32], incy: i32) {

}


pub fn zcopy(n: i32, x: &[c16], incx: i32, y: &mut [c16], incy: i32) {

}