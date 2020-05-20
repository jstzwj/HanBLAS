use crate::{HanInt, c32, c64};

pub fn sgbmv(
    trans: u8, 
    m: HanInt, 
    n: HanInt, 
    kl: HanInt, 
    ku: HanInt, 
    alpha: f32, 
    a: &[f32], 
    lda: HanInt, 
    x: &[f32], 
    incx: HanInt, 
    beta: f32, 
    y: &mut [f32], 
    incy: HanInt
) {

}

pub fn sgbmv_always_correct(
    trans: u8, 
    m: HanInt, 
    n: HanInt, 
    kl: HanInt, 
    ku: HanInt, 
    alpha: f32, 
    a: &[f32], 
    lda: HanInt, 
    x: &[f32], 
    incx: HanInt, 
    beta: f32, 
    y: &mut [f32], 
    incy: HanInt
) {
    let mut info = 0;
    if !lsame(trans,'N') && !lsame(trans,'T') && !lsame(trans,'C')) {
        info = 1;
    } else if m < 0 {
        info = 2;
    }
    else if n < 0 {
        info = 3;
    } else if kl < 0 {
        info = 4;
    } else if ku < 0 {
        info = 5;
    } else if lda < (kl+ku+1) {
        info = 8;
    } else if incx == 0 {
        info = 10;
    } else if incy == 0 {
        info = 13
    }
    if info != 0 {
        CALL xerbla('SGBMV ',info)
        RETURN
    }
    if ((m == 0)  ||  (n == 0)  || 
    +    ((alpha == zero)&& (beta == one))) RETURN
    if (lsame(trans,'N')) {
        lenx = n
        leny = m
    else
        lenx = m
        leny = n
    }
    if (incx>0) {
        kx = 1;
    } else {
        kx = 1 - (lenx-1)*incx
    }
    if (incy>0) {
        ky = 1
    } else {
        ky = 1 - (leny-1)*incy
    }
    if (beta != one) {
        if (incy == 1) {
            if (beta == zero) {
                DO i = 1,leny
                    y(i) = zero
                CONTINUE
            } else {
                DO 20 i = 1,leny
                    y(i) = beta*y(i)
                CONTINUE
            }
        else
            iy = ky
            if (beta == zero) {
                DO 30 i = 1,leny
                    y(iy) = zero
                    iy = iy + incy
                CONTINUE
            } else {
                DO i = 1,leny
                    y(iy) = beta*y(iy)
                    iy = iy + incy
                CONTINUE
            }
        }
    }
    if (alpha == zero) RETURN
    kup1 = ku + 1
    if (lsame(trans,'N')) {
        jx = kx
        if (incy == 1) {
            DO 60 j = 1,n
                temp = alpha*x(jx)
                k = kup1 - j
                DO 50 i = max(1,j-ku),min(m,j+kl)
                    y(i) = y(i) + temp*a(k+i,j)
                CONTINUE
                jx = jx + incx
            CONTINUE
        } else {
            DO 80 j = 1,n
                temp = alpha*x(jx)
                iy = ky
                k = kup1 - j
                DO 70 i = max(1,j-ku),min(m,j+kl)
                    y(iy) = y(iy) + temp*a(k+i,j)
                    iy = iy + incy
70             CONTINUE
                jx = jx + incx
                if (j>ku) ky = ky + incy
80         CONTINUE
        }
    else
        jy = ky
        if (incx == 1) {
            DO 100 j = 1,n
                temp = zero
                k = kup1 - j
                DO 90 i = max(1,j-ku),min(m,j+kl)
                    temp = temp + a(k+i,j)*x(i)
                CONTINUE
                y(jy) = y(jy) + alpha*temp
                jy = jy + incy
            CONTINUE
        } else {
            DO 120 j = 1,n
                temp = zero
                ix = kx
                k = kup1 - j
                DO 110 i = max(1,j-ku),min(m,j+kl)
                    temp = temp + a(k+i,j)*x(ix)
                    ix = ix + incx
                CONTINUE
                y(jy) = y(jy) + alpha*temp
                jy = jy + incy
                if (j>ku) kx = kx + incx
            CONTINUE
        }
    }
    RETURN
}


pub fn dgbmv(
    trans: u8, 
    m: HanInt, 
    n: HanInt, 
    kl: HanInt, 
    ku: HanInt, 
    alpha: f64, 
    a: &[f64], 
    lda: HanInt, 
    x: &[f64], 
    incx: HanInt, 
    beta: f64, 
    y: &mut [f64], 
    incy: HanInt
) {

}

pub fn cgbmv(
    trans: u8, 
    m: HanInt, 
    n: HanInt, 
    kl: HanInt, 
    ku: HanInt, 
    alpha: c32, 
    a: &[c32], 
    lda: HanInt, 
    x: &[c32], 
    incx: HanInt, 
    beta: c32, 
    y: &mut [c32], 
    incy: HanInt
) {

}

pub fn zgbmv(
    trans: u8, 
    m: HanInt, 
    n: HanInt, 
    kl: HanInt, 
    ku: HanInt, 
    alpha: c64, 
    a: &[c64], 
    lda: HanInt, 
    x: &[c64], 
    incx: HanInt, 
    beta: c64, 
    y: &mut [c64], 
    incy: HanInt
) {

}