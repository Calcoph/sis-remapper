use std::arch::x86_64::__m512;
use std::arch::x86_64::_mm512_loadu_ps as loadu_f32;
use std::arch::x86_64::_mm512_load_ps as load_f32;
use std::arch::x86_64::_mm512_sub_ps as sub_f32;
use std::arch::x86_64::_mm512_add_ps as add_f32;
use std::arch::x86_64::_mm512_mask_add_ps as mask_add_f32;
use std::arch::x86_64::_mm512_mul_ps as mul_f32;
use std::arch::x86_64::_mm512_div_ps as div_f32;
use std::arch::x86_64::_mm512_store_ps as recover_f32;
use std::arch::x86_64::_mm512_rcp14_ps as reciprocal_f32;
use std::arch::x86_64::_mm512_sqrt_ps as sqrt_f32;
use std::arch::x86_64::_mm512_permute_ps as permute_f32;
use std::arch::x86_64::_mm512_setzero_ps as zero_f32;
use std::arch::x86_64::_mm512_unpacklo_ps as unpack_even_f32;
use std::arch::x86_64::_mm512_cmpnlt_ps_mask as cmp_ge_f32;

use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Sub;

#[repr(align(64))]
pub(crate) struct ConstM512([f32;16]);

impl ConstM512 {
    pub(crate) const fn single(v: f32) -> Self {
        ConstM512([
            v,v,v,v,
            v,v,v,v,
            v,v,v,v,
            v,v,v,v,
        ])
    }

    pub(crate) const fn repeat_2(a: f32, b: f32) -> Self {
        ConstM512([
            a,b,a,b,
            a,b,a,b,
            a,b,a,b,
            a,b,a,b,
        ])
    }

    pub(crate) fn as_ptr(&self) -> *const f32 {
        self.0.as_ptr()
    }
}

#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
pub(crate) struct f32x16(__m512);

impl f32x16 {
    pub(crate) fn load_aligned(addr: *const f32) -> Self {
        unsafe {
            f32x16(load_f32(addr))
        }
    }

    pub(crate) fn load_unaligned(addr: *const f32) -> Self {
        unsafe {
            f32x16(loadu_f32(addr))
        }
    }

    pub(crate) fn reciprocal(self) -> Self {
        unsafe {
            f32x16(reciprocal_f32(self.0))
        }
    }

    pub(crate) fn recover(self, dest: *mut f32){
        unsafe {
            recover_f32(dest, self.0);
        }
    }

    pub(crate) fn sqrt(self) -> Self {
        unsafe {
            f32x16(sqrt_f32(self.0))
        }
    }

    /// swaps pairs of f32
    ///
    /// 00 01 02 03
    ///
    /// 04 05 06 07
    ///
    /// 08 09 10 11
    ///
    /// 12 13 14 15
    ///
    /// becomes:
    ///
    /// 01 00 03 02
    ///
    /// 05 04 07 06
    ///
    /// 09 08 11 10
    ///
    /// 13 12 15 14
    pub(crate) fn swap2_same(self) -> Self {
        unsafe {
            f32x16(permute_f32::<0b10_11_00_01>(self.0))
        }
    }

    /// swaps pairs of f32
    ///
    /// 00 01 02 03
    ///
    /// 04 05 06 07
    ///
    /// 08 09 10 11
    ///
    /// 12 13 14 15
    ///
    /// becomes:
    ///
    /// 00 03 02 01
    ///
    /// 04 07 06 05
    ///
    /// 08 11 10 09
    ///
    /// 12 15 14 13
    pub(crate) fn swap2_right(self) -> Self {
        unsafe {
            f32x16(permute_f32::<0b01_10_11_00>(self.0))
        }
    }

    pub(crate) fn zero() -> Self {
        unsafe {
            f32x16(zero_f32())
        }
    }

    /// interleave 2 vectors (the even f64s)
    /// having 2 vectors:
    ///
    /// self:
    ///
    /// a0 a1 xx xx
    ///
    /// a2 a3 xx xx
    ///
    /// a4 a5 xx xx
    ///
    /// a6 a7 xx xx
    ///
    /// b:
    ///
    /// b0 b1 xx xx
    ///
    /// b2 b3 xx xx
    ///
    /// b4 b5 xx xx
    ///
    /// b6 b7 xx xx
    ///
    /// result:
    ///
    /// a0 b0 a1 b1
    ///
    /// a2 b2 a3 b3
    ///
    /// a4 b4 a5 b5
    ///
    /// a6 b6 a7 b7
    pub(crate) fn unpack_even(self, b: f32x16) -> Self {
        unsafe {
            f32x16(unpack_even_f32(self.0, b.0))
        }
    }

    /// Returns true if all passed the test
    ///
    /// if self >= b {
    ///     (counter, true)
    /// } else {
    ///     counter += sum;
    ///     (counter, false)
    /// }
    pub(crate) fn incr_if_ge(self, b: f32x16, counter: f32x16, sum: f32x16) -> (f32x16, bool) {
        unsafe {
            let result = cmp_ge_f32(self.0, b.0);
            if result == 0xff_ff {
                // all of self >= b
                (counter, true)
            } else {
                (counter.masked_sum(sum, result), false)
            }
        }
    }

    fn masked_sum(&self, sum: f32x16, mask: u16) -> f32x16 {
        unsafe {
            f32x16(mask_add_f32(self.0, mask, self.0, sum.0))
        }
    }
}

impl Add for f32x16 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        unsafe {
            f32x16(add_f32(self.0, rhs.0))
        }
    }
}

impl Sub for f32x16 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        unsafe {
            f32x16(sub_f32(self.0, rhs.0))
        }
    }
}

impl Mul for f32x16 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        unsafe {
            f32x16(mul_f32(self.0, rhs.0))
        }
    }
}

impl Div for f32x16 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        unsafe {
            f32x16(div_f32(self.0, rhs.0))
        }
    }
}
