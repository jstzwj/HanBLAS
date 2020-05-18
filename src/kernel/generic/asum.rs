use crate::{HanInt, c32, c64};

pub unsafe fn sasum_generic(n: HanInt, x: *const f32, incx: HanInt) -> f32 {
    let mut ret = 0.0e0f32;
    if n <= 0 || incx <= 0 {
        return ret;
    }

    if incx == 1 {
        let mut px = x;
        let px_end = x.offset(n as isize);

        let m = n%4;
        if m != 0 {
            for _ in 0..m {
                ret = ret + (*px).abs();
                px = px.offset(1);
            }
            if n < 4 {
                return ret;
            }
        }

        let mut sum1 = 0.0f32;
        let mut sum2 = 0.0f32;
        let mut sum3 = 0.0f32;
        let mut sum4 = 0.0f32;

        while px < px_end {
            sum1 = sum1 + (*px).abs();
            sum2 = sum2 + (*px.offset(1)).abs();
            sum3 = sum3 + (*px.offset(2)).abs();
            sum4 = sum4 + (*px.offset(3)).abs();

            px = px.offset(4);
        }
        
        ret = ret + sum1 + sum2 + sum3 + sum4;
    } else {
        let mut px = x;
        let px_end = x.offset((n*incx) as isize);

        let m = n%4;
        if m != 0 {
            for _ in 0..m {
                ret = ret + (*px).abs();
                px = px.offset(incx as isize);
            }
            if n < 4 {
                return ret;
            }
        }

        let mut sum1 = 0.0f32;
        let mut sum2 = 0.0f32;
        let mut sum3 = 0.0f32;
        let mut sum4 = 0.0f32;
        
        while px < px_end {
            sum1 = sum1 + (*px).abs();
            px = px.offset(incx as isize);
            sum2 = sum2 + (*px).abs();
            px = px.offset(incx as isize);
            sum3 = sum3 + (*px).abs();
            px = px.offset(incx as isize);
            sum4 = sum4 + (*px).abs();
            px = px.offset(incx as isize);
        }

        ret = ret + sum1 + sum2 + sum3 + sum4;
    }
    return ret;
}

pub unsafe fn dasum_generic(n: HanInt, x: *const f64, incx: HanInt) -> f64 {
    let mut ret = 0.0e0f64;
    if n <= 0 || incx <= 0 {
        return ret;
    }

    if incx == 1 {
        let mut px = x;
        let px_end = x.offset(n as isize);

        let m = n%4;
        if m != 0 {
            for _ in 0..m {
                ret = ret + (*px).abs();
                px = px.offset(1);
            }
            if n < 4 {
                return ret;
            }
        }

        let mut sum1 = 0.0f64;
        let mut sum2 = 0.0f64;
        let mut sum3 = 0.0f64;
        let mut sum4 = 0.0f64;

        while px < px_end {
            sum1 = sum1 + (*px).abs();
            sum2 = sum2 + (*px.offset(1)).abs();
            sum3 = sum3 + (*px.offset(2)).abs();
            sum4 = sum4 + (*px.offset(3)).abs();

            px = px.offset(4);
        }
        
        ret = ret + sum1 + sum2 + sum3 + sum4;
    } else {
        let mut px = x;
        let px_end = x.offset((n*incx) as isize);

        let m = n%4;
        if m != 0 {
            for _ in 0..m {
                ret = ret + (*px).abs();
                px = px.offset(incx as isize);
            }
            if n < 4 {
                return ret;
            }
        }

        let mut sum1 = 0.0f64;
        let mut sum2 = 0.0f64;
        let mut sum3 = 0.0f64;
        let mut sum4 = 0.0f64;
        
        while px < px_end {
            sum1 = sum1 + (*px).abs();
            px = px.offset(incx as isize);
            sum2 = sum2 + (*px).abs();
            px = px.offset(incx as isize);
            sum3 = sum3 + (*px).abs();
            px = px.offset(incx as isize);
            sum4 = sum4 + (*px).abs();
            px = px.offset(incx as isize);
        }

        ret = ret + sum1 + sum2 + sum3 + sum4;
    }
    return ret;
}


pub unsafe fn scasum_generic(n: HanInt, x: *const c32, incx: HanInt) -> f32 {
    let mut ret = 0.0f32;
    if n <= 0 || incx <= 0 {return ret;}

    let mut px = x;
    let px_end = x.offset((n*incx) as isize);
    if incx == 1 {
        while px < px_end {
            ret = ret + (*px).re.abs() + (*px).im.abs();
            px = px.offset(1);
        }
    } else {
        while px < px_end {
            ret = ret + (*px).re.abs() + (*px).im.abs();
            px = px.offset(incx as isize);
        }
    }
    return ret;
}

pub unsafe fn dzasum_generic(n: HanInt, x: *const c64, incx: HanInt) -> f64 {
    let mut ret = 0.0f64;
    if n <= 0 || incx <= 0 {return ret;}

    let mut px = x;
    let px_end = x.offset((n*incx) as isize);
    if incx == 1 {
        while px < px_end {
            ret = ret + (*px).re.abs() + (*px).im.abs();
            px = px.offset(1);
        }
    } else {
        while px < px_end {
            ret = ret + (*px).re.abs() + (*px).im.abs();
            px = px.offset(incx as isize);
        }
    }
    return ret;
}