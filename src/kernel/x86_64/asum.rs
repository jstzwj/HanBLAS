use rayon::prelude::*;

#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

fn sasum_check_arguments(n: isize, sx: &[f32], incx: isize) {
    assert!(n >= 0, "n must be positive or zero");
    assert!(sx.len() as isize == 1 + (n-1)*incx.abs(), "the dimension of sx is not 1+(n-1)*abs(incx)");
}

#[cfg(any(feature = "avx2", feature = "avx"))]
#[target_feature(enable = "avx")]
pub unsafe fn sasum_x86_64_avx(n: isize, sx: &[f32], incx: isize) -> f32 {
    sasum_check_arguments(n, sx, incx);
    let mut result = 0.0e0f32;

    if n <= 0 || incx <= 0 {
        return result;
    }
    if incx == 1 {
        let m = n%8;
        if m != 0 {
            for i in 0..m {
                result = result + sx[i as usize].abs();
            }
            if n < 8 {
                return result;
            }
        }
        let mut temp = std::arch::x86_64::_mm256_setzero_ps();

        for i in (m as usize..n as usize).step_by(8) {
            let sx_slice = std::arch::x86_64::_mm256_loadu_ps(sx.as_ptr().add(i));
            temp = std::arch::x86_64::_mm256_add_ps(temp, sx_slice);
        }
        
        // store and cum
        let mut temp_array = [0.0f32;8];
        std::arch::x86_64::_mm256_storeu_ps(temp_array.as_mut_ptr(), temp);
        for i in temp_array.iter() {
            result += i;
        }
    } else {
        // fixme: incx是负的
        for sxi in sx.iter().step_by(incx as usize) {
            result = result + sxi.abs();
        }
    }
    return result;
}


#[cfg(all(any(feature = "avx2", feature = "avx"), feature = "multi_thread"))]
#[target_feature(enable = "avx")]
pub unsafe fn sasum_x86_64_mt_avx(n: isize, sx: &[f32], incx: isize) -> f32 {
    // small size disable mt
    if 1 + (n-1)*incx.abs() < 500000 {
        return sasum_x86_64_avx(n, sx, incx);
    }

    sasum_check_arguments(n, sx, incx);
    let mut result = 0.0e0f32;

    if n <= 0 || incx <= 0 {
        return result;
    }

    if incx == 1 {
        let chunk_size = 64 * 1024 / 4;
        let m = n%chunk_size;
        
        if m != 0 {
            for i in 0..m {
                result = result + sx[i as usize].abs();
            }
            if n < chunk_size {
                return result;
            }
        }
        
        let temps: Vec<_> = sx[m as usize..]
            .par_chunks(chunk_size as usize)
            .map(|chunk| {
                let mut temp = std::arch::x86_64::_mm256_setzero_ps();
                for i in (0..chunk.len()).step_by(8) {
                    let sx_slice = std::arch::x86_64::_mm256_loadu_ps(chunk.as_ptr().add(i));
                    temp = std::arch::x86_64::_mm256_add_ps(temp, sx_slice);
                }
                temp
            })
            .collect();

        // store and cum
        let mut temp_array = [0.0f32;8];
        for each_temp in temps.iter() {
            std::arch::x86_64::_mm256_storeu_ps(temp_array.as_mut_ptr(), *each_temp);
            for i in temp_array.iter() {
                result += i;
            }
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
    feature = "sse4_2"))]
#[target_feature(enable = "sse")]
pub unsafe fn sasum_x86_64_sse(n: isize, sx: &[f32], incx: isize) -> f32 {
    sasum_check_arguments(n, sx, incx);
    let mut result = 0.0e0f32;
    let mut temp: std::arch::x86_64::__m128 = std::arch::x86_64::_mm_setzero_ps();

    if n <= 0 || incx <= 0 {
        return result;
    }
    if incx == 1 {
        let m = n%4;
        if m != 0 {
            for i in 0..m {
                result = result + sx[i as usize].abs();
            }
            if n < 4 {
                return result;
            }
        }
        for i in (m as usize..n as usize).step_by(4) {
            let sx_slice = std::arch::x86_64::_mm_loadu_ps(sx.as_ptr().add(i));
            temp = std::arch::x86_64::_mm_add_ps(temp, sx_slice);
        }
        // store and cum
        let mut temp_array = [0.0f32;4];
        std::arch::x86_64::_mm_storeu_ps(temp_array.as_mut_ptr(), temp);
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


#[cfg(
    all(any(
    feature = "sse", 
    feature = "sse2", 
    feature = "sse3", 
    feature = "ssse3", 
    feature = "sse4_1", 
    feature = "sse4_2"),
    feature = "multi_thread")
)]
#[target_feature(enable = "sse")]
pub unsafe fn sasum_x86_64_mt_sse(n: isize, sx: &[f32], incx: isize) -> f32 {
    // small size disable mt
    if 1 + (n-1)*incx.abs() < 100000 {
        return sasum_x86_64_sse(n, sx, incx);
    }

    sasum_check_arguments(n, sx, incx);
    let mut result = 0.0e0f32;

    if n <= 0 || incx <= 0 {
        return result;
    }

    if incx == 1 {
        let chunk_size = 64 * 1024 / 4;
        let m = n%chunk_size;
        
        if m != 0 {
            for i in 0..m {
                result = result + sx[i as usize].abs();
            }
            if n < chunk_size {
                return result;
            }
        }
        
        let temps: Vec<_> = sx[m as usize..]
            .par_chunks(chunk_size as usize)
            .map(|chunk| {
                let mut temp = std::arch::x86_64::_mm_setzero_ps();
                for i in (0..chunk.len()).step_by(4) {
                    let sx_slice = std::arch::x86_64::_mm_loadu_ps(chunk.as_ptr().add(i));
                    temp = std::arch::x86_64::_mm_add_ps(temp, sx_slice);
                }
                temp
            })
            .collect();

        // store and cum
        let mut temp_array = [0.0f32;4];
        for each_temp in temps.iter() {
            std::arch::x86_64::_mm_storeu_ps(temp_array.as_mut_ptr(), *each_temp);
            for i in temp_array.iter() {
                result += i;
            }
        }
    } else {
        for sxi in sx.iter().step_by(incx as usize) {
            result = result + sxi.abs();
        }
    }
    return result;
}