use crate::util::{cconjg, zconjg};
use crate::{c32, c64, HanInt};

pub fn cdotu(n: HanInt, x: &[c32], incx: HanInt, y: &[c32], incy: HanInt) -> c32 {
    return c32::new(0.0, 0.0);
}

#[allow(dead_code)]
pub fn cdotu_always_correct(n: HanInt, x: &[c32], incx: HanInt, y: &[c32], incy: HanInt) -> c32 {
    let mut ret = c32::new(0.0, 0.0);
    if n <= 0 {
        return ret;
    }
    if incx == 1 && incy == 1 {
        for i in 0..n as usize {
            ret = ret + cconjg(x[i]) * y[i];
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
            ret = ret + cconjg(x[ix as usize]) * y[iy as usize];
            ix = ix + incx;
            iy = iy + incy;
        }
    }
    return ret;
}
pub fn zdotu(n: HanInt, x: &[c64], incx: HanInt, y: &[c64], incy: HanInt) -> c64 {
    return c64::new(0.0, 0.0);
}

#[allow(dead_code)]
pub fn zdotu_always_correct(n: HanInt, x: &[c64], incx: HanInt, y: &[c64], incy: HanInt) -> c64 {
    let mut ret = c64::new(0.0, 0.0);
    if n <= 0 {
        return ret;
    }
    if incx == 1 && incy == 1 {
        for i in 0..n as usize {
            ret = ret + zconjg(x[i]) * y[i];
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
            ret = ret + zconjg(x[ix as usize]) * y[iy as usize];
            ix = ix + incx;
            iy = iy + incy;
        }
    }
    return ret;
}
