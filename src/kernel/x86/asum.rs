#[cfg(target_arch = "x86")]
use std::arch::x86::*;

#[cfg(any(feature = "avx2", feature = "avx"))]
#[target_feature(enable = "avx")]
pub unsafe fn sasum_x86_avx(n: HanInt, sx: &[f32], incx: HanInt) -> f32 {
    assert!(
        sx.len() as i32 == 1 + (n - 1) * incx.abs(),
        "the dimension of sx is not 1+(n-1)*abs(incx)"
    );
    let mut result = 0.0e0f32;

    if n <= 0 || incx <= 0 {
        return result;
    }
    if incx == 1 {
        let m = n % 8;
        if m != 0 {
            for i in 0..m {
                result = result + sx[i as usize].abs();
            }
            if n < (8) {
                return result;
            }
        }
        let mut temp = std::arch::x86::_mm256_setzero_ps();

        for i in (m as usize..n as usize).step_by(8) {
            let sx_slice = std::arch::x86::_mm256_loadu_ps(sx.as_ptr().add(i));
            temp = std::arch::x86::_mm256_add_ps(temp, sx_slice);
        }

        // store and cum
        let mut temp_array = [0.0f32; 8];
        std::arch::x86::_mm256_storeu_ps(temp_array.as_mut_ptr(), temp);
        for i in temp_array.iter() {
            result += i;
        }
    } else {
        for sxi in sx.iter().step_by(incx as usize) {
            result = result + sxi.abs();
        }
    }
    return result;
}

#[cfg(any(
    feature = "sse",
    feature = "sse2",
    feature = "sse3",
    feature = "ssse3",
    feature = "sse4_1",
    feature = "sse4_2"
))]
#[target_feature(enable = "sse")]
pub unsafe fn sasum_x86_sse(n: HanInt, sx: &[f32], incx: HanInt) -> f32 {
    assert!(
        sx.len() as i32 == 1 + (n - 1) * incx.abs(),
        "the dimension of sx is not 1+(n-1)*abs(incx)"
    );
    let mut result = 0.0e0f32;
    let mut temp: std::arch::x86::__m128 = std::arch::x86::_mm_setzero_ps();

    if n <= 0 || incx <= 0 {
        return result;
    }
    if incx == 1 {
        let m = n % 4;
        if m != 0 {
            for i in 0..m {
                result = result + sx[i as usize].abs();
            }
            if n < 4 {
                return result;
            }
        }
        for i in (m as usize..n as usize).step_by(4) {
            let sx_slice = std::arch::x86::_mm_loadu_ps(sx.as_ptr().add(i));
            temp = std::arch::x86::_mm_add_ps(temp, sx_slice);
        }
        // store and cum
        let mut temp_array = [0.0f32; 4];
        std::arch::x86::_mm_storeu_ps(temp_array.as_mut_ptr(), temp);
        for i in temp_array.iter() {
            result += i;
        }
    } else {
        for sxi in sx.iter().step_by(incx as usize) {
            result = result + sxi.abs();
        }
    }
    return result;
}
