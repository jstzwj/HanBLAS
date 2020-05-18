use std::time::{Duration, Instant};
use rand::Rng;

use hanblas::HanInt;

fn main() {
    let sx: Vec<f32> = vec![
        0.0, -0.0, 4.0, 4.0,
        1.0, 1.0, 1.0, 1.0,
        1.0, 1.0, 1.0, 1.0,
        1.0, 1.0, 1.0, 1.0,
        1.0, 1.0, 1.0, 1.0,
        1.0, 1.0, 1.0, 1.0,
        1.0, 1.0, 1.0, 1.0,
        1.0, 1.0, 1.0, 1.0];
    let out = hanblas::asum::sasum(30, &sx[1..], 1);
    println!("{}", out);
}