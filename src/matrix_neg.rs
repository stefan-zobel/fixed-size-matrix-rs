//! Implementation of Neg

use crate::matrix::*;
use std::ops::Neg;

#[inline]
fn neg<T: Numeric<T>, const ROWS: usize, const COLS: usize>(
    a: &[[T; COLS]; ROWS],
    b: &mut [[T; COLS]; ROWS],
) {
    for row in 0..ROWS {
        for col in 0..COLS {
            b[row][col] = -a[row][col];
        }
    }
}

impl<T: Numeric<T>, const ROWS: usize, const COLS: usize> Neg for SMatrix<T, ROWS, COLS> {
    type Output = SMatrix<T, ROWS, COLS>;

    //noinspection ALL
    #[inline]
    fn neg(self) -> Self::Output {
        let mut b = MF::<T, ROWS, COLS>::new_stack();
        neg(self.array(), b.array_mut());
        b
    }
}

impl<T: Numeric<T>, const ROWS: usize, const COLS: usize> Neg for &SMatrix<T, ROWS, COLS> {
    type Output = SMatrix<T, ROWS, COLS>;

    //noinspection ALL
    #[inline]
    fn neg(self) -> Self::Output {
        let mut b = MF::<T, ROWS, COLS>::new_stack();
        neg(self.array(), b.array_mut());
        b
    }
}

impl<T: Numeric<T>, const ROWS: usize, const COLS: usize> Neg for &mut SMatrix<T, ROWS, COLS> {
    type Output = SMatrix<T, ROWS, COLS>;

    //noinspection ALL
    #[inline]
    fn neg(self) -> Self::Output {
        let mut b = MF::<T, ROWS, COLS>::new_stack();
        neg(self.array(), b.array_mut());
        b
    }
}

impl<T: Numeric<T>, const ROWS: usize, const COLS: usize> Neg for HMatrix<T, ROWS, COLS> {
    type Output = HMatrix<T, ROWS, COLS>;

    //noinspection ALL
    #[inline]
    fn neg(self) -> Self::Output {
        let mut b = MF::<T, ROWS, COLS>::new_heap();
        neg(self.array(), b.array_mut());
        b
    }
}

impl<T: Numeric<T>, const ROWS: usize, const COLS: usize> Neg for &HMatrix<T, ROWS, COLS> {
    type Output = HMatrix<T, ROWS, COLS>;

    //noinspection ALL
    #[inline]
    fn neg(self) -> Self::Output {
        let mut b = MF::<T, ROWS, COLS>::new_heap();
        neg(self.array(), b.array_mut());
        b
    }
}

impl<T: Numeric<T>, const ROWS: usize, const COLS: usize> Neg for &mut HMatrix<T, ROWS, COLS> {
    type Output = HMatrix<T, ROWS, COLS>;

    //noinspection ALL
    #[inline]
    fn neg(self) -> Self::Output {
        let mut b = MF::<T, ROWS, COLS>::new_heap();
        neg(self.array(), b.array_mut());
        b
    }
}

#[cfg(test)]
mod matrix_neg_tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut a = MF::<f64, 2, 2>::new_stack();
        a[0][0] = 2.0;
        a[1][1] = 4.0;
        let b = -a;
        println!("{:?}", b);
        let a = &MF::<f64, 2, 2>::new_stack();
        let b = -a;
        println!("{:?}", b);
        let mut a = &mut MF::<f64, 2, 2>::new_stack();
        a[0][0] = 2.0;
        a[1][1] = 4.0;
        let b = -a;
        println!("{:?}", b);
    }

    #[test]
    fn test_2() {
        let mut a = MF::<f64, 2, 2>::new_heap();
        a[0][0] = 2.0;
        a[1][1] = 4.0;
        let b = -a;
        println!("{:?}", b);
        let a = &MF::<f64, 2, 2>::new_heap();
        let b = -a;
        println!("{:?}", b);
        let mut a = &mut MF::<f64, 2, 2>::new_heap();
        a[0][0] = 2.0;
        a[1][1] = 4.0;
        let b = -a;
        println!("{:?}", b);
    }
}
