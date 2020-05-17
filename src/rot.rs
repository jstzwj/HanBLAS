use crate::{HanInt, c32, c64};

pub fn srot(n: HanInt, x:&mut[f32], incx:HanInt, y:&mut[f32], incy:HanInt, c:f32, s:f32) {

}

pub fn srot_always_correct(n: HanInt, x:&mut[f32], incx:HanInt, y:&mut[f32], incy:HanInt, c:f32, s:f32) {
    if n <= 0 {return;}
    if incx == 1 && incy == 1 {
        for i in 0..n as usize {
            let stemp = c*x[i] + s*y[i];
            y[i] = c*y[i] - s*x[i];
            x[i] = stemp;
        }
    } else {
        let mut ix:usize = 1;
        let mut iy:usize = 1;
        if incx < 0 {
            ix = ((-n+1)*incx + 1) as usize;
        }
        if incy < 0 {
            iy = ((-n+1)*incy + 1) as usize;
        }
        for _ in 0..n {
            let stemp = c*x[ix] + s*y[iy];
            y[iy] = c*y[iy] - s*x[ix];
            x[ix] = stemp;
            ix = ix + incx as usize;
            iy = iy + incy as usize;
        }
    }
}


pub fn drot(n: HanInt, x:&[f64], incx:HanInt, y:&[f64], incy:HanInt, c:f64, s:f64) {
    
}

pub fn csrot(n: HanInt, x:&[c32], incx:HanInt, y:&[c32], incy:HanInt, c:f32, s:f32) {

}

pub fn dzrot(n: HanInt, x:&[c64], incx:HanInt, y:&[c64], incy:HanInt, c:f32, s:f32) {

}