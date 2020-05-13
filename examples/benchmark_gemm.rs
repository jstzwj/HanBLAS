use std::time::{Duration, Instant};
use rand::Rng;

fn main() {
    let nflops_per_cycle = 4;
    let nprocessors = 1;
    let GHz_of_processor = 2.0;

    let mut rng = rand::thread_rng();
    let test_num = 1000;
    let mut wtr = csv::Writer::from_path("gemm.csv").unwrap();

    wtr.write_record(&["size", "GFLOPS"]).unwrap();

    let mut size = 1;
    while size < 1000 {
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
        
        let now = Instant::now();
        for _ in 0..test_num {
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
                size as i32
            );
        }
        let time = (now.elapsed().as_nanos() as f32) / (test_num as f32);
        let gflops = (nprocessors as f32) * (nflops_per_cycle as f32) * GHz_of_processor * time / 1000.0;
        wtr.write_record(&[size.to_string(), gflops.to_string()]).unwrap();
        println!("{}", size);

        // size += 10i32.pow((size as f32).log10() as u32) as usize + 1;
        size += 10;
    }
    wtr.flush().unwrap();
    
}