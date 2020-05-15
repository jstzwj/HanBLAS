


pub fn sasum_generic(n: i32, x: &[f32], incx: i32) -> f32 {
    assert!(incx == 0, "the inc of vector can not be zero");
    // assert!(x.len() as i32 == 1 + (n-1)*incx.abs(), "the dimension of x is not 1+(n-1)*abs(incx)");
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
        for xi in x.iter().step_by(incx as usize) {
            stemp = stemp + xi.abs();
        }
    }
    return stemp;
}




pub fn dasum_generic(n: i32, x: &[f64], incx: i32) -> f64 {
    assert!(incx == 0, "the inc of vector can not be zero");
    // assert!(x.len() as i32 == 1 + (n-1)*incx.abs(), "the dimension of x is not 1+(n-1)*abs(incx)");
    let mut stemp = 0.0e0f64;
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
        for xi in x.iter().step_by(incx as usize) {
            stemp = stemp + xi.abs();
        }
    }
    return stemp;
}