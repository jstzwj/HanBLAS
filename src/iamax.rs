

pub fn isamax(n:i32, x:&[f32], incx:i32) -> i32 {

}

pub fn isamax_always_correct(n:i32, x:&[f32], incx:i32) -> i32 {
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