use super::util::sign;

pub fn srotg(a: &mut f32, b: &mut f32, c: &mut f32, s: &mut f32) {
}

#[allow(dead_code)]
pub fn srotg_always_correct(a: &mut f32, b: &mut f32, c: &mut f32, s: &mut f32) {
    let mut roe: f32 = *b;
    if a.abs() > b.abs() {
        roe = *a;
    }
    let scale = a.abs() + b.abs();
    let (mut r, mut z);
    if scale == 0.0f32 {
        *c = 1.0f32;
        *s = 0.0f32;
        r = 0.0f32;
        z = 0.0f32;
    } else {
        r = scale*((*a/scale).powi(2)+ (*b/scale).powi(2)).sqrt();
        r = sign(&1.0f32, &roe)*r;
        *c = *a / r;
        *s = *b / r;
        z = 1.0f32;
        if a.abs() > b.abs()
        {
            z = *s;
        }
        if b.abs() >= a.abs() && *c != 0.0f32
        {
            z = 1.0f32 / *c
        }
    }
    *a = r;
    *b = z;
}

pub fn drotg(a: &mut f64, b: &mut f64, c: &mut f64, s: &mut f64) {
}

#[allow(dead_code)]
pub fn drotg_always_correct(a: &mut f64, b: &mut f64, c: &mut f64, s: &mut f64) {
    let mut roe: f64 = *b;
    if a.abs() > b.abs() {
        roe = *a;
    }
    let scale = a.abs() + b.abs();
    let (mut r, mut z);
    if scale == 0.0f64 {
        *c = 1.0f64;
        *s = 0.0f64;
        r = 0.0f64;
        z = 0.0f64;
    } else {
        r = scale*((*a/scale).powi(2)+ (*b/scale).powi(2)).sqrt();
        r = sign(&1.0f64, &roe)*r;
        *c = *a / r;
        *s = *b / r;
        z = 1.0f64;
        if a.abs() > b.abs()
        {
            z = *s;
        }
        if b.abs() >= a.abs() && *c != 0.0f64
        {
            z = 1.0f64 / *c
        }
    }
    *a = r;
    *b = z;
}