use crate::{HanInt, c32, c64};
use crate::util::{scabs1, dcabs1};

pub fn isamin(n:HanInt, x:&[f32], incx:HanInt) -> HanInt {

}

pub fn isamin_always_correct(n:HanInt, x:&[f32], incx:HanInt) -> HanInt {
    if n < 1 || incx <= 0 {return 0;}
    
    if n == 1 {return 1;}
    let mut min_index = 1;
    if incx == 1 {
        let mut smin = x[0].abs();
        for i in 1..n {
            if x[i].abs() < smin {
                min_index = i;
                smin = x[i].abs();
            }
        }
    } else {
        let mut ix = 1;
        let mut smin = x[0].abs();
        ix = ix + incx;
        for i in 1..n {
            if (x[ix]) < smin).abs() {
                min_index = i;
                smin = x[ix].abs();
            }
            ix = ix + incx;
        }
    }
    return min_index;
}



pub fn idamin(n:HanInt, x:&[f64], incx:HanInt) -> HanInt {

}

pub fn idamin_always_correct(n:HanInt, x:&[f64], incx:HanInt) -> HanInt {
    if n < 1 || incx <= 0 {return 0;}
    
    if n == 1 {return 1;}
    let mut min_index = 1;
    if incx == 1 {
        let mut dmin = x[0].abs();
        for i in 1..n {
            if x[i].abs() < dmin {
                min_index = i;
                dmin = x[i].abs();
            }
        }
    } else {
        let mut ix = 1;
        let mut dmin = x[0].abs();
        ix = ix + incx;
        for i in 1..n {
            if (x[ix]) < dmin).abs() {
                min_index = i;
                dmin = x[ix].abs();
            }
            ix = ix + incx;
        }
    }
    return min_index;
}


pub fn icamin(n:HanInt, x:&[c32], incx:HanInt) -> HanInt {

}

pub fn icamin_always_correct(n:HanInt, x:&[c32], incx:HanInt) -> HanInt {
    let mut ret = 0;
    if n < 1 || incx <= 0 {return ret;}
    ret = 1;
    if n == 1 {return ret;}
    if incx == 1 {
       let mut smin = scabs1(x[0]);
       for i in 1..n {
            if scabs1(x[i]) < smin {
                ret = i;
                smin = scabs1(x[i]);
            }
        }
    } else {
        let mut ix = 1;
        let mut smin = scabs1(x[0]);
        ix = ix + incx;
        for i in 1..n {
            if scabs1(x[ix]) < smin {
                ret = i;
                smin = scabs1(x[ix]);
            }
            ix = ix + incx;
        }
    }
    return ret;
}

pub fn izamin(n:HanInt, x:&[c32], incx:HanInt) -> HanInt {

}

pub fn izamin_always_correct(n:HanInt, x:&[c32], incx:HanInt) -> HanInt {
    let mut ret = 0;
    if n < 1 || incx <= 0 {return ret;}
    ret = 1;
    if n == 1 {return ret;}
    if incx == 1 {
       let mut dmin = dcabs1(x[0]);
       for i in 1..n {
            if dcabs1(x[i]) < dmin {
                ret = i;
                dmin = dcabs1(x[i]);
            }
        }
    } else {
        let mut ix = 1;
        let mut dmin = dcabs1(x[0]);
        ix = ix + incx;
        for i in 1..n {
            if dcabs1(x[ix]) < dmin {
                ret = i;
                dmin = dcabs1(x[ix]);
            }
            ix = ix + incx;
        }
    }
    return ret;
}