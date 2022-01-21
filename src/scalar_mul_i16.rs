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
// allowed

#[inline]
fn scalar_mul_i16<const ROWS: usize, const COLS: usize>(
    scalar: i16,
    a: &[[i16; COLS]; ROWS],
    b: &mut [[i16; COLS]; ROWS],
) {
    for row in 0..ROWS {
        for col in 0..COLS {
            b[row][col] = scalar * a[row][col];
        }
    }
}

impl<const ROWS: usize, const COLS: usize> Mul<SMatrix<i16, ROWS, COLS>> for i16 {
    type Output = SMatrix<i16, ROWS, COLS>;
    #[inline]
    fn mul(self, rhs: SMatrix<i16, ROWS, COLS>) -> Self::Output {
        let mut b = MF::<i16, ROWS, COLS>::new_stack();
        scalar_mul_i16(self, rhs.array(), b.array_mut());
        b
    }
}

impl<const ROWS: usize, const COLS: usize> Mul<HMatrix<i16, ROWS, COLS>> for i16 {
    type Output = HMatrix<i16, ROWS, COLS>;
    #[inline]
    fn mul(self, rhs: HMatrix<i16, ROWS, COLS>) -> Self::Output {
        let mut b = MF::<i16, ROWS, COLS>::new_heap();
        scalar_mul_i16(self, rhs.array(), b.array_mut());
        b
    }
}

impl<const ROWS: usize, const COLS: usize> Mul<&SMatrix<i16, ROWS, COLS>> for i16 {
    type Output = SMatrix<i16, ROWS, COLS>;
    #[inline]
    fn mul(self, rhs: &SMatrix<i16, ROWS, COLS>) -> Self::Output {
        let mut b = MF::<i16, ROWS, COLS>::new_stack();
        scalar_mul_i16(self, rhs.array(), b.array_mut());
        b
    }
}

impl<const ROWS: usize, const COLS: usize> Mul<&HMatrix<i16, ROWS, COLS>> for i16 {
    type Output = HMatrix<i16, ROWS, COLS>;
    #[inline]
    fn mul(self, rhs: &HMatrix<i16, ROWS, COLS>) -> Self::Output {
        let mut b = MF::<i16, ROWS, COLS>::new_heap();
        scalar_mul_i16(self, rhs.array(), b.array_mut());
        b
    }
}

impl<const ROWS: usize, const COLS: usize> Mul<&mut SMatrix<i16, ROWS, COLS>> for i16 {
    type Output = SMatrix<i16, ROWS, COLS>;
    #[inline]
    fn mul(self, rhs: &mut SMatrix<i16, ROWS, COLS>) -> Self::Output {
        let mut b = MF::<i16, ROWS, COLS>::new_stack();
        scalar_mul_i16(self, rhs.array(), b.array_mut());
        b
    }
}

impl<const ROWS: usize, const COLS: usize> Mul<&mut HMatrix<i16, ROWS, COLS>> for i16 {
    type Output = HMatrix<i16, ROWS, COLS>;
    #[inline]
    fn mul(self, rhs: &mut HMatrix<i16, ROWS, COLS>) -> Self::Output {
        let mut b = MF::<i16, ROWS, COLS>::new_heap();
        scalar_mul_i16(self, rhs.array(), b.array_mut());
        b
    }
}

#[cfg(test)]
mod scalar_mul_tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut a = MF::<i16, 2, 2>::new_stack();
        a[0][0] = 2i16;
        a[1][1] = 4i16;
        let b = 2i16 * a;
        println!("{:?}", b);
        let mut a = MF::<i16, 2, 2>::new_heap();
        a[0][0] = 2i16;
        a[1][1] = 4i16;
        let b = 2i16 * a;
        println!("{:?}", b);
    }

    #[test]
    fn test_2() {
        let mut a = &mut MF::<i16, 2, 2>::new_stack();
        a[0][0] = 2i16;
        a[1][1] = 4i16;
        let b = 2i16 * a;
        println!("{:?}", b);
        let mut a = &mut MF::<i16, 2, 2>::new_heap();
        a[0][0] = 2i16;
        a[1][1] = 4i16;
        let b = 2i16 * a;
        println!("{:?}", b);
    }

    #[test]
    fn test_3() {
        let mut a = MF::<i16, 2, 2>::new_stack();
        a[0][0] = 2i16;
        a[1][1] = 4i16;
        let b = &a;
        let c = 2i16 * b;
        println!("{:?}", c);
        let mut a = MF::<i16, 2, 2>::new_heap();
        a[0][0] = 2i16;
        a[1][1] = 4i16;
        let b = &a;
        let c = 2i16 * b;
        println!("{:?}", c);
    }
}