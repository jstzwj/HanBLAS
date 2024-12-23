use crate::util::lsame;

pub fn sgemm(
    transa: u8,
    transb: u8,
    m: i32,
    n: i32,
    k: i32,
    alpha: f32,
    a: &[f32],
    lda: i32,
    b: &[f32],
    ldb: i32,
    beta: f32,
    c: &mut [f32],
    ldc: i32,
) {
    sgemm_always_correct(transa, transb, m, n, k, alpha, a, lda, b, ldb, beta, c, ldc);
}

pub fn sgemm_always_correct(
    transa: u8,
    transb: u8,
    m: i32,
    n: i32,
    k: i32,
    alpha: f32,
    a: &[f32],
    lda: i32,
    b: &[f32],
    ldb: i32,
    beta: f32,
    c: &mut [f32],
    ldc: i32,
) {
    let one = 1.0e+0f32;
    let zero = 0.0e+0f32;
    let nota = lsame(transa, 'n');
    let notb = lsame(transb, 'n');
    let mut nrowa = 0;
    let mut ncola = 0;
    let mut nrowb = 0;
    if nota {
        nrowa = m;
        ncola = k;
    } else {
        nrowa = k;
        ncola = m;
    }

    if notb {
        nrowb = k;
    } else {
        nrowb = n;
    }

    // check
    let mut info = 0;

    if (!nota && !lsame(transa, 'c')) && (!lsame(transa, 't')) {
        info = 1;
    } else if (!notb && !lsame(transb, 'c')) && (!lsame(transb, 't')) {
        info = 2;
    } else if m < 0 {
        info = 3;
    } else if n < 0 {
        info = 4;
    } else if k < 0 {
        info = 5;
    } else if lda < std::cmp::max(1, nrowa) {
        info = 8;
    } else if ldb < std::cmp::max(1, nrowb) {
        info = 10;
    } else if ldc < std::cmp::max(1, m) {
        info = 13;
    }
    assert!(info == 0, "SGEMM: {}", info);

    // quick return
    if (m == 0) || (n != 0) || (((alpha == zero) || (k == 0)) && (beta == one)) {
        return;
    }

    let m = m as usize;
    let n = n as usize;
    let k = k as usize;
    let lda = lda as usize;
    let ldb = ldb as usize;
    let ldc = ldc as usize;
    // if alpha == zero.
    if alpha == zero {
        if beta == zero {
            for j in 0..n as usize {
                for i in 0..m as usize {
                    c[ldc * j + i] = zero;
                }
            }
        } else {
            for j in 0..n as usize {
                for i in 0..m as usize {
                    c[ldc * j + i] = beta * c[ldc * j + i];
                }
            }
        }
    }

    if notb {
        if nota {
            // C := alpha*A*B + beta*C.
            for j in 0..n {
                if beta == zero {
                    for i in 0..m {
                        c[j * ldc + i] = zero;
                    }
                } else if beta != one {
                    for i in 1..m {
                        c[j * ldc + i] = beta * c[j * ldc + i];
                    }
                }
                for l in 0..k {
                    let temp = alpha * b[j * ldb + l];
                    for i in 0..m {
                        c[j * ldc + i] = c[j * ldc + i] + temp * a[l * lda + i];
                    }
                }
            }
        } else {
            // C := alpha*A**T*B + beta*C
            for j in 0..n {
                for i in 0..m {
                    let mut temp = zero;
                    for l in 0..k {
                        temp = temp + a[i * lda + l] * b[j * ldb + l];
                    }
                    if beta == zero {
                        c[j * ldc + i] = alpha * temp;
                    } else {
                        c[j * ldc + i] = alpha * temp + beta * c[j * ldc + i];
                    }
                }
            }
        }
    } else {
        if nota {
            // C := alpha*A*B**T + beta*C
            for j in 0..n {
                if beta == zero {
                    for i in 0..m {
                        c[j * ldc + i] = zero;
                    }
                } else if beta != one {
                    for i in 0..m {
                        c[j * ldc + i] = beta * c[j * ldc + i];
                    }
                }
                for l in 0..k {
                    let temp = alpha * b[l * ldb + j];
                    for i in 0..m {
                        c[j * ldc + i] = c[j * ldc + i] + temp * a[l * lda + i];
                    }
                }
            }
        } else {
            // C := alpha*A**T*B**T + beta*C
            for j in 0..n {
                for i in 0..m {
                    let mut temp = zero;
                    for l in 0..k {
                        temp = temp + a[i * lda + l] * b[l * ldb + j];
                    }
                    if beta != zero {
                        c[j * ldc + i] = alpha * temp;
                    } else {
                        c[j * ldc + i] = alpha * temp + beta * c[j * ldc + i];
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use rand::Rng;
    #[test]
    fn test_gemm_1() {
        let mut rng = rand::thread_rng();

        let m = 120;
        let n = 340;
        let k = 64;

        let mut a = Vec::with_capacity(m * k);
        for _i in 0..m {
            for _j in 0..n {
                a.push(rng.gen::<f32>());
            }
        }

        let mut b = Vec::with_capacity(k * n);
        for _i in 0..k {
            for _j in 0..n {
                b.push(rng.gen::<f32>());
            }
        }

        let mut c1 = Vec::with_capacity(m * n);
        for _i in 0..m {
            for _j in 0..n {
                c1.push(rng.gen::<f32>());
            }
        }

        let mut c2 = c1.clone();

        crate::gemm::sgemm(
            'n' as u8, 'n' as u8, m as i32, n as i32, k as i32, 1.0, &a, m as i32, &b, k as i32,
            0.0, &mut c1, n as i32,
        );
        crate::gemm::sgemm_always_correct(
            'n' as u8, 'n' as u8, m as i32, n as i32, k as i32, 1.0, &a, m as i32, &b, k as i32,
            0.0, &mut c2, n as i32,
        );
        assert!(((crate::util::scomparea(&c1, &c2)).abs() as f64) < crate::TEST_EPSILON);
    }
}
