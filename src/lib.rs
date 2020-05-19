#![feature(asm)]
#![feature(ptr_offset_from)]

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


pub const TEST_EPSILON: f64 = 0.01;
