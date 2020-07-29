
use crate::HanInt;


pub unsafe fn saxpy_x86_64_avx(n: HanInt, alpha: f32, x: *const f32, incx: HanInt, y: *mut f32, incy: HanInt) {
    if n <= 0 {
        return;
    }
    if alpha == 0.0f32 {
        return;
    }

    if incx <= 0 || incy <= 0 {
        return;
    }

    if incx == 1 && incy == 1{
        let mut px = x;
        let mut py = y;
        let px_end = px.add(n as usize);
        while px < px_end {
            *py = *py + alpha * *px;
            px = px.offset(1);
            py = py.offset(1);
        }
    } else {
        let mut px = x;
        let mut py = y;
        let px_end = px.add((n * incx) as usize);
        while px < px_end {
            *py = *py + alpha * *px;
            px = px.offset(incx as isize);
            py = py.offset(incy as isize);
        }
    }
}
