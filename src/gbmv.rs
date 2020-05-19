use crate::{HanInt, c32, c64};

pub fn sgbmv(
    trans: u8, 
    m: HanInt, 
    n: HanInt, 
    kl: HanInt, 
    ku: HanInt, 
    alpha: f32, 
    a: &[f32], 
    lda: HanInt, 
    x: &[f32], 
    incx: HanInt, 
    beta: f32, 
    y: &mut [f32], 
    incy: HanInt
) {

}


pub fn dgbmv(
    trans: u8, 
    m: HanInt, 
    n: HanInt, 
    kl: HanInt, 
    ku: HanInt, 
    alpha: f64, 
    a: &[f64], 
    lda: HanInt, 
    x: &[f64], 
    incx: HanInt, 
    beta: f64, 
    y: &mut [f64], 
    incy: HanInt
) {

}

pub fn cgbmv(
    trans: u8, 
    m: HanInt, 
    n: HanInt, 
    kl: HanInt, 
    ku: HanInt, 
    alpha: c32, 
    a: &[c32], 
    lda: HanInt, 
    x: &[c32], 
    incx: HanInt, 
    beta: c32, 
    y: &mut [c32], 
    incy: HanInt
) {

}

pub fn zgbmv(
    trans: u8, 
    m: HanInt, 
    n: HanInt, 
    kl: HanInt, 
    ku: HanInt, 
    alpha: c64, 
    a: &[c64], 
    lda: HanInt, 
    x: &[c64], 
    incx: HanInt, 
    beta: c64, 
    y: &mut [c64], 
    incy: HanInt
) {

}