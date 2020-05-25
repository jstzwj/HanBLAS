use crate::{HanInt, c32, c64};

pub fn sscal(n: HanInt, a: f32, x: &mut [f32], incx: HanInt) {

}

pub fn sscal_always_correct(n: HanInt, a: f32, x: &mut [f32], incx: HanInt) {
    if n <= 0 || incx <= 0 {return;}
    let nincx = n*incx;
    for i in (0..nincx as usize).step_by(incx as usize) {
        x[i] = a*x[i];
    }
}

pub fn dscal(n: HanInt, a: f64, x: &mut [f64], incx: HanInt) {

}

pub fn dscal_always_correct(n: HanInt, a: f32, x: &mut [f32], incx: HanInt) {
    if n <= 0 || incx <= 0 {return;}
    let nincx = n*incx;
    for i in (0..nincx as usize).step_by(incx as usize) {
        x[i] = a*x[i];
    }
}

pub fn csscal(n: HanInt, a: f32, x: &mut [c32], incx: HanInt) {

}

pub fn csscal_always_correct(n: HanInt, a: f32, x: &mut [c32], incx: HanInt) {
    if n <= 0 || incx <= 0 {return;}
    let nincx = n*incx;
    for i in (0..nincx as usize).step_by(incx as usize) {
        x[i] = a*x[i];
    }
}

pub fn zdscal(n: HanInt, a: f64, x: &mut [c64], incx: HanInt) {

}

pub fn zdscal_always_correct(n: HanInt, a: f64, x: &mut [c64], incx: HanInt) {
    if n <= 0 || incx <= 0 {return;}
    let nincx = n*incx;
    for i in (0..nincx as usize).step_by(incx as usize) {
        x[i] = a*x[i];
    }
}