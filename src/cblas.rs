

use super::asum;
#[no_mangle]
pub extern "C" fn sasum(n: i32, sx: *const f32, incx: i32) -> f32 {
    unsafe {
        return asum::sasum(
            n,
            std::slice::from_raw_parts(sx, (1 + (n-1)*incx.abs())as usize),
            incx
        );
    }
}