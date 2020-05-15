use std::time::{Duration, Instant};
use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let test_num = 1000;
    let mut wtr = csv::Writer::from_path("asum.csv").unwrap();

    wtr.write_record(&["size", "time"]).unwrap();

    let mut size = 0;
    while size < 10000000 {
        let mut sx = Vec::with_capacity(size);
        for _i in 0..size {
            sx.push(rng.gen::<f32>());
        }
        
        let now = Instant::now();
        for _ in 0..test_num {
            hanblas::asum::sasum(size as isize, &sx[..size], 1);
        }
        let time = (now.elapsed().as_nanos() as f32) / (test_num as f32);
        wtr.write_record(&[size.to_string(), time.to_string()]).unwrap();
        println!("{}", size);

        size += 10i32.pow((size as f32).log10() as u32) as usize + 1;
    }
    wtr.flush().unwrap();
    
}