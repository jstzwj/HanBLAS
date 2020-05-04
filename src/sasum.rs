

pub fn sasum(n: i32, sx: &[f32], incx: i32) -> f32 {
    let mut ret = 0.0e0f32;
    let mut stemp = 0.0e0f32;
    if n <= 0 || incx <= 0 {
        return ret;
    }
    if incx == 1 {
        let m = n%6;
        if m != 0 {
            for i in 0..m {
                stemp = stemp + sx[i as usize].abs();
            }
            if n < 6 {
                ret = stemp;
                return ret;
            }
        }
        for i in (m..n).step_by(6) {
            stemp = stemp + sx[i as usize].abs() + sx[i as usize + 1].abs() +
                sx[i as usize + 2].abs() + sx[i as usize + 3].abs() +
                sx[i as usize + 4].abs() + sx[i as usize + 5].abs();
        }
    } else {
        let nincx = n*incx;
        for i in (0..nincx).step_by(incx as usize) {
            stemp = stemp + sx[i as usize].abs();
        }
    }
    ret = stemp;
    return ret;
}