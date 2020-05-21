use crate::{HanInt, c32, c64};

pub unsafe fn srot_generic(
    n: HanInt,
    x: *mut f32,
    incx: HanInt,
    y: *mut f32,
    incy: HanInt,
    c: f32,
    s: f32,
) {
    if n <= 0 {
        return;
    }
    // TODO: if we need negative inc?
    if incx <= 0 || incy <= 0 {
        return;
    }
    let mut px = x;
    let mut py = y;
    let px_end = x.offset((n * incx) as isize);
    if incx == 1 && incy == 1 {
        let unroll_n = n % 4;
        let px_unroll = px.offset(unroll_n as isize);
        while px < px_unroll {
            let mut temp;
            temp = c * (*px) + s * (*py);
            (*py) = c * (*py) - s * (*px);
            (*px) = temp;

            temp = c * (*px.offset(1)) + s * (*py.offset(1));
            (*py.offset(1)) = c * (*py.offset(1)) - s * (*px.offset(1));
            (*px.offset(1)) = temp;

            temp = c * (*px.offset(2)) + s * (*py.offset(2));
            (*py.offset(2)) = c * (*py.offset(2)) - s * (*px.offset(2));
            (*px.offset(2)) = temp;

            temp = c * (*px.offset(3)) + s * (*py.offset(3));
            (*py.offset(3)) = c * (*py.offset(3)) - s * (*px.offset(3));
            (*px.offset(3)) = temp;

            px = px.offset(4);
            py = py.offset(4);
        }

        while px < px_end {
            let temp = c * (*px) + s * (*py);
            (*py) = c * (*py) - s * (*px);
            (*px) = temp;
            px = px.offset(1);
            py = py.offset(1);
        }
    } else {
        while px < px_end {
            let temp = c * (*px) + s * (*py);
            (*py) = c * (*py) - s * (*px);
            (*px) = temp;
            px = px.offset(incx as isize);
            py = py.offset(incy as isize);
        }
    }
}



pub unsafe fn drot_generic(
    n: HanInt,
    x: *mut f64,
    incx: HanInt,
    y: *mut f64,
    incy: HanInt,
    c: f64,
    s: f64,
) {
    if n <= 0 {
        return;
    }
    // TODO: if we need negative inc?
    if incx <= 0 || incy <= 0 {
        return;
    }
    let mut px = x;
    let mut py = y;
    let px_end = x.offset((n * incx) as isize);
    if incx == 1 && incy == 1 {
        let unroll_n = n % 4;
        let px_unroll = px.offset(unroll_n as isize);
        while px < px_unroll {
            let mut temp;
            temp = c * (*px) + s * (*py);
            (*py) = c * (*py) - s * (*px);
            (*px) = temp;

            temp = c * (*px.offset(1)) + s * (*py.offset(1));
            (*py.offset(1)) = c * (*py.offset(1)) - s * (*px.offset(1));
            (*px.offset(1)) = temp;

            temp = c * (*px.offset(2)) + s * (*py.offset(2));
            (*py.offset(2)) = c * (*py.offset(2)) - s * (*px.offset(2));
            (*px.offset(2)) = temp;

            temp = c * (*px.offset(3)) + s * (*py.offset(3));
            (*py.offset(3)) = c * (*py.offset(3)) - s * (*px.offset(3));
            (*px.offset(3)) = temp;

            px = px.offset(4);
            py = py.offset(4);
        }

        while px < px_end {
            let temp = c * (*px) + s * (*py);
            (*py) = c * (*py) - s * (*px);
            (*px) = temp;
            px = px.offset(1);
            py = py.offset(1);
        }
    } else {
        while px < px_end {
            let temp = c * (*px) + s * (*py);
            (*py) = c * (*py) - s * (*px);
            (*px) = temp;
            px = px.offset(incx as isize);
            py = py.offset(incy as isize);
        }
    }
}