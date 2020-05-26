use crate::{HanInt, c32, c64};


pub unsafe fn sswap_generic(n: HanInt, x: *mut f32, incx: HanInt, y: *mut f32, incy: HanInt) {
    if n <= 0 {return;}
    let px = x;
    let py = y;
    let px_end = px.offset((n*incx) as isize);
    if incx == 1 && incy == 1 {
        let m = n - n % 4;
        let px_unroll_end = px.offset(m as isize);
        while px < px_unroll_end {
            std::ptr::swap(px, py);
            std::ptr::swap(px.offset(1), py.offset(1));
            std::ptr::swap(px.offset(2), py.offset(2));
            std::ptr::swap(px.offset(3), py.offset(3));
            px.offset(4);
            py.offset(4);
        }
        while px < px_end {
            std::ptr::swap(px, py);
            px.offset(1);
            py.offset(1);
        }
    } else {
        while px < px_end {
            std::ptr::swap(px, py);
            px.offset(incx as isize);
            py.offset(incy as isize);
        }
    }
}