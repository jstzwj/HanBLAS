
use crate::HanInt;

pub fn daxpy(
    n: HanInt, 
    alpha: f64, 
    x: &[f64], 
    incx: HanInt, 
    y: &mut [f64], 
    incy: HanInt
) {
    return daxpy_always_correct(n, alpha, x, incx, y, incy);
}

pub fn daxpy_always_correct(
    n: HanInt, 
    alpha: f64, 
    x: &[f64], 
    incx: HanInt, 
    y: &mut [f64], 
    incy: HanInt
) {
    if n <= 0 {return;}
    if alpha == 0.0f64 {return;}
    if incx == 1 && incy == 1 {
        let m = n % 4;
        if m != 0 {
            for i in 0..m as usize {
                y[i] = y[i] + alpha*x[i];
            }
        }
        if n < 4 {return;}
        for i in (m as usize..n as usize).step_by(4) {
            y[i] = y[i] + alpha*x[i];
            y[i+1] = y[i+1] + alpha*x[i+1];
            y[i+2] = y[i+2] + alpha*x[i+2];
            y[i+3] = y[i+3] + alpha*x[i+3];
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
        for _i in 1..n {
            y[iy as usize] = y[iy as usize] + alpha*x[ix as usize];
            ix = ix + incx;
            iy = iy + incy;
        }
    }
}