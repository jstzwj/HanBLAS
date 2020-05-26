use crate::{HanInt, c32, c64};

pub fn saxpy(n: HanInt, alpha: f32, x: &[f32], incx: HanInt, y: &mut [f32], incy: HanInt) {
    // check array length
    assert!(
        x.len() as HanInt >= 1 + (n - 1) * incx.abs(),
        "the dimension of x should greater than 1+(n-1)*abs(incx)"
    );
    assert!(
        y.len() as HanInt >= 1 + (n - 1) * incy.abs(),
        "the dimension of y should greater than 1+(n-1)*abs(incy)"
    );
    return saxpy_always_correct(n, alpha, x, incx, y, incy);
}


pub fn saxpy_always_correct(
    n: HanInt,
    alpha: f32,
    x: &[f32],
    incx: HanInt,
    y: &mut [f32],
    incy: HanInt,
) {
    if n <= 0 {
        return;
    }
    if alpha == 0.0f32 {
        return;
    }

    if incx <= 0 || incy <= 0 {
        return;
    }
    
    let mut ix: usize = 0;
    let mut iy: usize = 0;
    for _i in 1..n as usize {
        y[iy] = y[iy] + alpha * x[ix];
        ix = ix + incx as usize;
        iy = iy + incy as usize;
    }
}


pub fn daxpy(n: HanInt, alpha: f64, x: &[f64], incx: HanInt, y: &mut [f64], incy: HanInt) {
    // check array length
    assert!(
        x.len() as HanInt >= 1 + (n - 1) * incx.abs(),
        "the dimension of x should greater than 1+(n-1)*abs(incx)"
    );
    assert!(
        y.len() as HanInt >= 1 + (n - 1) * incy.abs(),
        "the dimension of y should greater than 1+(n-1)*abs(incy)"
    );
    return daxpy_always_correct(n, alpha, x, incx, y, incy);
}

pub fn daxpy_always_correct(
    n: HanInt,
    alpha: f64,
    x: &[f64],
    incx: HanInt,
    y: &mut [f64],
    incy: HanInt,
) {
    if n <= 0 {
        return;
    }
    if alpha == 0.0f64 {
        return;
    }
    if incx <= 0 || incy <= 0 {
        return;
    }

    if incx == 1 && incy == 1 {
        let m = n % 4;
        if m != 0 {
            for i in 0..m as usize {
                y[i] = y[i] + alpha * x[i];
            }
        }
        if n < 4 {
            return;
        }
        for i in (m as usize..n as usize).step_by(4) {
            y[i] = y[i] + alpha * x[i];
            y[i + 1] = y[i + 1] + alpha * x[i + 1];
            y[i + 2] = y[i + 2] + alpha * x[i + 2];
            y[i + 3] = y[i + 3] + alpha * x[i + 3];
        }
    } else {
        let mut ix = 0;
        let mut iy = 0;
        if incx < 0 {
            ix = (-n + 1) * incx + 1;
        }
        if incy < 0 {
            iy = (-n + 1) * incy + 1;
        }
        for _i in 1..n {
            y[iy as usize] = y[iy as usize] + alpha * x[ix as usize];
            ix = ix + incx;
            iy = iy + incy;
        }
    }
}


pub fn caxpy(n: HanInt, alpha: f32, x: &[c32], incx: HanInt, y: &mut [c32], incy: HanInt) {
    // check array length
    assert!(
        x.len() as HanInt >= 1 + (n - 1) * incx.abs(),
        "the dimension of x should greater than 1+(n-1)*abs(incx)"
    );
    assert!(
        y.len() as HanInt >= 1 + (n - 1) * incy.abs(),
        "the dimension of y should greater than 1+(n-1)*abs(incy)"
    );
}


pub fn caxpy_always_correct(n: HanInt, alpha: f32, x: &[c32], incx: HanInt, y: &mut [c32], incy: HanInt) {
    if n <= 0 {
        return;
    }
    if alpha == 0.0f32 {
        return;
    }

    if incx <= 0 || incy <= 0 {
        return;
    }
    
    let mut ix: usize = 0;
    let mut iy: usize = 0;
    for _i in 1..n as usize {
        y[iy] = y[iy] + alpha * x[ix];
        ix = ix + incx as usize;
        iy = iy + incy as usize;
    }
}

pub fn zaxpy(n: HanInt, alpha: f64, x: &[c64], incx: HanInt, y: &mut [c64], incy: HanInt) {
    // check array length
    assert!(
        x.len() as HanInt >= 1 + (n - 1) * incx.abs(),
        "the dimension of x should greater than 1+(n-1)*abs(incx)"
    );
    assert!(
        y.len() as HanInt >= 1 + (n - 1) * incy.abs(),
        "the dimension of y should greater than 1+(n-1)*abs(incy)"
    );
}

pub fn zaxpy_always_correct(n: HanInt, alpha: f64, x: &[c64], incx: HanInt, y: &mut [c64], incy: HanInt) {
    if n <= 0 {
        return;
    }
    if alpha == 0.0f64 {
        return;
    }

    if incx <= 0 || incy <= 0 {
        return;
    }
    
    let mut ix: usize = 0;
    let mut iy: usize = 0;
    for _i in 1..n as usize {
        y[iy] = y[iy] + alpha * x[ix];
        ix = ix + incx as usize;
        iy = iy + incy as usize;
    }
}