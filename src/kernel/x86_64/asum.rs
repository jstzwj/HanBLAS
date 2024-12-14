#[cfg(feature = "thread")]
use rayon::prelude::*;

// #[cfg(target_arch = "x86_64")]
// use std::arch::x86_64::*;

use std::arch::asm;

use crate::HanInt;

#[cfg(any(target_feature = "avx"))]
#[target_feature(enable = "avx")]
pub unsafe fn sasum_x86_64_avx(n: HanInt, x: *const f32, incx: HanInt) -> f32 {
    let mut ret = 0.0e0f32;
    if n <= 0 || incx <= 0 {
        return ret;
    }

    if incx == 1 {
        let mut px = x;
        let offset = px.align_offset(8 * std::mem::align_of::<f32>());
        let px_left_align = px.add(offset);
        let px_end = x.offset(n as isize);
        let m = (n - offset as HanInt) % 32;
        let px_right_align = px_end.sub(m as usize);
        if px_left_align >= px_end {
            while px < px_end {
                ret = ret + (*px).abs();
                px = px.offset(1);
            }
            return ret;
        } else {
            while px < px_left_align {
                ret = ret + (*px).abs();
                px = px.offset(1);
            }
        }
        

        let mut temp_array: [f32; 8] = [0.0f32; 8];
        llvm_asm!("
            vpcmpeqb %ymm0, %ymm0, %ymm0
            vpsrld $$1, %ymm0, %ymm0

            vxorps %ymm1, %ymm1, %ymm1
            vxorps %ymm2, %ymm2, %ymm2
            vxorps %ymm3, %ymm3, %ymm3
            vxorps %ymm4, %ymm4, %ymm4

            movq $1, %rax

            cmp %rax, $2
            jle sasum_x86_64_avx_inc1_sum_all
            sasum_x86_64_avx_inc1_loop:
            vmovaps (%rax), %ymm5
            vmovaps 32(%rax), %ymm6
            vmovaps 64(%rax), %ymm7
            vmovaps 96(%rax), %ymm8

            vandps %ymm0, %ymm5, %ymm5
            vaddps %ymm5, %ymm1, %ymm1

            vandps %ymm0, %ymm6, %ymm6
            vaddps %ymm6, %ymm2, %ymm2
            
            vandps %ymm0, %ymm7, %ymm7
            vaddps %ymm7, %ymm3, %ymm3

            vandps %ymm0, %ymm8, %ymm8
            vaddps %ymm8, %ymm4, %ymm4

            addq $$128, %rax
            cmp %rax, $2
            ja sasum_x86_64_avx_inc1_loop
            
            sasum_x86_64_avx_inc1_sum_all:
            vaddps %ymm2, %ymm1, %ymm1
            vaddps %ymm4, %ymm3, %ymm3
            vaddps %ymm3, %ymm1, %ymm1
            vmovups %ymm1, $0

            movq %rax, $1
            "
            : "=*m"(temp_array.as_mut_ptr()), "=r"(px)
            : "m"(px_right_align), "1"(px)
            : "rax", "ymm0", "ymm1", "ymm2", "ymm3", "ymm4", "ymm5", "ymm6", "ymm7", "ymm8"
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

        let mut sum: f32 = 0.0f32;
        llvm_asm!("
            pcmpeqb %xmm15, %xmm15
            psrld $$1, %xmm15

            xorps %xmm0, %xmm0
            xorps %xmm1, %xmm1
            xorps %xmm2, %xmm2
            xorps %xmm3, %xmm3

            movq $1, %rax

            cmp %rax, $2
            jle sasum_x86_64_avx_incx_sum_all
            sasum_x86_64_avx_incx_loop:

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
            ja sasum_x86_64_avx_incx_loop
            
            sasum_x86_64_avx_incx_sum_all:
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

#[cfg(all(any(target_feature = "avx"), feature = "thread"))]
#[target_feature(enable = "avx")]
pub unsafe fn sasum_x86_64_mt_avx(n: HanInt, sx: &[f32], incx: HanInt) -> f32 {
    // small size disable mt
    if 1 + (n - 1) * incx.abs() < 500000 {
        return sasum_x86_64_avx(n, sx, incx);
    }

    let mut result = 0.0e0f32;

    if n <= 0 || incx <= 0 {
        return result;
    }

    if incx == 1 {
        let chunk_size = 64 * 1024 / 4;
        let m = n % chunk_size;

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
        let mut temp_array = [0.0f32; 8];
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
        if px_left_align >= px_end {
            while px < px_end {
                ret = ret + (*px).abs();
                px = px.offset(1);
            }
            return ret;
        } else {
            while px < px_left_align {
                ret = ret + (*px).abs();
                px = px.offset(1);
            }
        }
        

        let mut temp_array: [f32; 4] = [0.0f32; 4];
        asm!(
            "pcmpeqb xmm15, xmm15",
            "psrld xmm15, 0x1",

            "xorps xmm0, xmm0",
            "xorps xmm1, xmm1",
            "xorps xmm2, xmm2",
            "xorps xmm3, xmm3",

            "movq rax, {px}",

            "cmp rax, {px_right_align}",
            "jle 3f",
            "2:",
            "movaps [rax], xmm4",
            "movaps [rax + 16], xmm5",
            "movaps [rax + 32], xmm6",
            "movaps [rax + 48], xmm7",

            "andps xmm15, xmm4",
            "addps xmm4, xmm0",

            "andps xmm15, xmm5",
            "addps xmm5, xmm1",

            "andps xmm15, xmm6",
            "addps xmm6, xmm2",
            
            "andps xmm15, xmm7",
            "addps xmm7, xmm3",

            "addq 64, rax",
            "cmp rax, {px_right_align}",
            "ja 2b",
            
            "3:",
            "addps xmm1, xmm0",
            "addps xmm3, xmm2",
            "addps xmm2, xmm0",
            "movups xmm0, {temp_array}",

            "movq rax, {px}",
            temp_array = in(reg) &mut temp_array,
            px = inout(reg) px,
            px_right_align = in(reg) px_right_align,
        );
        // Output: "=*m"(temp_array.as_mut_ptr()), "=r"(px)
        // Input: "m"(px_right_align), "1"(px)
        // : "rax", "xmm0", "xmm1", "xmm2", "xmm3", "xmm4", "xmm5", "xmm6", "xmm7", "xmm15"

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

        let step_bytes: usize = incx as usize * std::mem::size_of::<f32>();
        let mut sum: f32 = 0.0f32;
        asm!(
            "pcmpeqb xmm15, xmm15",
            "psrld 0x1, xmm15",

            "xorps xmm0, xmm0",
            "xorps xmm1, xmm1",
            "xorps xmm2, xmm2",
            "xorps xmm3, xmm3",

            "movq {px}, rax",

            "cmp rax, {px_end}",
            "jle 3f",
            "2:",

            // load data from memory to xmm4-7, then abs and sum.
            "movss [rax], xmm4",
            "addq {step_bytes}, rax",
            "andps xmm15, xmm4",
            "addss xmm4, xmm0",

            "movss [rax], xmm5",
            "addq {step_bytes}, rax",
            "andps xmm15, xmm5",
            "addss xmm5, xmm1",

            "movss [rax], xmm6",
            "addq {step_bytes}, rax",
            "andps xmm15, xmm6",
            "addss xmm6, xmm2",

            "movss [rax], xmm7",
            "addq {step_bytes}, rax",
            "andps xmm15, xmm7",
            "addss xmm7, xmm3",

            "movss [rax], xmm4",
            "addq {step_bytes}, rax",
            "andps xmm15, xmm4",
            "addss xmm4, xmm0",

            "movss [rax], xmm5",
            "addq {step_bytes}, rax",
            "andps xmm15, xmm5",
            "addss xmm5, xmm1",

            "movss [rax], xmm6",
            "addq {step_bytes}, rax",
            "andps xmm15, xmm6",
            "addss xmm6, xmm2",

            "movss [rax], xmm7",
            "addq {step_bytes}, rax",
            "andps xmm15, xmm7",
            "addss xmm7, xmm3",
            
            "cmp rax, {px_end}",
            "ja 2b",
            
            "3:",
            "addss xmm1, xmm0",
            "addss xmm3, xmm2",
            "addss xmm2, xmm0",
            "movss xmm0, {sum}",

            "movq rax, {px}",
            sum = in(reg) &mut sum,
            px = inout(reg) px,
            step_bytes = in(reg) step_bytes,
            px_end = in(reg) px_end,
        );
        // Output: "=*m"(&mut sum as *mut f32), "=r"(px)
        // Input: "m"(px_end), "r"(incx as usize * std::mem::size_of::<f32>()), "1"(px)
        // : "rax", "xmm0", "xmm1", "xmm2", "xmm3", "xmm4",
        //     "xmm5", "xmm6", "xmm7", "xmm15"
        // println!("{:?}", sum);
        ret = ret + sum;
    }
    return ret;
}

#[cfg(all(target_feature = "sse", feature = "thread"))]
#[target_feature(enable = "sse")]
pub unsafe fn sasum_x86_64_mt_sse(n: HanInt, sx: &[f32], incx: HanInt) -> f32 {
    // small size disable mt
    if 1 + (n - 1) * incx.abs() < 100000 {
        return sasum_x86_64_sse(n, sx, incx);
    }

    let mut result = 0.0e0f32;

    if n <= 0 || incx <= 0 {
        return result;
    }

    if incx == 1 {
        let chunk_size = 64 * 1024 / 4;
        let m = n % chunk_size;

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
        let mut temp_array = [0.0f32; 4];
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
