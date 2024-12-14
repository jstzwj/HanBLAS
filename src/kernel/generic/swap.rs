use crate::HanInt;


pub unsafe fn sswap_generic(n: HanInt, x: *mut f32, incx: HanInt, y: *mut f32, incy: HanInt) {
    if n <= 0 {return;}
    let mut px = x;
    let mut py = y;
    let px_end = px.offset((n*incx) as isize);
    if incx == 1 && incy == 1 {
        let m = n - n % 4;
        let px_unroll_end = px.offset(m as isize);
        while px < px_unroll_end {
            std::ptr::swap(px, py);
            std::ptr::swap(px.offset(1), py.offset(1));
            std::ptr::swap(px.offset(2), py.offset(2));
            std::ptr::swap(px.offset(3), py.offset(3));
            px = px.offset(4);
            py = py.offset(4);
        }
        while px < px_end {
            std::ptr::swap(px, py);
            px = px.offset(1);
            py = py.offset(1);
        }
    } else {
        while px < px_end {
            std::ptr::swap(px, py);
            px = px.offset(incx as isize);
            py = py.offset(incy as isize);
        }
    }
}