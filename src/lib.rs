
pub mod util;
pub mod kernel;
// level 1
pub mod asum;
pub mod axpy;
pub mod copy;
pub mod dot;
pub mod dotu;
pub mod dotc;
pub mod rot;
pub mod rotm;
pub mod rotg;
pub mod rotmg;

// level 2


// level 3
pub mod gemm;

pub mod cblas;


// custom types
pub type HanInt = i32;

#[allow(non_camel_case_types)]
pub type c16 = num_complex::Complex<half::f16>;
#[allow(non_camel_case_types)]
pub type c32 = num_complex::Complex<f32>;
#[allow(non_camel_case_types)]
pub type c64 = num_complex::Complex<f64>;


#[cfg(test)]
mod tests {
    use rand::Rng;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_sasum_1() {
        let mut rng = rand::thread_rng();
        let mut sx = Vec::with_capacity(65536);
        for _i in 0..65536 {
            sx.push(rng.gen::<f32>());
        }

        let result = super::asum::sasum(1001, &sx[..3001], 3);
        let result_correct = super::asum::sasum_always_correct(1001, &sx[..3001], 3);
        assert!((result - result_correct).abs() < 1e-3);
    }

    #[test]
    fn test_sasum_2() {
        let mut rng = rand::thread_rng();
        let mut sx = Vec::with_capacity(65536);
        for _i in 0..65536 {
            sx.push(rng.gen::<f32>());
        }

        let result = super::asum::sasum(1001, &sx[..1001], 1);
        let result_correct = super::asum::sasum_always_correct(1001, &sx[..1001], 1);
        assert!((result - result_correct).abs() < 1e-3);
    }

    #[test]
    fn test_gemm_1() {
        let mut rng = rand::thread_rng();

        let m = 120;
        let n = 340;
        let k = 64;
        
        let mut a = Vec::with_capacity(m*k);
        for _i in 0..m {
            for _j in 0..n {
                a.push(rng.gen::<f32>());
            }
        }

        let mut b = Vec::with_capacity(k*n);
        for _i in 0..k {
            for _j in 0..n {
                b.push(rng.gen::<f32>());
            }
        }

        let mut c1 = Vec::with_capacity(m*n);
        for _i in 0..m {
            for _j in 0..n {
                c1.push(rng.gen::<f32>());
            }
        }

        let mut c2 = c1.clone();

        self::super::gemm::sgemm(
            'n' as u8,
            'n' as u8,
            m as i32,
            n as i32,
            k as i32,
            1.0,
            &a,
            m as i32,
            &b,
            k as i32,
            0.0,
            &mut c1,
            n as i32
        );
        self::super::gemm::sgemm_always_correct(
            'n' as u8,
            'n' as u8,
            m as i32,
            n as i32,
            k as i32,
            1.0,
            &a,
            m as i32,
            &b,
            k as i32,
            0.0,
            &mut c2,
            n as i32
        );
        assert!((self::super::util::scomparea(&c1, &c2)).abs() < 1e-6);
    }
}
