
#[cfg(target_arch = "x86")]
use std::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

pub fn sasum(n: i32, sx: &[f32], incx: i32) -> f32 {
    #[cfg(target_arch = "x86_64")]
    unsafe {
        #[cfg(feature = "avx2")]
        return sasum_x86_64_avx2(n, sx, incx);

        #[cfg(feature = "avx")]
        return sasum_x86_64_avx(n, sx, incx);

        #[cfg(feature = "sse4_2")]
        return sasum_x86_64_sse4_2(n, sx, incx);
    }

    #[cfg(target_arch = "x86")]
    unsafe {
        #[cfg(feature = "avx2")]
        return sasum_x86_avx2(n, sx, incx);
    }

    return sasum_fallback(n, sx, incx);
}

fn sasum_fallback(n: i32, sx: &[f32], incx: i32) -> f32 {
    assert!(sx.len() as i32 == 1 + (n-1)*incx.abs(), "the dimension of sx is not 1+(n-1)*abs(incx)");
    let mut stemp = 0.0e0f32;
    if n <= 0 || incx <= 0 {
        return stemp;
    }
    if incx == 1 {
        let m = n%8;
        if m != 0 {
            for i in 0..m {
                stemp = stemp + sx[i as usize].abs();
            }
            if n < 8 {
                return stemp;
            }
        }
        for i in (m as usize..n as usize).step_by(8) {
            stemp = stemp + sx[i].abs() + sx[i + 1].abs() +
                sx[i + 2].abs() + sx[i + 3].abs() +
                sx[i + 4].abs() + sx[i + 5].abs() +
                sx[i + 6].abs() + sx[i + 7].abs();
        }
    } else {
        for sxi in sx.iter().step_by(incx as usize) {
            stemp = stemp + sxi.abs();
        }
    }
    return stemp;
}

#[cfg(all(target_arch = "x86_64", feature = "avx2"))]
#[target_feature(enable = "avx2")]
unsafe fn sasum_x86_64_avx2(n: i32, sx: &[f32], incx: i32) -> f32 {
    assert!(sx.len() as i32 == 1 + (n-1)*incx.abs(), "the dimension of sx is not 1+(n-1)*abs(incx)");
    let mut result = 0.0e0f32;
    let mut temp: std::arch::x86_64::__m256 = std::arch::x86_64::_mm256_setzero_ps();

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
        for sxi in sx.iter().step_by(incx as usize) {
            result = result + sxi.abs();
        }
    }
    return result;
}


#[cfg(all(target_arch = "x86", feature = "avx2"))]
#[target_feature(enable = "avx2")]
unsafe fn sasum_x86_avx2(n: i32, sx: &[f32], incx: i32) -> f32 {
    assert!(sx.len() as i32 == 1 + (n-1)*incx.abs(), "the dimension of sx is not 1+(n-1)*abs(incx)");
    let mut result = 0.0e0f32;
    let mut temp: std::arch::x86::__m256 = std::arch::x86::_mm256_setzero_ps();

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
        for i in (m as usize..n as usize).step_by(8) {
            let sx_slice = std::arch::x86::_mm256_loadu_ps(&sx[i]);
            temp = std::arch::x86::_mm256_add_ps(temp, sx_slice);
        }
        // store and cum
        let mut temp_array = [0.0f32;8];
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


pub fn dasum(n: i32, sx: &[f64], incx: i32) -> f64 {
    assert!(sx.len() as i32 == 1 + (n-1)*incx.abs(), "the dimension of sx is not 1+(n-1)*abs(incx)");
    let mut ret = 0.0e0f64;
    let mut stemp = 0.0e0f64;
    if n <= 0 || incx <= 0 {
        return ret;
    }
    if incx == 1 {
        let m = n%6;
        if m != 0 {
            for i in 0..m {
                stemp = stemp + sx[i as usize].abs();
            }
            if n < 6 {
                ret = stemp;
                return ret;
            }
        }
        for i in (m..n).step_by(6) {
            stemp = stemp + sx[i as usize].abs() + sx[i as usize + 1].abs() +
                sx[i as usize + 2].abs() + sx[i as usize + 3].abs() +
                sx[i as usize + 4].abs() + sx[i as usize + 5].abs();
        }
    } else {
        let nincx = n*incx;
        for i in (0..nincx).step_by(incx as usize) {
            stemp = stemp + sx[i as usize].abs();
        }
    }
    ret = stemp;
    return ret;
}