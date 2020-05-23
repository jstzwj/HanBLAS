use crate::{HanInt, c32, c64};

pub fn sswap(n: HanInt, x: &mut [f32], incx: HanInt, y: &mut [f32], incy: HanInt) {
    #[cfg(feature = "naive")]
    return sswap_always_correct(n, x, incx, y, incy);
    unsafe {
    }
}

pub fn sswap_always_correct(n: HanInt, x: &mut [f32], incx: HanInt, y: &mut [f32], incy: HanInt) {
    if n <= 0 {return;}
    if incx == 1 && incy == 1 {
        let m = n%3;
        if m != 0 {
            for i in 0..m {
                let temp = x[i];
                x[i] = y[i];
                y[i] = temp;
            }
            if n < 3 {return;}
        }
        for i in (m..n).step_by(3) {
            let mut temp:f32 = 0.0f32;
            temp = x[i];
            x[i] = y[i];
            y[i] = temp;
            temp = x[i+1];
            x[i+1] = y[i+1];
            y[i+1] = temp;
            temp = x[i+2];
            x[i+2] = y[i+2];
            y[i+2] = temp;
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
        for i in 0..n {
            let temp = x[ix];
            x[ix] = y[iy];
            y[iy] = temp;
            ix = ix + incx;
            iy = iy + incy;
        }
    }
}

pub fn dswap(n: HanInt, x: &mut [f64], incx: HanInt, y: &mut [f64], incy: HanInt) {
    #[cfg(feature = "naive")]
    return dswap_always_correct(n, x, incx, y, incy);
    unsafe {
    }
}

pub fn dswap_always_correct(n: HanInt, x: &mut [f32], incx: HanInt, y: &mut [f32], incy: HanInt) {
    if n <= 0 {return;}
    if incx == 1 && incy == 1 {
        let m = n%4;
        if m != 0 {
            for i in 0..m {
                let temp = x[i];
                x[i] = y[i];
                y[i] = temp;
            }
            if n < 4 {return;}
        }
        for i in (m..n).step_by(4) {
            let mut temp:f32 = 0.0f64;
            temp = x[i];
            x[i] = y[i];
            y[i] = temp;
            temp = x[i+1];
            x[i+1] = y[i+1];
            y[i+1] = temp;
            temp = x[i+2];
            x[i+2] = y[i+2];
            y[i+2] = temp;
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
        for i in 0..n {
            let temp = x[ix];
            x[ix] = y[iy];
            y[iy] = temp;
            ix = ix + incx;
            iy = iy + incy;
        }
    }
}

pub fn cswap(n: HanInt, x: &mut [c32], incx: HanInt, y: &mut [c32], incy: HanInt) {
    #[cfg(feature = "naive")]
    return cswap_always_correct(n, x, incx, y, incy);
    unsafe {
    }
}

pub fn cswap_always_correct(n: HanInt, x: &mut [c32], incx: HanInt, y: &mut [c32], incy: HanInt) {
    if n <= 0 {return;}
    if incx == 1 && incy == 1 {
        let m = n%3;
        if m != 0 {
            for i in 0..m {
                let temp = x[i];
                x[i] = y[i];
                y[i] = temp;
            }
            if n < 3 {return;}
        }
        for i in (m..n).step_by(3) {
            let mut temp:c32 = c32::new(0.0, 0.0);
            temp = x[i];
            x[i] = y[i];
            y[i] = temp;
            temp = x[i+1];
            x[i+1] = y[i+1];
            y[i+1] = temp;
            temp = x[i+2];
            x[i+2] = y[i+2];
            y[i+2] = temp;
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
        for i in 0..n {
            let temp = x[ix];
            x[ix] = y[iy];
            y[iy] = temp;
            ix = ix + incx;
            iy = iy + incy;
        }
    }
}

pub fn zswap(n: HanInt, x: &mut [c64], incx: HanInt, y: &mut [c64], incy: HanInt) {
    #[cfg(feature = "naive")]
    return zswap_always_correct(n, x, incx, y, incy);
    unsafe {
    }
}

pub fn zswap_always_correct(n: HanInt, x: &mut [c64], incx: HanInt, y: &mut [c64], incy: HanInt) {
    if n <= 0 {return;}
    if incx == 1 && incy == 1 {
        let m = n%3;
        if m != 0 {
            for i in 0..m {
                let temp = x[i];
                x[i] = y[i];
                y[i] = temp;
            }
            if n < 3 {return;}
        }
        for i in (m..n).step_by(3) {
            let mut temp:c64 = c64::new(0.0, 0.0);
            temp = x[i];
            x[i] = y[i];
            y[i] = temp;
            temp = x[i+1];
            x[i+1] = y[i+1];
            y[i+1] = temp;
            temp = x[i+2];
            x[i+2] = y[i+2];
            y[i+2] = temp;
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
        for i in 0..n {
            let temp = x[ix];
            x[ix] = y[iy];
            y[iy] = temp;
            ix = ix + incx;
            iy = iy + incy;
        }
    }
}