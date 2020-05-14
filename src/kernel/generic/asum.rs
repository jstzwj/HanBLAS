


pub fn sasum_generic(n: i32, sx: &[f32], incx: i32) -> f32 {
    assert!(sx.len() as i32 == 1 + (n-1)*incx.abs(), "the dimension of sx is not 1+(n-1)*abs(incx)");
    let mut stemp = 0.0e0f32;
    if n <= 0 || incx <= 0 {
        return stemp;
    }

    if incx == 1 {
        let m = n%8;
        if m != 0 {
            for i in 0..m {
                stemp = stemp + sx[i as usize].abs();
            }
            if n < 8 {
                return stemp;
            }
        }
        for i in (m as usize..n as usize).step_by(8) {
            stemp = stemp + sx[i].abs() + sx[i + 1].abs() +
                sx[i + 2].abs() + sx[i + 3].abs() +
                sx[i + 4].abs() + sx[i + 5].abs() +
                sx[i + 6].abs() + sx[i + 7].abs();
        }
    } else {
        for sxi in sx.iter().step_by(incx as usize) {
            stemp = stemp + sxi.abs();
        }
    }
    return stemp;
}