use self::super::{c32, c64, HanInt};

pub fn sdot(n: HanInt, x: &[f32], incx: HanInt, y: &[f32], incy: HanInt) -> f32 {
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
    return sdot_always_correct(n, x, incx, y, incy);
    unsafe {
        return crate::kernel::generic::dot::sdot_generic(n, x.as_ptr(), incx, y.as_ptr(), incy);
    }
}

pub fn sdot_always_correct(n: HanInt, x: &[f32], incx: HanInt, y: &[f32], incy: HanInt) -> f32 {
    let mut stemp = 0.0f32;
    if n <= 0 {
        return stemp;
    }
    if incx == 1 && incy == 1 {
        let m = n % 4;
        if m != 0 {
            for i in 0..m as usize {
                stemp = stemp + x[i] * y[i];
            }
            if n < 4 {
                return stemp;
            }
        }
        for i in (m as usize..n as usize).step_by(4) {
            stemp = stemp
                + x[i] * y[i]
                + x[i + 1] * y[i + 1]
                + x[i + 2] * y[i + 2]
                + x[i + 3] * y[i + 3];
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
            stemp = stemp + x[ix as usize] * y[iy as usize];
            ix = ix + incx;
            iy = iy + incy;
        }
    }
    return stemp;
}

pub fn ddot(n: HanInt, x: &[f64], incx: HanInt, y: &[f64], incy: HanInt) -> f64 {
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
    return ddot_always_correct(n, x, incx, y, incy);
    unsafe {
        return crate::kernel::generic::dot::ddot_generic(n, x.as_ptr(), incx, y.as_ptr(), incy);
    }
}

pub fn ddot_always_correct(n: HanInt, x: &[f64], incx: HanInt, y: &[f64], incy: HanInt) -> f64 {
    let mut ret = 0.0f64;
    if n <= 0 {
        return ret;
    }
    if incx == 1 && incy == 1 {
        let m = n % 4;
        if m != 0 {
            for i in 0..m as usize {
                ret = ret + x[i] * y[i];
            }
            if n < 4 {
                return ret;
            }
        }
        for i in (m as usize..n as usize).step_by(4) {
            ret =
                ret + x[i] * y[i] + x[i + 1] * y[i + 1] + x[i + 2] * y[i + 2] + x[i + 3] * y[i + 3];
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

pub fn dsdot(n: HanInt, x: &[f32], incx: HanInt, y: &[f32], incy: HanInt) -> f64 {
    return 0.0;
}

pub fn dsdot_always_correct(n: HanInt, x: &[f32], incx: HanInt, y: &[f32], incy: HanInt) -> f64 {
    let mut ret = 0.0f64;
    if n <= 0 {
        return ret;
    }
    if incx == incy && incx > 0 {
        let ns = n * incx;
        for i in (0..ns as usize).step_by(incx as usize) {
            ret = ret + (x[i] as f64) * (y[i] as f64);
        }
    } else {
        let mut kx = 1;
        let mut ky = 1;
        if incx < 0 {
            kx = 1 + (1 - n) * incx;
        }
        if incy < 0 {
            ky = 1 + (1 - n) * incy;
        }
        for i in 0..n as usize {
            ret = ret + (x[kx as usize] as f64) * (y[ky as usize] as f64);
            kx = kx + incx;
            ky = ky + incy;
        }
    }
    return ret;
}
