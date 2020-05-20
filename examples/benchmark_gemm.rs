use rand::Rng;
use std::time::Instant;

fn main() {
    // let nflops_per_cycle = 4;
    // let nprocessors = 1;
    // let GHz_of_processor = 2.0;

    let mut rng = rand::thread_rng();
    let test_num = 20;
    let mut wtr = csv::Writer::from_path("gemm.csv").unwrap();

    wtr.write_record(&["size", "GFLOPS"]).unwrap();

    for size in (40..800).step_by(40) {
        let m = size as f32;
        let n = size as f32;
        let k = size as f32;
        let gflops = 2.0 * m * n * k * 1.0e-09;

        let mut a = Vec::with_capacity(size);
        for _i in 0..size {
            for _j in 0..size {
                a.push(rng.gen::<f32>());
            }
        }

        let mut b = Vec::with_capacity(size);
        for _i in 0..size {
            for _j in 0..size {
                b.push(rng.gen::<f32>());
            }
        }

        let mut c = Vec::with_capacity(size);
        for _i in 0..size {
            for _j in 0..size {
                c.push(rng.gen::<f32>());
            }
        }

        let mut times = Vec::new();
        for _ in 0..test_num {
            let now = Instant::now();
            hanblas::gemm::sgemm(
                'n' as u8,
                'n' as u8,
                size as i32,
                size as i32,
                size as i32,
                1.0,
                &a,
                size as i32,
                &b,
                size as i32,
                0.0,
                &mut c,
                size as i32,
            );
            let time = now.elapsed().as_nanos();
            times.push(time);
        }

        let min_time = times.iter().min().unwrap();
        let min_time_sec = (*min_time as f32) / 1e9;
        wtr.write_record(&[size.to_string(), (gflops / min_time_sec).to_string()])
            .unwrap();

        // size += 10i32.pow((size as f32).log10() as u32) as usize + 1;
    }
    wtr.flush().unwrap();
}
