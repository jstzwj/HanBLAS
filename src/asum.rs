
pub fn sasum(n: i32, sx: &[f32], incx: i32) -> f32 {
    #[cfg(target_arch = "x86_64")]
    unsafe {
        #[cfg(not(feature = "multi_thread"))]
        {
            #[cfg(feature = "avx2")]
            return crate::kernel::x86_64::asum::sasum_x86_64_avx(n, sx, incx);

            #[cfg(feature = "avx")]
            return crate::kernel::x86_64::asum::sasum_x86_64_avx(n, sx, incx);

            #[cfg(feature = "sse4_2")]
            return crate::kernel::x86_64::asum::sasum_x86_64_sse(n, sx, incx);

            #[cfg(feature = "sse4_1")]
            return crate::kernel::x86_64::asum::sasum_x86_64_sse(n, sx, incx);

            #[cfg(feature = "ssse3")]
            return crate::kernel::x86_64::asum::sasum_x86_64_sse(n, sx, incx);

            #[cfg(feature = "sse3")]
            return crate::kernel::x86_64::asum::sasum_x86_64_sse(n, sx, incx);

            #[cfg(feature = "sse2")]
            return crate::kernel::x86_64::asum::sasum_x86_64_sse(n, sx, incx);

            #[cfg(feature = "sse")]
            return crate::kernel::x86_64::asum::sasum_x86_64_sse(n, sx, incx);
        }

        #[cfg(feature = "multi_thread")]
        {
            #[cfg(feature = "avx2")]
            return crate::kernel::x86_64::asum::sasum_x86_64_mt_avx(n, sx, incx);

            #[cfg(feature = "avx")]
            return crate::kernel::x86_64::asum::sasum_x86_64_mt_avx(n, sx, incx);

            #[cfg(feature = "sse4_2")]
            return crate::kernel::x86_64::asum::sasum_x86_64_mt_sse(n, sx, incx);

            #[cfg(feature = "sse4_1")]
            return crate::kernel::x86_64::asum::sasum_x86_64_mt_sse(n, sx, incx);

            #[cfg(feature = "ssse3")]
            return crate::kernel::x86_64::asum::sasum_x86_64_mt_sse(n, sx, incx);

            #[cfg(feature = "sse3")]
            return crate::kernel::x86_64::asum::sasum_x86_64_mt_sse(n, sx, incx);

            #[cfg(feature = "sse2")]
            return crate::kernel::x86_64::asum::sasum_x86_64_mt_sse(n, sx, incx);

            #[cfg(feature = "sse")]
            return crate::kernel::x86_64::asum::sasum_x86_64_mt_sse(n, sx, incx);
        }
    }

    #[cfg(target_arch = "x86")]
    unsafe {
        #[cfg(feature = "avx2")]
        return crate::kernel::x86::asum::sasum_x86_avx2(n, sx, incx);
    }

    return crate::kernel::generic::asum::sasum_generic(n, sx, incx);
}

pub fn sasum_always_correct(n: i32, sx: &[f32], incx: i32) -> f32 {
    assert!(sx.len() as i32 == 1 + (n-1)*incx.abs(), "the dimension of sx is not 1+(n-1)*abs(incx)");
    let mut stemp = 0.0e0f32;
    if n <= 0 || incx <= 0 {
        return stemp;
    }
    if incx == 1 {
        let m = n%6;
        if m != 0 {
            for i in 0..m {
                stemp = stemp + sx[i as usize].abs();
            }
            if n < 6 {
                return stemp;
            }
        }
        for i in (m as usize..n as usize).step_by(6) {
            stemp = stemp + sx[i].abs() + sx[i + 1].abs() +
                sx[i + 2].abs() + sx[i + 3].abs() +
                sx[i + 4].abs() + sx[i + 5].abs() +
                sx[i + 6].abs() + sx[i + 7].abs();
        }
    } else {
        let nincx = n*incx;
        for i in (0..nincx as usize).step_by(incx as usize) {
            stemp = stemp + sx[i].abs();
        }
    }
    return stemp;
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