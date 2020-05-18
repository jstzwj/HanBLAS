
use rayon::prelude::*;

#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

use crate::HanInt;

#[cfg(any(target_feature = "avx"))]
#[target_feature(enable = "avx")]
pub unsafe fn sasum_x86_64_avx(n: HanInt, sx: &[f32], incx: HanInt) -> f32 {
    let mut result = 0.0e0f32;

    if n <= 0 || incx <= 0 {
        return result;
    }
    if incx == 1 {
        let m = n%8;
        if m != 0 {
            for i in 0..m {
                result = result + sx[i as usize].abs();
            }
            if n < 8 {
                return result;
            }
        }
        let mut temp = std::arch::x86_64::_mm256_setzero_ps();

        for i in (m as usize..n as usize).step_by(8) {
            let sx_slice = std::arch::x86_64::_mm256_loadu_ps(sx.as_ptr().add(i));
            temp = std::arch::x86_64::_mm256_add_ps(temp, sx_slice);
        }
        
        // store and cum
        let mut temp_array = [0.0f32;8];
        std::arch::x86_64::_mm256_storeu_ps(temp_array.as_mut_ptr(), temp);
        for i in temp_array.iter() {
            result += i;
        }
    } else {
        // fixme: incx是负的
        for sxi in sx.iter().step_by(incx as usize) {
            result = result + sxi.abs();
        }
    }
    return result;
}


#[cfg(all(any(target_feature = "avx"), feature = "thread"))]
#[target_feature(enable = "avx")]
pub unsafe fn sasum_x86_64_mt_avx(n: HanInt, sx: &[f32], incx: HanInt) -> f32 {
    // small size disable mt
    if 1 + (n-1)*incx.abs() < 500000 {
        return sasum_x86_64_avx(n, sx, incx);
    }

    let mut result = 0.0e0f32;

    if n <= 0 || incx <= 0 {
        return result;
    }

    if incx == 1 {
        let chunk_size = 64 * 1024 / 4;
        let m = n%chunk_size;
        
        if m != 0 {
            for i in 0..m {
                result = result + sx[i as usize].abs();
            }
            if n < chunk_size {
                return result;
            }
        }
        
        let temps: Vec<_> = sx[m as usize..]
            .par_chunks(chunk_size as usize)
            .map(|chunk| {
                let mut temp = std::arch::x86_64::_mm256_setzero_ps();
                for i in (0..chunk.len()).step_by(8) {
                    let sx_slice = std::arch::x86_64::_mm256_loadu_ps(chunk.as_ptr().add(i));
                    temp = std::arch::x86_64::_mm256_add_ps(temp, sx_slice);
                }
                temp
            })
            .collect();

        // store and cum
        let mut temp_array = [0.0f32;8];
        for each_temp in temps.iter() {
            std::arch::x86_64::_mm256_storeu_ps(temp_array.as_mut_ptr(), *each_temp);
            for i in temp_array.iter() {
                result += i;
            }
        }
    } else {
        for sxi in sx.iter().step_by(incx as usize) {
            result = result + sxi.abs();
        }
    }
    return result;
}


