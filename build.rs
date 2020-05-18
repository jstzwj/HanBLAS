fn main() {
    println!("cargo:rustc-link-lib=mkl_intel_lp64");
    println!("cargo:rustc-link-lib=mkl_core");
    println!("cargo:rustc-link-lib=mkl_sequential");
}