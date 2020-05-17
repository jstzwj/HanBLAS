use crate::{HanInt, c32, c64};

pub unsafe fn sasum_generic(n: HanInt, x: *const f32, incx: HanInt) -> f32 {
    let mut stemp = 0.0e0f32;
    if n <= 0 || incx <= 0 {
        return stemp;
    }

    if incx == 1 {
        let mut px = x;
        let px_end = x.offset(n as isize);
        let m = n%8;
        if m != 0 {
            for _ in 0..m {
                stemp = stemp + (*px).abs();
                px = px.offset(1);
            }
            if n < 8 {
                return stemp;
            }
        }
        while px < px_end {
            stemp = stemp + 
                (*px).abs() + (*px.offset(1)).abs() +
                (*px.offset(2)).abs() + (*px.offset(3)).abs() +
                (*px.offset(4)).abs() + (*px.offset(5)).abs() +
                (*px.offset(6)).abs() + (*px.offset(7)).abs();
            px = px.offset(8);
        }
    } else {
        let mut px = x;
        for _ in 0..n {
            stemp = stemp + (*px).abs();
            px = px.offset(incx as isize);
        }
    }
    return stemp;
}

pub unsafe fn dasum_generic(n: HanInt, x: *const f64, incx: HanInt) -> f64 {
    let mut stemp = 0.0e0f64;
    if n <= 0 || incx <= 0 {
        return stemp;
    }

    if incx == 1 {
        let m = n%8;
        if m != 0 {
            for i in 0..m {
                stemp = stemp + (*x.offset(i as isize)).abs();
            }
            if n < 8 {
                return stemp;
            }
        }
        for i in (m as isize..n as isize).step_by(8) {
            stemp = stemp + 
                (*x.offset(i)).abs() + (*x.offset(i+1)).abs() +
                (*x.offset(i+2)).abs() + (*x.offset(i+3)).abs() +
                (*x.offset(i+4)).abs() + (*x.offset(i+5)).abs() +
                (*x.offset(i+6)).abs() + (*x.offset(i+7)).abs();
        }
    } else {
        let mut px = x;
        for _ in 0..n {
            stemp = stemp + (*px).abs();
            px = px.offset(incx as isize);
        }
    }
    return stemp;
}


pub unsafe fn scasum_generic(n: HanInt, x: *const c32, incx: HanInt) -> f32 {
    let mut stemp = 0.0f32;
    if n <= 0 || incx <= 0 {return 0.0f32;}
    if incx == 1 {
        for i in 0..n as isize {
            stemp = stemp + (*x.offset(i)).re.abs() + (*x.offset(i)).im.abs();
        }
    } else {
        let nincx = n*incx;
        for i in (0..nincx as isize).step_by(incx as usize) {
            stemp = stemp + (*x.offset(i)).re.abs() + (*x.offset(i)).im.abs();
        }
    }
    return stemp;
}

pub unsafe fn dzasum_generic(n: HanInt, x: *const c64, incx: HanInt) -> f64 {
    let mut dtemp = 0.0f64;
    if n <= 0 || incx <= 0 {return 0.0f64;}
    if incx == 1 {
        for i in 0..n as isize {
            dtemp = dtemp + (*x.offset(i)).re.abs() + (*x.offset(i)).im.abs();
        }
    } else {
        let nincx = n*incx;
        for i in (0..nincx as isize).step_by(incx as usize) {
            dtemp = dtemp + (*x.offset(i)).re.abs() + (*x.offset(i)).im.abs();
        }
    }
    return dtemp;
}