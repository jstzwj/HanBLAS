use crate::{HanInt, c32, c64};


pub unsafe fn scopy_generic(n: HanInt, x: *const f32, incx: HanInt, y: *mut f32, incy: HanInt) {
    if n <= 0 {return;}
    // TODO: if we need negative inc?
    if incx <= 0 || incy <= 0 {return;}
    let mut px = x;
    let mut py = y;
    let px_end = x.offset((n * incx) as isize);
    if incx == 1 && incy == 1 {
        let m = n - n % 4;
        let px_unroll = px.offset(m as isize);
        while px < px_unroll {
            *py = *px;
            *py.offset(1) = *px.offset(1);
            *py.offset(2) = *px.offset(2);
            *py.offset(3) = *px.offset(3);
            px = px.offset(4);
            py = py.offset(4);
        }

        while px < px_end {
            *py = *px;
            px = px.offset(1);
            py = py.offset(1);
        }
    } else {
        while px < px_end {
            *py = *px;
            px = px.offset(incx as isize);
            py = py.offset(incy as isize);
        }
    }
}


pub unsafe fn dcopy_generic(n: HanInt, x: *const f64, incx: HanInt, y: *mut f64, incy: HanInt) {
    if n <= 0 {return;}
    // TODO: if we need negative inc?
    if incx <= 0 || incy <= 0 {return;}
    let mut px = x;
    let mut py = y;
    let px_end = x.offset((n * incx) as isize);
    if incx == 1 && incy == 1 {
        while px < px_end {
            *py = *px;
            px = px.offset(1);
            py = py.offset(1);
        }
    } else {
        while px < px_end {
            *py = *px;
            px = px.offset(incx as isize);
            py = py.offset(incy as isize);
        }
    }
}