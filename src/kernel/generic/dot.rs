use crate::{HanInt, c32, c64};

pub unsafe fn sdot_generic(n: HanInt, x: *const f32, incx: HanInt, y: *const f32, incy: HanInt) -> f32 {
    let mut ret = 0.0f32;
    if n <= 0 {return ret;}
    // TODO: if we need negative inc?
    if incx <= 0 || incy <= 0 {return ret;}

    let mut px = x;
    let mut py = y;
    let px_end = x.offset((n * incx) as isize);

    if incx == 1 && incy == 1 {
        let m = n - n % 4;
        let px_unroll = px.offset(n as isize);
        while px < px_unroll {
            ret = ret
                + (*px) * (*py)
                + (*px.offset(1)) * (*py.offset(1))
                + (*px.offset(2)) * (*py.offset(2))
                + (*px.offset(3)) * (*py.offset(3));
            px = px.offset(1);
            py = py.offset(1);
        }

        while px < px_end {
            ret = ret + (*px) * (*py);
            px = px.offset(1);
            py = py.offset(1);
        }
    } else {
        while px < px_end {
            ret = ret + (*px) * (*py);
            px = px.offset(incx as isize);
            py = py.offset(incy as isize);
        }
    }
    return ret;
}


pub unsafe fn ddot_generic(n: HanInt, x: *const f64, incx: HanInt, y: *const f64, incy: HanInt) -> f64 {
    let mut ret = 0.0f64;
    if n <= 0 {return ret;}
    // TODO: if we need negative inc?
    if incx <= 0 || incy <= 0 {return ret;}

    let mut px = x;
    let mut py = y;
    let px_end = x.offset((n * incx) as isize);

    if incx == 1 && incy == 1 {
        let m = n - n % 4;
        let px_unroll = px.offset(n as isize);
        while px < px_unroll {
            ret = ret
                + (*px) * (*py)
                + (*px.offset(1)) * (*py.offset(1))
                + (*px.offset(2)) * (*py.offset(2))
                + (*px.offset(3)) * (*py.offset(3));
            px = px.offset(1);
            py = py.offset(1);
        }

        while px < px_end {
            ret = ret + (*px) * (*py);
            px = px.offset(1);
            py = py.offset(1);
        }
    } else {
        while px < px_end {
            ret = ret + (*px) * (*py);
            px = px.offset(incx as isize);
            py = py.offset(incy as isize);
        }
    }
    return ret;
}
