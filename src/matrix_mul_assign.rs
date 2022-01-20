//! MulAssign implementations

use crate::matrix_mul::*;
use crate::matrix::*;
use std::ops::{Add, Mul, MulAssign};

// 17) &mut SMatrix * SMatrix
impl<T: Numeric, const ROWS_LEFT: usize, const COLS_LEFT: usize, const COLS_RIGHT: usize>
    Mul<SMatrix<T, COLS_LEFT, COLS_RIGHT>> for &mut SMatrix<T, ROWS_LEFT, COLS_LEFT>
where
    T: Mul<Output = T> + Add<Output = T>,
{
    type Output = SMatrix<T, ROWS_LEFT, COLS_RIGHT>;

    //noinspection ALL
    #[inline]
    fn mul(self, rhs: SMatrix<T, COLS_LEFT, COLS_RIGHT>) -> Self::Output {
        let mut c = MF::<T, ROWS_LEFT, COLS_RIGHT>::new_stack();
        multiply(self.array(), rhs.array(), c.array_mut());
        c
    }
}

// 18) &mut SMatrix * HMatrix
impl<T: Numeric, const ROWS_LEFT: usize, const COLS_LEFT: usize, const COLS_RIGHT: usize>
    Mul<HMatrix<T, COLS_LEFT, COLS_RIGHT>> for &mut SMatrix<T, ROWS_LEFT, COLS_LEFT>
where
    T: Mul<Output = T> + Add<Output = T>,
{
    type Output = HMatrix<T, ROWS_LEFT, COLS_RIGHT>;

    //noinspection ALL
    #[inline]
    fn mul(self, rhs: HMatrix<T, COLS_LEFT, COLS_RIGHT>) -> Self::Output {
        let mut c = MF::<T, ROWS_LEFT, COLS_RIGHT>::new_heap();
        multiply(self.array(), rhs.array(), c.array_mut());
        c
    }
}

// 19) &mut SMatrix * &SMatrix
impl<T: Numeric, const ROWS_LEFT: usize, const COLS_LEFT: usize, const COLS_RIGHT: usize>
    Mul<&SMatrix<T, COLS_LEFT, COLS_RIGHT>> for &mut SMatrix<T, ROWS_LEFT, COLS_LEFT>
where
    T: Mul<Output = T> + Add<Output = T>,
{
    type Output = SMatrix<T, ROWS_LEFT, COLS_RIGHT>;

    //noinspection ALL
    #[inline]
    fn mul(self, rhs: &SMatrix<T, COLS_LEFT, COLS_RIGHT>) -> Self::Output {
        let mut c = MF::<T, ROWS_LEFT, COLS_RIGHT>::new_stack();
        multiply(self.array(), rhs.array(), c.array_mut());
        c
    }
}

// 20) &mut SMatrix * &HMatrix
impl<T: Numeric, const ROWS_LEFT: usize, const COLS_LEFT: usize, const COLS_RIGHT: usize>
    Mul<&HMatrix<T, COLS_LEFT, COLS_RIGHT>> for &mut SMatrix<T, ROWS_LEFT, COLS_LEFT>
where
    T: Mul<Output = T> + Add<Output = T>,
{
    type Output = HMatrix<T, ROWS_LEFT, COLS_RIGHT>;

    //noinspection ALL
    #[inline]
    fn mul(self, rhs: &HMatrix<T, COLS_LEFT, COLS_RIGHT>) -> Self::Output {
        let mut c = MF::<T, ROWS_LEFT, COLS_RIGHT>::new_heap();
        multiply(self.array(), rhs.array(), c.array_mut());
        c
    }
}

// 21) &mut HMatrix * HMatrix
impl<T: Numeric, const ROWS_LEFT: usize, const COLS_LEFT: usize, const COLS_RIGHT: usize>
    Mul<HMatrix<T, COLS_LEFT, COLS_RIGHT>> for &mut HMatrix<T, ROWS_LEFT, COLS_LEFT>
