use crate::{c32, c64, HanInt};

pub fn scopy(n: HanInt, x: &[f32], incx: HanInt, y: &mut [f32], incy: HanInt) {
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
    return scopy_always_correct(n, x, incx, y, incy);
    unsafe {
        return crate::kernel::generic::copy::scopy_generic(
            n,
            x.as_ptr(),
            incx,
            y.as_mut_ptr(),
            incy,
        );
    }
}

#[allow(dead_code)]
fn scopy_always_correct(n: HanInt, x: &[f32], incx: HanInt, y: &mut [f32], incy: HanInt) {
    if n <= 0 {
        return;
    }
    if incx == 1 && incy == 1 {
        let m = n % 8;
        if m != 0 {
            for i in 0..m as usize {
                y[i] = x[i];
            }
            if n < 8 {
                return;
            }
        }
        for i in (m as usize..n as usize).step_by(8) {
            y[i] = x[i];
            y[i + 1] = x[i + 1];
            y[i + 2] = x[i + 2];
            y[i + 3] = x[i + 3];
            y[i + 4] = x[i + 4];
            y[i + 5] = x[i + 5];
            y[i + 6] = x[i + 6];
            y[i + 7] = x[i + 7];
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
        for _i in 0..n as usize {
            y[iy as usize] = x[ix as usize];
            ix = ix + incx;
            iy = iy + incy;
        }
    }
}

pub fn dcopy(n: HanInt, x: &[f64], incx: HanInt, y: &mut [f64], incy: HanInt) {
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
    return dcopy_always_correct(n, x, incx, y, incy);
    unsafe {
        return crate::kernel::generic::copy::dcopy_generic(
            n,
            x.as_ptr(),
            incx,
            y.as_mut_ptr(),
            incy,
        );
    }
}

#[allow(dead_code)]
fn dcopy_always_correct(n: HanInt, x: &[f64], incx: HanInt, y: &mut [f64], incy: HanInt) {
    if n <= 0 {
        return;
    }
    if incx == 1 && incy == 1 {
        let m = n % 8;
        if m != 0 {
            for i in 0..m as usize {
                y[i] = x[i];
            }
            if n < 8 {
                return;
            }
        }
        for i in (m as usize..n as usize).step_by(8) {
            y[i] = x[i];
            y[i + 1] = x[i + 1];
            y[i + 2] = x[i + 2];
            y[i + 3] = x[i + 3];
            y[i + 4] = x[i + 4];
            y[i + 5] = x[i + 5];
            y[i + 6] = x[i + 6];
            y[i + 7] = x[i + 7];
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
        for _i in 0..n as usize {
            y[iy as usize] = x[ix as usize];
            ix = ix + incx;
            iy = iy + incy;
        }
    }
}

pub fn ccopy(n: HanInt, x: &[c32], incx: HanInt, y: &mut [c32], incy: HanInt) {}

pub fn ccopy_always_correct(n: HanInt, x: &[c32], incx: HanInt, y: &mut [c32], incy: HanInt) {
    if n <= 0 {
        return;
    }
    if incx == 1 && incy == 1 {
        for i in 0..n as usize {
            y[i] = x[i];
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
        for i in 0..n as usize {
            y[iy as usize] = x[ix as usize];
            ix = ix + incx;
            iy = iy + incy;
        }
    }
}

pub fn zcopy(n: HanInt, x: &[c64], incx: HanInt, y: &mut [c64], incy: HanInt) {}

pub fn zcopy_always_correct(n: HanInt, x: &[c64], incx: HanInt, y: &mut [c64], incy: HanInt) {
    if n <= 0 {
        return;
    }
    if incx == 1 && incy == 1 {
        for i in 0..n as usize {
            y[i] = x[i];
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
        for i in 0..n {
            y[iy as usize] = x[ix as usize];
            ix = ix + incx;
            iy = iy + incy;
        }
    }
}
