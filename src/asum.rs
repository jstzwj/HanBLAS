use crate::{HanInt, c32, c64};

pub fn sasum(n: HanInt, x: &[f32], incx: HanInt) -> f32 {
    #[cfg(target_arch = "x86_64")]
    unsafe {
        #[cfg(not(feature = "multi_thread"))]
        {
            #[cfg(feature = "avx2")]
            return crate::kernel::x86_64::asum::sasum_x86_64_avx(n, x, incx);

            #[cfg(feature = "avx")]
            return crate::kernel::x86_64::asum::sasum_x86_64_avx(n, x, incx);

            #[cfg(feature = "sse4_2")]
            return crate::kernel::x86_64::asum::sasum_x86_64_sse(n, x, incx);

            #[cfg(feature = "sse4_1")]
            return crate::kernel::x86_64::asum::sasum_x86_64_sse(n, x, incx);

            #[cfg(feature = "ssse3")]
            return crate::kernel::x86_64::asum::sasum_x86_64_sse(n, x, incx);

            #[cfg(feature = "sse3")]
            return crate::kernel::x86_64::asum::sasum_x86_64_sse(n, x, incx);

            #[cfg(feature = "sse2")]
            return crate::kernel::x86_64::asum::sasum_x86_64_sse(n, x, incx);

            #[cfg(feature = "sse")]
            return crate::kernel::x86_64::asum::sasum_x86_64_sse(n, x, incx);
        }

        #[cfg(feature = "multi_thread")]
        {
            #[cfg(feature = "avx2")]
            return crate::kernel::x86_64::asum::sasum_x86_64_mt_avx(n, x, incx);

            #[cfg(feature = "avx")]
            return crate::kernel::x86_64::asum::sasum_x86_64_mt_avx(n, x, incx);

            #[cfg(feature = "sse4_2")]
            return crate::kernel::x86_64::asum::sasum_x86_64_mt_sse(n, x, incx);

            #[cfg(feature = "sse4_1")]
            return crate::kernel::x86_64::asum::sasum_x86_64_mt_sse(n, x, incx);

            #[cfg(feature = "ssse3")]
            return crate::kernel::x86_64::asum::sasum_x86_64_mt_sse(n, x, incx);

            #[cfg(feature = "sse3")]
            return crate::kernel::x86_64::asum::sasum_x86_64_mt_sse(n, x, incx);

            #[cfg(feature = "sse2")]
            return crate::kernel::x86_64::asum::sasum_x86_64_mt_sse(n, x, incx);

            #[cfg(feature = "sse")]
            return crate::kernel::x86_64::asum::sasum_x86_64_mt_sse(n, x, incx);
        }
    }

    #[cfg(target_arch = "x86")]
    unsafe {
        #[cfg(feature = "avx2")]
        return crate::kernel::x86::asum::sasum_x86_avx2(n, x, incx);
    }

    unsafe {
        // return crate::kernel::generic::asum::sasum_generic(n, x.as_ptr(), incx);
        return sasum_always_correct(n, x, incx);
    }
}

pub fn sasum_always_correct(n: HanInt, x: &[f32], incx: HanInt) -> f32 {
    assert!(x.len() as HanInt == 1 + (n-1)*incx.abs(), "the dimension of x is not 1+(n-1)*abs(incx)");
    let mut stemp = 0.0e0f32;
    if n <= 0 || incx <= 0 {
        return stemp;
    }
    if incx == 1 {
        let m = n%8;
        if m != 0 {
            for i in 0..m {
                stemp = stemp + x[i as usize].abs();
            }
            if n < 8 {
                return stemp;
            }
        }
        for i in (m as usize..n as usize).step_by(8) {
            stemp = stemp + x[i].abs() + x[i + 1].abs() +
                x[i + 2].abs() + x[i + 3].abs() +
                x[i + 4].abs() + x[i + 5].abs() +
                x[i + 6].abs() + x[i + 7].abs();
        }
    } else {
        let nincx = n*incx;
        for i in (0..nincx as usize).step_by(incx as usize) {
            stemp = stemp + x[i].abs();
        }
    }
    return stemp;
}

pub fn dasum(n: HanInt, x: &[f64], incx: HanInt) -> f64 {
    unsafe {
        return crate::kernel::generic::asum::dasum_generic(n, x.as_ptr(), incx);
    }
}

pub fn dasum_always_correct(n: HanInt, x: &[f64], incx: HanInt) -> f64 {
    assert!(x.len() as HanInt == 1 + (n-1)*incx.abs(), "the dimension of x is not 1+(n-1)*abs(incx)");
    let mut ret = 0.0e0f64;
    let mut stemp = 0.0e0f64;
    if n <= 0 || incx <= 0 {
        return ret;
    }
    if incx == 1 {
        let m = n%6;
        if m != 0 {
            for i in 0..m {
                stemp = stemp + x[i as usize].abs();
            }
            if n < 6 {
                ret = stemp;
                return ret;
            }
        }
        for i in (m..n).step_by(6) {
            stemp = stemp + x[i as usize].abs() + x[i as usize + 1].abs() +
                x[i as usize + 2].abs() + x[i as usize + 3].abs() +
                x[i as usize + 4].abs() + x[i as usize + 5].abs();
        }
    } else {
        let nincx = n*incx;
        for i in (0..nincx).step_by(incx as usize) {
            stemp = stemp + x[i as usize].abs();
        }
    }
    ret = stemp;
    return ret;
}

pub fn scasum(n: HanInt, x: &[c32], incx: HanInt) -> f32 {
    unsafe {
        return crate::kernel::generic::asum::scasum_generic(n, x.as_ptr(), incx);
    }
}

pub fn scasum_always_correct(n: HanInt, x: &[c32], incx: HanInt) -> f32 {
    let mut stemp = 0.0f32;
    if n <= 0 || incx <= 0 {return 0.0f32;}
    if incx == 1 {
        for i in 0..n as usize {
            stemp = stemp + x[i].re.abs() + x[i].im.abs();
        }
    } else {
        let nincx = n*incx;
        for i in (0..nincx as usize).step_by(incx as usize) {
            stemp = stemp + x[i].re.abs() + x[i].im.abs();
        }
    }
    return stemp;
}

pub fn dzasum(n: HanInt, x: &[c64], incx: HanInt) -> f64 {
    unsafe {
        return crate::kernel::generic::asum::dzasum_generic(n, x.as_ptr(), incx);
    }
}

pub fn dzasum_always_correct(n: HanInt, x: &[c64], incx: HanInt) -> f64 {
    let mut dtemp = 0.0f64;
    if n <= 0 || incx <= 0 {return 0.0f64;}
    if incx == 1 {
        for i in 0..n as usize {
            dtemp = dtemp + x[i].re.abs() + x[i].im.abs();
        }
    } else {
        let nincx = n*incx;
        for i in (0..nincx as usize).step_by(incx as usize) {
            dtemp = dtemp + x[i].re.abs() + x[i].im.abs();
        }
    }
    return dtemp;
}