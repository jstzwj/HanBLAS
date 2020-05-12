
pub mod util;
pub mod asum;
pub mod rotg;
pub mod cblas;

type c32 = num_complex::Complex<f32>;
type c64 = num_complex::Complex<f64>;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
