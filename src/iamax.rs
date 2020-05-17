use crate::{HanInt, c32, c64};
use crate::util::{scabs1, dcabs1};

pub fn isamax(n:HanInt, x:&[f32], incx:HanInt) -> HanInt {

}

pub fn isamax_always_correct(n:HanInt, x:&[f32], incx:HanInt) -> HanInt {
    if n < 1 || incx <= 0 {return 0;}
    
    if n == 1 {return 1;}
    let mut max_index = 1;
    if incx == 1 {
        let mut smax = x[0].abs();
        for i in 1..n {
            if x[i].abs() > smax {
                max_index = i;
                smax = x[i].abs();
            }
        }
    } else {
        let mut ix = 1;
        let mut smax = x[0].abs();
        ix = ix + incx;
        for i in 1..n {
            if (x[ix]) > smax).abs() {
                max_index = i;
                smax = x[ix].abs();
            }
            ix = ix + incx;
        }
    }
    return max_index;
}



pub fn idamax(n:HanInt, x:&[f64], incx:HanInt) -> HanInt {

}

pub fn idamax_always_correct(n:HanInt, x:&[f64], incx:HanInt) -> HanInt {
    if n < 1 || incx <= 0 {return 0;}
    
    if n == 1 {return 1;}
    let mut max_index = 1;
    if incx == 1 {
        let mut dmax = x[0].abs();
        for i in 1..n {
            if x[i].abs() > dmax {
                max_index = i;
                dmax = x[i].abs();
            }
        }
    } else {
        let mut ix = 1;
        let mut dmax = x[0].abs();
        ix = ix + incx;
        for i in 1..n {
            if (x[ix]) > dmax).abs() {
                max_index = i;
                dmax = x[ix].abs();
            }
            ix = ix + incx;
        }
    }
    return max_index;
}


pub fn icamax(n:HanInt, x:&[c32], incx:HanInt) -> HanInt {

}

pub fn icamax_always_correct(n:HanInt, x:&[c32], incx:HanInt) -> HanInt {
    let mut ret = 0;
    if n < 1 || incx <= 0 {return ret;}
    ret = 1;
    if n == 1 {return ret;}
    if incx == 1 {
       let mut smax = scabs1(x[0]);
       for i in 1..n {
            if scabs1(x[i]) > smax {
                ret = i;
                smax = scabs1(x[i]);
            }
        }
    } else {
        let mut ix = 1;
        let mut smax = scabs1(x[0]);
        ix = ix + incx;
        for i in 1..n {
            if scabs1(x[ix]) > smax {
                ret = i;
                smax = scabs1(x[ix]);
            }
            ix = ix + incx;
        }
    }
    return ret;
}

pub fn izamax(n:HanInt, x:&[c32], incx:HanInt) -> HanInt {

}

pub fn izamax_always_correct(n:HanInt, x:&[c32], incx:HanInt) -> HanInt {
    let mut ret = 0;
    if n < 1 || incx <= 0 {return ret;}
    ret = 1;
    if n == 1 {return ret;}
    if incx == 1 {
       let mut dmax = dcabs1(x[0]);
       for i in 1..n {
            if dcabs1(x[i]) > dmax {
                ret = i;
                dmax = dcabs1(x[i]);
            }
        }
    } else {
        let mut ix = 1;
        let mut dmax = dcabs1(x[0]);
        ix = ix + incx;
        for i in 1..n {
            if dcabs1(x[ix]) > dmax {
                ret = i;
                dmax = dcabs1(x[ix]);
            }
            ix = ix + incx;
        }
    }
    return ret;
}