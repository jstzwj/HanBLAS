#![feature(test)]
extern crate test;

pub mod util;
pub mod kernel;
// level 1
pub mod asum;
pub mod axpy;
pub mod copy;
pub mod dot;
pub mod rotg;

// level 2


// level 3
pub mod gemm;

pub mod cblas;

#[allow(non_camel_case_types)]
type c16 = num_complex::Complex<half::f16>;
#[allow(non_camel_case_types)]
type c32 = num_complex::Complex<f32>;
#[allow(non_camel_case_types)]
type c64 = num_complex::Complex<f64>;


#[cfg(test)]
mod tests {
    use test::Bencher;
    use rand::Rng;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[bench]
    fn sasum_bench1(b: &mut Bencher) {
        let mut rng = rand::thread_rng();
        let mut sx = Vec::with_capacity(65536);
        for _i in 0..65536 {
            sx.push(rng.gen::<f32>());
        }
        b.iter(
            || {
                super::asum::sasum(1001, &sx[..3001], 3)
            }
        );
    }

    #[bench]
    fn sasum_bench2(b: &mut Bencher) {
        let mut rng = rand::thread_rng();
        let mut sx = Vec::with_capacity(65536);
        for _i in 0..65536 {
            sx.push(rng.gen::<f32>());
        }
        b.iter(
            || {
                super::asum::sasum(3001, &sx[..3001], 1)
            }
        );
    }
}
