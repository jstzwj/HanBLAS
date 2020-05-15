
use crate::HanInt;
use crate::asum;

type CBLAS_INDEX = usize;

enum CBLAS_ORDER {
    CblasRowMajor = 101,
    CblasColMajor = 102,
}

enum CBLAS_TRANSPOSE {
    CblasNoTrans = 111,
    CblasTrans = 112,
    CblasConjTrans = 113,
}

enum CBLAS_UPLO {
    CblasUpper = 121,
    CblasLower = 122,
}

enum CBLAS_DIAG {
    CblasNonUnit = 131,
    CblasUnit = 132,
}

enum CBLAS_SIDE {
    CblasLeft = 141,
    CblasRight = 142,
}

#[no_mangle]
pub extern "C" fn cblas_sasum(n: i32, sx: *const f32, incx: i32) -> f32 {
    unsafe {
        return asum::sasum(
            n as HanInt,
            std::slice::from_raw_parts(sx, (1 + (n-1)*incx.abs())as usize),
            incx as HanInt
        );
    }
}