#[cfg(target_feature = "sse")]
#[target_feature(enable = "sse")]
pub unsafe fn sasum_x86_64_sse(n: HanInt, x: *const f32, incx: HanInt) -> f32 {
    let mut ret = 0.0e0f32;
    if n <= 0 || incx <= 0 {
        return ret;
    }

    if incx == 1 {
        let mut px = x;
        let offset = px.align_offset(4 * std::mem::align_of::<f32>());
        let px_left_align = px.add(offset);
        let px_end = x.offset(n as isize);
        let m = (n - offset as HanInt) % 16;
        let px_right_align = px_end.sub(m as usize);

        while px < px_left_align {
            if px >= px_end {
                return ret;
            }
            ret = ret + (*px).abs();
            px = px.offset(1);
        }

        let mut temp_array:[f32;4] = [0.0f32;4];
        asm!("
            pcmpeqb %xmm15, %xmm15
            psrld $$1, %xmm15

            xorps %xmm0, %xmm0
            xorps %xmm1, %xmm1
            xorps %xmm2, %xmm2
            xorps %xmm3, %xmm3

            movq $1, %rax

            cmp %rax, $2
            jle sasum_x86_64_sse_inc1_sum_all
            sasum_x86_64_sse_inc1_loop:
            movaps (%rax), %xmm4
            movaps 16(%rax), %xmm5
            movaps 32(%rax), %xmm6
            movaps 48(%rax), %xmm7

            andps %xmm15, %xmm4
            addps %xmm4, %xmm0

            andps %xmm15, %xmm5
            addps %xmm5, %xmm1

            andps %xmm15, %xmm6
            addps %xmm6, %xmm2
            
            andps %xmm15, %xmm7
            addps %xmm7, %xmm3

            addq $$64, %rax
            cmp %rax, $2
            ja sasum_x86_64_sse_inc1_loop
            
            sasum_x86_64_sse_inc1_sum_all:
            addps %xmm1, %xmm0
            addps %xmm3, %xmm2
            addps %xmm2, %xmm0
            movups %xmm0, $0

            movq %rax, $1
            "
            : "=*m"(temp_array.as_mut_ptr()), "=r"(px)
            : "m"(px_right_align), "1"(px)
            : "rax", "xmm0", "xmm1", "xmm2", "xmm3", "xmm4", "xmm5", "xmm6", "xmm7", "xmm15"
        );

        // println!("{:?}", temp_array);

        // store and cum
        for value in temp_array.iter() {
            ret += value;
        }

        // left nums
        while px < px_end {
            ret = ret + (*px).abs();
            px = px.offset(1);
        }
    } else {
        let mut px = x;
        let px_end = x.offset((n * incx) as isize);
        let m = n % 8;
        let px_left_unroll = px.add((m * incx) as usize);

        while px < px_left_unroll {
            if px >= px_end {
                return ret;
            }
            ret = ret + (*px).abs();
            px = px.offset(incx as isize);
        }

        let mut sum:f32 = 0.0f32;
        asm!("
            pcmpeqb %xmm15, %xmm15
            psrld $$1, %xmm15

            xorps %xmm0, %xmm0
            xorps %xmm1, %xmm1
            xorps %xmm2, %xmm2
            xorps %xmm3, %xmm3

            movq $1, %rax

            cmp %rax, $2
            jle sasum_x86_64_sse_incx_sum_all
            sasum_x86_64_sse_incx_loop:

            # load data from memory to xmm4-7, then abs and sum.
            movss (%rax), %xmm4
            addq $3, %rax
            andps %xmm15, %xmm4
            addss %xmm4, %xmm0

            movss (%rax), %xmm5
            addq $3, %rax
            andps %xmm15, %xmm5
            addss %xmm5, %xmm1

            movss (%rax), %xmm6
            addq $3, %rax
            andps %xmm15, %xmm6
            addss %xmm6, %xmm2

            movss (%rax), %xmm7
            addq $3, %rax
            andps %xmm15, %xmm7
            addss %xmm7, %xmm3

            movss (%rax), %xmm4
            addq $3, %rax
            andps %xmm15, %xmm4
            addss %xmm4, %xmm0

            movss (%rax), %xmm5
            addq $3, %rax
            andps %xmm15, %xmm5
            addss %xmm5, %xmm1

            movss (%rax), %xmm6
            addq $3, %rax
            andps %xmm15, %xmm6
            addss %xmm6, %xmm2

            movss (%rax), %xmm7
            addq $3, %rax
            andps %xmm15, %xmm7
            addss %xmm7, %xmm3
            
            cmp %rax, $2
            ja sasum_x86_64_sse_incx_loop
            
            sasum_x86_64_sse_incx_sum_all:
            addss %xmm1, %xmm0
            addss %xmm3, %xmm2
            addss %xmm2, %xmm0
            movss %xmm0, $0

            movq %rax, $1
            "
            : "=*m"(&mut sum as *mut f32), "=r"(px)
            : "m"(px_end), "r"(incx as usize * std::mem::size_of::<f32>()), "1"(px)
            : "rax", "xmm0", "xmm1", "xmm2", "xmm3", "xmm4",
                "xmm5", "xmm6", "xmm7", "xmm15"
        );
        // println!("{:?}", sum);
        ret = ret + sum;
    }
    return ret;
}


#[cfg(
    all(target_feature = "sse", feature = "thread")
)]
#[target_feature(enable = "sse")]
pub unsafe fn sasum_x86_64_mt_sse(n: HanInt, sx: &[f32], incx: HanInt) -> f32 {
    // small size disable mt
    if 1 + (n-1)*incx.abs() < 100000 {
        return sasum_x86_64_sse(n, sx, incx);
    }

    let mut result = 0.0e0f32;

    if n <= 0 || incx <= 0 {
        return result;
    }

    if incx == 1 {
        let chunk_size = 64 * 1024 / 4;
        let m = n%chunk_size;
        
        if m != 0 {
            for i in 0..m {
                result = result + sx[i as usize].abs();
            }
            if n < chunk_size {
                return result;
            }
        }
        
        let temps: Vec<_> = sx[m as usize..]
            .par_chunks(chunk_size as usize)
            .map(|chunk| {
                let mut temp = std::arch::x86_64::_mm_setzero_ps();
                for i in (0..chunk.len()).step_by(4) {
                    let sx_slice = std::arch::x86_64::_mm_loadu_ps(chunk.as_ptr().add(i));
                    temp = std::arch::x86_64::_mm_add_ps(temp, sx_slice);
                }
                temp
            })
            .collect();

        // store and cum
        let mut temp_array = [0.0f32;4];
        for each_temp in temps.iter() {
            std::arch::x86_64::_mm_storeu_ps(temp_array.as_mut_ptr(), *each_temp);
            for i in temp_array.iter() {
                result += i;
            }
        }
    } else {
        for sxi in sx.iter().step_by(incx as usize) {
            result = result + sxi.abs();
        }
    }
    return result;
}