where
    T: Mul<Output = T> + Add<Output = T>,
{
    type Output = HMatrix<T, ROWS_LEFT, COLS_RIGHT>;

    //noinspection ALL
    #[inline]
    fn mul(self, rhs: HMatrix<T, COLS_LEFT, COLS_RIGHT>) -> Self::Output {
        let mut c = MF::<T, ROWS_LEFT, COLS_RIGHT>::new_heap();
        multiply(self.array(), rhs.array(), c.array_mut());
        c
    }
}

// 22) &mut HMatrix * &HMatrix
impl<T: Numeric, const ROWS_LEFT: usize, const COLS_LEFT: usize, const COLS_RIGHT: usize>
    Mul<&HMatrix<T, COLS_LEFT, COLS_RIGHT>> for &mut HMatrix<T, ROWS_LEFT, COLS_LEFT>
where
    T: Mul<Output = T> + Add<Output = T>,
{
    type Output = HMatrix<T, ROWS_LEFT, COLS_RIGHT>;

    //noinspection ALL
    #[inline]
    fn mul(self, rhs: &HMatrix<T, COLS_LEFT, COLS_RIGHT>) -> Self::Output {
        let mut c = MF::<T, ROWS_LEFT, COLS_RIGHT>::new_heap();
        multiply(self.array(), rhs.array(), c.array_mut());
        c
    }
}

// 23) &mut HMatrix * SMatrix
impl<T: Numeric, const ROWS_LEFT: usize, const COLS_LEFT: usize, const COLS_RIGHT: usize>
    Mul<SMatrix<T, COLS_LEFT, COLS_RIGHT>> for &mut HMatrix<T, ROWS_LEFT, COLS_LEFT>
where
    T: Mul<Output = T> + Add<Output = T>,
{
    type Output = HMatrix<T, ROWS_LEFT, COLS_RIGHT>;

    //noinspection ALL
    #[inline]
    fn mul(self, rhs: SMatrix<T, COLS_LEFT, COLS_RIGHT>) -> Self::Output {
        let mut c = MF::<T, ROWS_LEFT, COLS_RIGHT>::new_heap();
        multiply(self.array(), rhs.array(), c.array_mut());
        c
    }
}

// 24) &mut HMatrix * &SMatrix
impl<T: Numeric, const ROWS_LEFT: usize, const COLS_LEFT: usize, const COLS_RIGHT: usize>
    Mul<&SMatrix<T, COLS_LEFT, COLS_RIGHT>> for &mut HMatrix<T, ROWS_LEFT, COLS_LEFT>
where
    T: Mul<Output = T> + Add<Output = T>,
{
    type Output = HMatrix<T, ROWS_LEFT, COLS_RIGHT>;

    //noinspection ALL
    #[inline]
    fn mul(self, rhs: &SMatrix<T, COLS_LEFT, COLS_RIGHT>) -> Self::Output {
        let mut c = MF::<T, ROWS_LEFT, COLS_RIGHT>::new_heap();
        multiply(self.array(), rhs.array(), c.array_mut());
        c
    }
}

// ------------------

// A1) SMatrix *= SMatrix
impl<T: Numeric, const ROWS: usize> MulAssign<Self> for SMatrix<T, ROWS, ROWS>
where
    T: Mul<Output = T> + Add<Output = T>,
{
    fn mul_assign(&mut self, rhs: Self) {
        let res = self.clone() * rhs;
        self.array_mut().copy_from_slice(res.array());
    }
}

// A2) SMatrix *= &SMatrix
impl<T: Numeric, const ROWS: usize> MulAssign<&Self> for SMatrix<T, ROWS, ROWS>
where
    T: Mul<Output = T> + Add<Output = T>,
{
    fn mul_assign(&mut self, rhs: &Self) {
        let res = self.clone() * rhs;
        self.array_mut().copy_from_slice(res.array());
    }
}

// A3) SMatrix *= HMatrix
impl<T: Numeric, const ROWS: usize> MulAssign<HMatrix<T, ROWS, ROWS>> for SMatrix<T, ROWS, ROWS>
where
    T: Mul<Output = T> + Add<Output = T>,
{
    fn mul_assign(&mut self, rhs: HMatrix<T, ROWS, ROWS>) {
        let res = self.clone() * rhs;
        self.array_mut().copy_from_slice(res.array());
    }
}

