// Copyright 2022 Stefan Zobel
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! Scalar multiplication

use crate::matrix::*;
use std::ops::Mul;

// Unfortunately, we have to implement scalar multiplication where the
// RHS is a scalar from std manually for each scalar type we want to
// support. See
// https://rust-lang.github.io/rfcs/2451-re-rebalancing-coherence.html
// and especially:
// "impl<T> ForeignTrait<LocalTypeCrateA> for T" is not allowed,
// because it might conflict with another crate writing
// "impl<T> ForeignTrait<T> for LocalTypeCrateB" which is always
// allowed. See also https://www.jstuber.net/2019/04/17/scalar-multiplication-in-rust/

#[inline]
fn scalar_mul_f32<const ROWS: usize, const COLS: usize>(
    scalar: f32,
    a: &[[f32; COLS]; ROWS],
    b: &mut [[f32; COLS]; ROWS],
) {
    for row in 0..ROWS {
        for col in 0..COLS {
            b[row][col] = scalar * a[row][col];
        }
    }
}

impl<const ROWS: usize, const COLS: usize> Mul<SMatrix<f32, ROWS, COLS>> for f32 {
    type Output = SMatrix<f32, ROWS, COLS>;
    #[inline]
    fn mul(self, rhs: SMatrix<f32, ROWS, COLS>) -> Self::Output {
        let mut b = MF::<f32, ROWS, COLS>::new_stack();
        scalar_mul_f32(self, rhs.array(), b.array_mut());
        b
    }
}

impl<const ROWS: usize, const COLS: usize> Mul<HMatrix<f32, ROWS, COLS>> for f32 {
    type Output = HMatrix<f32, ROWS, COLS>;
    #[inline]
    fn mul(self, rhs: HMatrix<f32, ROWS, COLS>) -> Self::Output {
        let mut b = MF::<f32, ROWS, COLS>::new_heap();
        scalar_mul_f32(self, rhs.array(), b.array_mut());
        b
    }
}

impl<const ROWS: usize, const COLS: usize> Mul<&SMatrix<f32, ROWS, COLS>> for f32 {
    type Output = SMatrix<f32, ROWS, COLS>;
    #[inline]
    fn mul(self, rhs: &SMatrix<f32, ROWS, COLS>) -> Self::Output {
        let mut b = MF::<f32, ROWS, COLS>::new_stack();
        scalar_mul_f32(self, rhs.array(), b.array_mut());
        b
    }
}

impl<const ROWS: usize, const COLS: usize> Mul<&HMatrix<f32, ROWS, COLS>> for f32 {
    type Output = HMatrix<f32, ROWS, COLS>;
    #[inline]
    fn mul(self, rhs: &HMatrix<f32, ROWS, COLS>) -> Self::Output {
        let mut b = MF::<f32, ROWS, COLS>::new_heap();
        scalar_mul_f32(self, rhs.array(), b.array_mut());
        b
    }
}

impl<const ROWS: usize, const COLS: usize> Mul<&mut SMatrix<f32, ROWS, COLS>> for f32 {
    type Output = SMatrix<f32, ROWS, COLS>;
    #[inline]
    fn mul(self, rhs: &mut SMatrix<f32, ROWS, COLS>) -> Self::Output {
        let mut b = MF::<f32, ROWS, COLS>::new_stack();
        scalar_mul_f32(self, rhs.array(), b.array_mut());
        b
    }
}

impl<const ROWS: usize, const COLS: usize> Mul<&mut HMatrix<f32, ROWS, COLS>> for f32 {
    type Output = HMatrix<f32, ROWS, COLS>;
    #[inline]
    fn mul(self, rhs: &mut HMatrix<f32, ROWS, COLS>) -> Self::Output {
        let mut b = MF::<f32, ROWS, COLS>::new_heap();
        scalar_mul_f32(self, rhs.array(), b.array_mut());
        b
    }
}

#[cfg(test)]
mod scalar_mul_tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut a = MF::<f32, 2, 2>::new_stack();
        a[0][0] = 2.0;
        a[1][1] = 4.0;
        let b = 2.0f32 * a;
        println!("{:?}", b);
        let mut a = MF::<f32, 2, 2>::new_heap();
        a[0][0] = 2.0;
        a[1][1] = 4.0;
        let b = 2.0f32 * a;
        println!("{:?}", b);
    }

    #[test]
    fn test_2() {
        let mut a = &mut MF::<f32, 2, 2>::new_stack();
        a[0][0] = 2.0;
        a[1][1] = 4.0;
        let b = 2.0f32 * a;
        println!("{:?}", b);
        let mut a = &mut MF::<f32, 2, 2>::new_heap();
        a[0][0] = 2.0;
        a[1][1] = 4.0;
        let b = 2.0f32 * a;
        println!("{:?}", b);
    }

    #[test]
    fn test_3() {
        let mut a = MF::<f32, 2, 2>::new_stack();
        a[0][0] = 2.0;
        a[1][1] = 4.0;
        let b = &a;
        let c = 2.0f32 * b;
        println!("{:?}", c);
        let mut a = MF::<f32, 2, 2>::new_heap();
        a[0][0] = 2.0;
        a[1][1] = 4.0;
        let b = &a;
        let c = 2.0f32 * b;
        println!("{:?}", c);
    }
}
