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
fn scalar_mul_i8<const ROWS: usize, const COLS: usize>(
    scalar: i8,
    a: &[[i8; COLS]; ROWS],
    b: &mut [[i8; COLS]; ROWS],
) {
    for row in 0..ROWS {
        for col in 0..COLS {
            b[row][col] = scalar * a[row][col];
        }
    }
}

impl<const ROWS: usize, const COLS: usize> Mul<SMatrix<i8, ROWS, COLS>> for i8 {
    type Output = SMatrix<i8, ROWS, COLS>;
    #[inline]
    fn mul(self, rhs: SMatrix<i8, ROWS, COLS>) -> Self::Output {
        let mut b = MF::<i8, ROWS, COLS>::new_stack();
        scalar_mul_i8(self, rhs.array(), b.array_mut());
        b
    }
}

impl<const ROWS: usize, const COLS: usize> Mul<HMatrix<i8, ROWS, COLS>> for i8 {
    type Output = HMatrix<i8, ROWS, COLS>;
    #[inline]
    fn mul(self, rhs: HMatrix<i8, ROWS, COLS>) -> Self::Output {
        let mut b = MF::<i8, ROWS, COLS>::new_heap();
        scalar_mul_i8(self, rhs.array(), b.array_mut());
        b
    }
}

impl<const ROWS: usize, const COLS: usize> Mul<&SMatrix<i8, ROWS, COLS>> for i8 {
    type Output = SMatrix<i8, ROWS, COLS>;
    #[inline]
    fn mul(self, rhs: &SMatrix<i8, ROWS, COLS>) -> Self::Output {
        let mut b = MF::<i8, ROWS, COLS>::new_stack();
        scalar_mul_i8(self, rhs.array(), b.array_mut());
        b
    }
}

impl<const ROWS: usize, const COLS: usize> Mul<&HMatrix<i8, ROWS, COLS>> for i8 {
    type Output = HMatrix<i8, ROWS, COLS>;
    #[inline]
    fn mul(self, rhs: &HMatrix<i8, ROWS, COLS>) -> Self::Output {
        let mut b = MF::<i8, ROWS, COLS>::new_heap();
        scalar_mul_i8(self, rhs.array(), b.array_mut());
        b
    }
}

impl<const ROWS: usize, const COLS: usize> Mul<&mut SMatrix<i8, ROWS, COLS>> for i8 {
    type Output = SMatrix<i8, ROWS, COLS>;
    #[inline]
    fn mul(self, rhs: &mut SMatrix<i8, ROWS, COLS>) -> Self::Output {
        let mut b = MF::<i8, ROWS, COLS>::new_stack();
        scalar_mul_i8(self, rhs.array(), b.array_mut());
        b
    }
}

impl<const ROWS: usize, const COLS: usize> Mul<&mut HMatrix<i8, ROWS, COLS>> for i8 {
    type Output = HMatrix<i8, ROWS, COLS>;
    #[inline]
    fn mul(self, rhs: &mut HMatrix<i8, ROWS, COLS>) -> Self::Output {
        let mut b = MF::<i8, ROWS, COLS>::new_heap();
        scalar_mul_i8(self, rhs.array(), b.array_mut());
        b
    }
}

#[cfg(test)]
mod scalar_mul_tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut a = MF::<i8, 2, 2>::new_stack();
        a[0][0] = 2i8;
        a[1][1] = 4i8;
        let b = 2i8 * a;
        println!("{:?}", b);
        let mut a = MF::<i8, 2, 2>::new_heap();
        a[0][0] = 2i8;
        a[1][1] = 4i8;
        let b = 2i8 * a;
        println!("{:?}", b);
    }

    #[test]
    fn test_2() {
        let mut a = &mut MF::<i8, 2, 2>::new_stack();
        a[0][0] = 2i8;
        a[1][1] = 4i8;
        let b = 2i8 * a;
        println!("{:?}", b);
        let mut a = &mut MF::<i8, 2, 2>::new_heap();
        a[0][0] = 2i8;
        a[1][1] = 4i8;
        let b = 2i8 * a;
        println!("{:?}", b);
    }

    #[test]
    fn test_3() {
        let mut a = MF::<i8, 2, 2>::new_stack();
        a[0][0] = 2i8;
        a[1][1] = 4i8;
        let b = &a;
        let c = 2i8 * b;
        println!("{:?}", c);
        let mut a = MF::<i8, 2, 2>::new_heap();
        a[0][0] = 2i8;
        a[1][1] = 4i8;
        let b = &a;
        let c = 2i8 * b;
        println!("{:?}", c);
    }
}