// A4) SMatrix *= &HMatrix
impl<T: Numeric, const ROWS: usize> MulAssign<&HMatrix<T, ROWS, ROWS>> for SMatrix<T, ROWS, ROWS>
where
    T: Mul<Output = T> + Add<Output = T>,
{
    fn mul_assign(&mut self, rhs: &HMatrix<T, ROWS, ROWS>) {
        let res = self.clone() * rhs;
        self.array_mut().copy_from_slice(res.array());
    }
}

// B1) HMatrix *= HMatrix
impl<T: Numeric, const ROWS: usize> MulAssign<Self> for HMatrix<T, ROWS, ROWS>
where
    T: Mul<Output = T> + Add<Output = T>,
{
    fn mul_assign(&mut self, rhs: Self) {
        let res = self.clone() * rhs;
        self.array_mut().copy_from_slice(res.array());
    }
}

// B2) HMatrix *= &HMatrix
impl<T: Numeric, const ROWS: usize> MulAssign<&Self> for HMatrix<T, ROWS, ROWS>
where
    T: Mul<Output = T> + Add<Output = T>,
{
    fn mul_assign(&mut self, rhs: &Self) {
        let res = self.clone() * rhs;
        self.array_mut().copy_from_slice(res.array());
    }
}

// B3) HMatrix *= SMatrix
impl<T: Numeric, const ROWS: usize> MulAssign<SMatrix<T, ROWS, ROWS>> for HMatrix<T, ROWS, ROWS>
where
    T: Mul<Output = T> + Add<Output = T>,
{
    fn mul_assign(&mut self, rhs: SMatrix<T, ROWS, ROWS>) {
        let res = self.clone() * rhs;
        self.array_mut().copy_from_slice(res.array());
    }
}

// B4) HMatrix *= &SMatrix
impl<T: Numeric, const ROWS: usize> MulAssign<&SMatrix<T, ROWS, ROWS>> for HMatrix<T, ROWS, ROWS>
where
    T: Mul<Output = T> + Add<Output = T>,
{
    fn mul_assign(&mut self, rhs: &SMatrix<T, ROWS, ROWS>) {
        let res = self.clone() * rhs;
        self.array_mut().copy_from_slice(res.array());
    }
}

#[cfg(test)]
mod mul_assign_tests {
    use super::*;

    #[test]
    fn test_mul_assign() {
        let mut a1 = MF::<f32, 4, 4>::new_stack();
        let a2 = MF::<f32, 4, 4>::new_stack();
        let a3 = MF::<f32, 4, 4>::new_heap();
        let a4 = &MF::<f32, 4, 4>::new_stack();
        let a5 = &MF::<f32, 4, 4>::new_heap();
        a1 *= a2;
        a1 *= a3;
        a1 *= a4;
        a1 *= a5;
        let mut b1 = MF::<f64, 8, 8>::new_heap();
        let b2 = MF::<f64, 8, 8>::new_heap();
        let b3 = MF::<f64, 8, 8>::new_stack();
        let b4 = &MF::<f64, 8, 8>::new_heap();
        let b5 = &MF::<f64, 8, 8>::new_stack();
        b1 *= b2;
        b1 *= b3;
        b1 *= b4;
        b1 *= b5;

        /*
        let c1 = &mut MF::<f32, 4, 4>::new_stack();
        let c2 = MF::<f32, 4, 4>::new_stack();
        c1 *= c2;
        */

        /*
        let d1 = &mut MF::<f32, 4, 4>::new_stack();
        let d2 = MF::<f32, 4, 4>::new_stack();
        let _d = d1 * d2;
        let e1 = MF::<f32, 4, 4>::new_stack();
        let e2 = &mut MF::<f32, 4, 4>::new_stack();
        let _e = e1 * e2;
        let f1 = &mut MF::<f32, 4, 4>::new_stack();
        let f2 = &mut MF::<f32, 4, 4>::new_stack();
        let _f = f1 * f2;
        */
    }
}
