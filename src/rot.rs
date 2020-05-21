use crate::{c32, c64, HanInt};

pub fn srot(n: HanInt, x: &mut [f32], incx: HanInt, y: &mut [f32], incy: HanInt, c: f32, s: f32) {
    // check array length first
    assert!(
        x.len() as HanInt >= 1 + (n - 1) * incx.abs(),
        "the dimension of x should greater than 1+(n-1)*abs(incx)"
    );
    assert!(
        y.len() as HanInt >= 1 + (n - 1) * incy.abs(),
        "the dimension of y should greater than 1+(n-1)*abs(incy)"
    );
    #[cfg(feature = "naive")]
    return srot_always_correct(n, x, incx, y, incy);
    unsafe {
        return crate::kernel::generic::rot::srot_generic(
            n,
            x.as_mut_ptr(),
            incx,
            y.as_mut_ptr(),
            incy,
            c,
            s
        );
    }
}

#[allow(dead_code)]
pub fn srot_always_correct(
    n: HanInt,
    x: &mut [f32],
    incx: HanInt,
    y: &mut [f32],
    incy: HanInt,
    c: f32,
    s: f32,
) {
    if n <= 0 {
        return;
    }
    if incx == 1 && incy == 1 {
        for i in 0..n as usize {
            let stemp = c * x[i] + s * y[i];
            y[i] = c * y[i] - s * x[i];
            x[i] = stemp;
        }
    } else {
        let mut ix: usize = 1;
        let mut iy: usize = 1;
        if incx < 0 {
            ix = ((-n + 1) * incx + 1) as usize;
        }
        if incy < 0 {
            iy = ((-n + 1) * incy + 1) as usize;
        }
        for _ in 0..n {
            let stemp = c * x[ix] + s * y[iy];
            y[iy] = c * y[iy] - s * x[ix];
            x[ix] = stemp;
            ix = ix + incx as usize;
            iy = iy + incy as usize;
        }
    }
}

pub fn drot(n: HanInt, x: &mut [f64], incx: HanInt, y: &mut [f64], incy: HanInt, c: f64, s: f64) {
    // check array length first
    assert!(
        x.len() as HanInt >= 1 + (n - 1) * incx.abs(),
        "the dimension of x should greater than 1+(n-1)*abs(incx)"
    );
    assert!(
        y.len() as HanInt >= 1 + (n - 1) * incy.abs(),
        "the dimension of y should greater than 1+(n-1)*abs(incy)"
    );
    #[cfg(feature = "naive")]
    return drot_always_correct(n, x, incx, y, incy);
    unsafe {
        return crate::kernel::generic::rot::drot_generic(
            n,
            x.as_mut_ptr(),
            incx,
            y.as_mut_ptr(),
            incy,
            c,
            s
        );
    }
}

pub fn csrot(n: HanInt, x: &[c32], incx: HanInt, y: &[c32], incy: HanInt, c: f32, s: f32) {}

pub fn dzrot(n: HanInt, x: &[c64], incx: HanInt, y: &[c64], incy: HanInt, c: f32, s: f32) {}
