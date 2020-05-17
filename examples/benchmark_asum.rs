use std::time::{Duration, Instant};
use rand::Rng;

use hanblas::HanInt;
use blas::*;

fn main() {
    let mut rng = rand::thread_rng();
    let test_num = 20;
    let mut wtr = csv::Writer::from_path("asum.csv").unwrap();

    wtr.write_record(&["size", "GFLOPS"]).unwrap();

    let mut size = 10000;
    while size < 1000000000 {
        let gflops = 2.0 * (size as f32) * 1.0e-09;

        let mut sx = Vec::with_capacity(size);
        for _i in 0..size {
            sx.push(rng.gen::<f32>());
        }
        
        let mut times = Vec::new();
        
        for _ in 0..test_num {
            let now = Instant::now();
            hanblas::asum::sasum(size as HanInt, &sx, 1);
            /*
            unsafe {
                blas::sasum(size as i32, &sx, 1);
            }
            */
            times.push(now.elapsed().as_nanos());
        }
        let min_time = times.iter().min().unwrap();
        let min_time_sec = (*min_time as f32) / 1e9;
        println!("{}", min_time_sec);

        wtr.write_record(&[size.to_string(), (gflops/min_time_sec).to_string()]).unwrap();
        println!("{}", size);

        size += 10i32.pow((size as f32).log10() as u32) as usize + 1;
    }
    wtr.flush().unwrap();
}