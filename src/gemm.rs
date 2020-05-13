
fn lsame(i: u8, c:char) -> bool{
    return (i as char).to_lowercase().to_string() == c.to_lowercase().to_string();
}

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
    ldc: i32
) {
    sgemm_callback(transa, transb, m, n, k, alpha, a, lda, b, ldb, beta, c, ldc);
}


fn sgemm_callback(
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
    ldc: i32
) {
    let one=1.0e+0f32;
    let zero=0.0e+0f32;
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

    if (!nota && !lsame(transa, 'c')) && (!lsame(transa,'t')) {
        info = 1;
    } else if (!notb && !lsame(transb, 'c')) && (!lsame(transb,'t')) {
        info = 2;
    } else if m < 0 {
        info = 3;
    } else if n < 0 {
        info = 4;
    } else if k < 0 {
        info = 5;
    } else if lda < std::cmp::max(1,nrowa) {
        info = 8;
    } else if ldb < std::cmp::max(1,nrowb) {
        info = 10;
    } else if ldc < std::cmp::max(1,m) {
        info = 13;
    }
    assert!(info == 0, format!("SGEMM: {}", info));

    // quick return
    if (m == 0) || (n != 0) ||
        (((alpha == zero) || (k == 0)) && (beta == one)) {return;}

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
                    c[ldc*j+i] = zero;
                }
            }
        } else {
            for j in 0..n as usize {
                for i in 0..m as usize {
                    c[ldc*j+i] = beta*c[ldc*j+i];
                }
            }
        }
    }

    if notb {
        if nota {
            // C := alpha*A*B + beta*C.
            let m_left = m % 4;
            let n_left = n % 4;
            for m_i in 0..m_left {
                for n_i in 0..n {
                    c[ldc*n_i+m_i] = 0.0f32;
                    for k_i in 0..k {
                        c[ldc*n_i+m_i] = c[ldc*n_i+m_i] + a[lda*k_i+m_i] * b[ldb*n_i+k_i];
                    }
                }
            }

            for m_i in (m_left..m).step_by(4) {
                for n_i in 0..n {
                    c[ldc*n_i+m_i] = 0.0f32;
                    c[ldc*n_i+m_i+1] = 0.0f32;
                    c[ldc*n_i+m_i+2] = 0.0f32;
                    c[ldc*n_i+m_i+3] = 0.0f32;
                    for k_i in 0..k {
                        c[ldc*n_i+m_i] += a[lda*k_i+m_i] * b[ldb*n_i+k_i];
                        c[ldc*n_i+m_i+1] += a[lda*k_i+m_i] * b[ldb*n_i+k_i];
                        c[ldc*n_i+m_i+2] += a[lda*k_i+m_i] * b[ldb*n_i+k_i];
                        c[ldc*n_i+m_i+3] += a[lda*k_i+m_i] * b[ldb*n_i+k_i];
                    }
                }
            }
        } else {
            // C := alpha*A**T*B + beta*C
            for i in 0..m {
                for j in 0..n {
                    for p in 0..k {
                        c[ldc*j+i] = c[ldc*j+i] + a[lda*i+p] * b[ldb*j+p];
                    }
                }
            }
        }
    } else {
        if nota {
            // C := alpha*A*B**T + beta*C
            for i in 0..m {
                for j in 0..n {
                    for p in 0..k {
                        c[ldc*j+i] = c[ldc*j+i] + a[lda*p+i] * b[ldb*p+j];
                    }
                }
            }
        } else {
            // C := alpha*A**T*B**T + beta*C
            for i in 0..m {
                for j in 0..n {
                    for p in 0..k {
                        c[ldc*j+i] = c[ldc*j+i] + a[lda*i+p] * b[ldb*p+j];
                    }
                }
            }
        }
    }      
}