
use crate::{HanInt, c32, c64};

pub unsafe fn sscal_generic(n: HanInt, a: f32, x: *mut f32, incx: HanInt) {
    if n <= 0 || incx <= 0 {return;}
    let mut px = x;
    let px_end = x.offset((n*incx) as isize);
    if incx == 1 {
        let m = n - n % 4;
        let px_unroll_end = px.offset(m as isize);
        while px < px_unroll_end {
            (*px) = a * (*px);
            (*px.offset(1)) = a * (*px.offset(1));
            (*px.offset(2)) = a * (*px.offset(2));
            (*px.offset(3)) = a * (*px.offset(3));
            px = px.offset(4);
        }
        while px < px_end {
            (*px) = a * (*px);
            px = px.offset(1);
        }
    } else {
        while px < px_end {
            (*px) = a * (*px);
            px = px.offset(incx as isize);
        }
    }
}

pub unsafe fn dscal_generic(n: HanInt, a: f32, x: *mut f32, incx: HanInt) {
    if n <= 0 || incx <= 0 {return;}
    let mut px = x;
    let px_end = x.offset((n*incx) as isize);
    if incx == 1 {
        let m = n - n % 4;
        let px_unroll_end = px.offset(m as isize);
        while px < px_unroll_end {
            (*px) = a * (*px);
            (*px.offset(1)) = a * (*px.offset(1));
            (*px.offset(2)) = a * (*px.offset(2));
            (*px.offset(3)) = a * (*px.offset(3));
            px = px.offset(4);
        }
        while px < px_end {
            (*px) = a * (*px);
            px = px.offset(1);
        }
    } else {
        while px < px_end {
            (*px) = a * (*px);
            px = px.offset(incx as isize);
        }
    }
}

pub unsafe fn csscal_generic(n: HanInt, a: f32, x: *mut c32, incx: HanInt) {
    if n <= 0 || incx <= 0 {return;}
    let mut px = x;
    let px_end = x.offset((n*incx) as isize);
    if incx == 1 {
        let m = n - n % 4;
        let px_unroll_end = px.offset(m as isize);
        while px < px_unroll_end {
            (*px) = a * (*px);
            (*px.offset(1)) = a * (*px.offset(1));
            (*px.offset(2)) = a * (*px.offset(2));
            (*px.offset(3)) = a * (*px.offset(3));
            px = px.offset(4);
        }
        while px < px_end {
            (*px) = a * (*px);
            px = px.offset(1);
        }
    } else {
        while px < px_end {
            (*px) = a * (*px);
            px = px.offset(incx as isize);
        }
    }
}

pub unsafe fn zdscal_generic(n: HanInt, a: f64, x: *mut c64, incx: HanInt) {
    if n <= 0 || incx <= 0 {return;}
    let mut px = x;
    let px_end = x.offset((n*incx) as isize);
    if incx == 1 {
        let m = n - n % 4;
        let px_unroll_end = px.offset(m as isize);
        while px < px_unroll_end {
            (*px) = a * (*px);
            (*px.offset(1)) = a * (*px.offset(1));
            (*px.offset(2)) = a * (*px.offset(2));
            (*px.offset(3)) = a * (*px.offset(3));
            px = px.offset(4);
        }
        while px < px_end {
            (*px) = a * (*px);
            px = px.offset(1);
        }
    } else {
        while px < px_end {
            (*px) = a * (*px);
            px = px.offset(incx as isize);
        }
    }
}