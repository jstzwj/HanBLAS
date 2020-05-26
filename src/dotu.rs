use crate::{c32, c64, HanInt};

pub fn cdotc(n: HanInt, x: &[c32], incx: HanInt, y: &[c32], incy: HanInt) -> c32 {
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
    return cdotc_always_correct(n, x, incx, y, incy);

    return c32::new(0.0, 0.0);
}

#[allow(dead_code)]
pub fn cdotc_always_correct(n: HanInt, x: &[c32], incx: HanInt, y: &[c32], incy: HanInt) -> c32 {
    let mut ret = c32::new(0.0, 0.0);
    if n <= 0 {
        return ret;
    }
    if incx == 1 && incy == 1 {
        for i in 0..n as usize {
            ret = ret + x[i] * y[i];
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
        for _ in 0..n {
            ret = ret + x[ix as usize] * y[iy as usize];
            ix = ix + incx;
            iy = iy + incy;
        }
    }
    return ret;
}
pub fn zdotc(n: HanInt, x: &[c64], incx: HanInt, y: &[c64], incy: HanInt) -> c64 {
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
    return zdotc_always_correct(n, x, incx, y, incy);

    return c64::new(0.0, 0.0);
}

#[allow(dead_code)]
pub fn zdotc_always_correct(n: HanInt, x: &[c64], incx: HanInt, y: &[c64], incy: HanInt) -> c64 {
    let mut ret = c64::new(0.0, 0.0);
    if n <= 0 {
        return ret;
    }
    if incx == 1 && incy == 1 {
        for i in 0..n as usize {
            ret = ret + x[i] * y[i];
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
        for _ in 0..n {
            ret = ret + x[ix as usize] * y[iy as usize];
            ix = ix + incx;
            iy = iy + incy;
        }
    }
    return ret;
}
