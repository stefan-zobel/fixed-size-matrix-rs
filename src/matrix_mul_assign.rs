//! MulAssign implementations

use crate::matrix::*;
use crate::matrix_mul::*;
use std::marker::PhantomData;
use std::ops::{Mul, MulAssign};

struct Mult<T: Numeric<T>, const ROWS_LEFT: usize, const COLS_LEFT: usize, const COLS_RIGHT: usize>
{
    phantom: PhantomData<T>,
}

impl<T: Numeric<T>, const ROWS_LEFT: usize, const COLS_LEFT: usize, const COLS_RIGHT: usize>
    Mult<T, ROWS_LEFT, COLS_LEFT, COLS_RIGHT>
{
    //noinspection ALL
    #[inline]
    fn mul_ref_s_ref_s(
        lhs: &SMatrix<T, ROWS_LEFT, COLS_LEFT>,
        rhs: &SMatrix<T, COLS_LEFT, COLS_RIGHT>,
    ) -> SMatrix<T, ROWS_LEFT, COLS_RIGHT> {
        let mut c = MF::<T, ROWS_LEFT, COLS_RIGHT>::new_stack();
        multiply(lhs.array(), rhs.array(), c.array_mut());
        c
    }

    //noinspection ALL
    #[inline]
    fn mul_ref_s_ref_h(
        lhs: &SMatrix<T, ROWS_LEFT, COLS_LEFT>,
        rhs: &HMatrix<T, COLS_LEFT, COLS_RIGHT>,
    ) -> HMatrix<T, ROWS_LEFT, COLS_RIGHT> {
        let mut c = MF::<T, ROWS_LEFT, COLS_RIGHT>::new_heap();
        multiply(lhs.array(), rhs.array(), c.array_mut());
        c
    }

    //noinspection ALL
    #[inline]
    fn mul_ref_h_ref_h(
        lhs: &HMatrix<T, ROWS_LEFT, COLS_LEFT>,
        rhs: &HMatrix<T, COLS_LEFT, COLS_RIGHT>,
    ) -> HMatrix<T, ROWS_LEFT, COLS_RIGHT> {
        let mut c = MF::<T, ROWS_LEFT, COLS_RIGHT>::new_heap();
        multiply(lhs.array(), rhs.array(), c.array_mut());
        c
    }

    //noinspection ALL
    #[inline]
    fn mul_ref_h_ref_s(
        lhs: &HMatrix<T, ROWS_LEFT, COLS_LEFT>,
        rhs: &SMatrix<T, COLS_LEFT, COLS_RIGHT>,
    ) -> HMatrix<T, ROWS_LEFT, COLS_RIGHT> {
        let mut c = MF::<T, ROWS_LEFT, COLS_RIGHT>::new_heap();
        multiply(lhs.array(), rhs.array(), c.array_mut());
        c
    }
}

// 17) &mut SMatrix * SMatrix
impl<T: Numeric<T>, const ROWS_LEFT: usize, const COLS_LEFT: usize, const COLS_RIGHT: usize>
    Mul<SMatrix<T, COLS_LEFT, COLS_RIGHT>> for &mut SMatrix<T, ROWS_LEFT, COLS_LEFT>
{
    type Output = SMatrix<T, ROWS_LEFT, COLS_RIGHT>;

    //noinspection ALL
    #[inline]
    fn mul(self, rhs: SMatrix<T, COLS_LEFT, COLS_RIGHT>) -> Self::Output {
        Mult::<T, ROWS_LEFT, COLS_LEFT, COLS_RIGHT>::mul_ref_s_ref_s(self, &rhs)
    }
}

// 18) &mut SMatrix * HMatrix
impl<T: Numeric<T>, const ROWS_LEFT: usize, const COLS_LEFT: usize, const COLS_RIGHT: usize>
    Mul<HMatrix<T, COLS_LEFT, COLS_RIGHT>> for &mut SMatrix<T, ROWS_LEFT, COLS_LEFT>
{
    type Output = HMatrix<T, ROWS_LEFT, COLS_RIGHT>;

    //noinspection ALL
    #[inline]
    fn mul(self, rhs: HMatrix<T, COLS_LEFT, COLS_RIGHT>) -> Self::Output {
        Mult::<T, ROWS_LEFT, COLS_LEFT, COLS_RIGHT>::mul_ref_s_ref_h(self, &rhs)
    }
}

// 19) &mut SMatrix * &SMatrix
impl<T: Numeric<T>, const ROWS_LEFT: usize, const COLS_LEFT: usize, const COLS_RIGHT: usize>
    Mul<&SMatrix<T, COLS_LEFT, COLS_RIGHT>> for &mut SMatrix<T, ROWS_LEFT, COLS_LEFT>
{
    type Output = SMatrix<T, ROWS_LEFT, COLS_RIGHT>;

    //noinspection ALL
    #[inline]
    fn mul(self, rhs: &SMatrix<T, COLS_LEFT, COLS_RIGHT>) -> Self::Output {
        Mult::<T, ROWS_LEFT, COLS_LEFT, COLS_RIGHT>::mul_ref_s_ref_s(self, rhs)
    }
}

// 20) &mut SMatrix * &mut SMatrix
impl<T: Numeric<T>, const ROWS_LEFT: usize, const COLS_LEFT: usize, const COLS_RIGHT: usize>
    Mul<&mut SMatrix<T, COLS_LEFT, COLS_RIGHT>> for &mut SMatrix<T, ROWS_LEFT, COLS_LEFT>
{
    type Output = SMatrix<T, ROWS_LEFT, COLS_RIGHT>;

    //noinspection ALL
    #[inline]
    fn mul(self, rhs: &mut SMatrix<T, COLS_LEFT, COLS_RIGHT>) -> Self::Output {
        Mult::<T, ROWS_LEFT, COLS_LEFT, COLS_RIGHT>::mul_ref_s_ref_s(self, rhs)
    }
}

// 21) &mut SMatrix * &HMatrix
impl<T: Numeric<T>, const ROWS_LEFT: usize, const COLS_LEFT: usize, const COLS_RIGHT: usize>
    Mul<&HMatrix<T, COLS_LEFT, COLS_RIGHT>> for &mut SMatrix<T, ROWS_LEFT, COLS_LEFT>
{
    type Output = HMatrix<T, ROWS_LEFT, COLS_RIGHT>;

    //noinspection ALL
    #[inline]
    fn mul(self, rhs: &HMatrix<T, COLS_LEFT, COLS_RIGHT>) -> Self::Output {
        Mult::<T, ROWS_LEFT, COLS_LEFT, COLS_RIGHT>::mul_ref_s_ref_h(self, rhs)
    }
}

// 22) &mut SMatrix * &mut HMatrix
impl<T: Numeric<T>, const ROWS_LEFT: usize, const COLS_LEFT: usize, const COLS_RIGHT: usize>
    Mul<&mut HMatrix<T, COLS_LEFT, COLS_RIGHT>> for &mut SMatrix<T, ROWS_LEFT, COLS_LEFT>
{
    type Output = HMatrix<T, ROWS_LEFT, COLS_RIGHT>;

    //noinspection ALL
    #[inline]
    fn mul(self, rhs: &mut HMatrix<T, COLS_LEFT, COLS_RIGHT>) -> Self::Output {
        Mult::<T, ROWS_LEFT, COLS_LEFT, COLS_RIGHT>::mul_ref_s_ref_h(self, rhs)
    }
}

// 23) &mut HMatrix * HMatrix
impl<T: Numeric<T>, const ROWS_LEFT: usize, const COLS_LEFT: usize, const COLS_RIGHT: usize>
    Mul<HMatrix<T, COLS_LEFT, COLS_RIGHT>> for &mut HMatrix<T, ROWS_LEFT, COLS_LEFT>
{
    type Output = HMatrix<T, ROWS_LEFT, COLS_RIGHT>;

    //noinspection ALL
    #[inline]
    fn mul(self, rhs: HMatrix<T, COLS_LEFT, COLS_RIGHT>) -> Self::Output {
        Mult::<T, ROWS_LEFT, COLS_LEFT, COLS_RIGHT>::mul_ref_h_ref_h(self, &rhs)
    }
}

// 24) &mut HMatrix * &HMatrix
impl<T: Numeric<T>, const ROWS_LEFT: usize, const COLS_LEFT: usize, const COLS_RIGHT: usize>
    Mul<&HMatrix<T, COLS_LEFT, COLS_RIGHT>> for &mut HMatrix<T, ROWS_LEFT, COLS_LEFT>
{
    type Output = HMatrix<T, ROWS_LEFT, COLS_RIGHT>;

    //noinspection ALL
    #[inline]
    fn mul(self, rhs: &HMatrix<T, COLS_LEFT, COLS_RIGHT>) -> Self::Output {
        Mult::<T, ROWS_LEFT, COLS_LEFT, COLS_RIGHT>::mul_ref_h_ref_h(self, rhs)
    }
}

// 25) &mut HMatrix * &mut HMatrix
impl<T: Numeric<T>, const ROWS_LEFT: usize, const COLS_LEFT: usize, const COLS_RIGHT: usize>
    Mul<&mut HMatrix<T, COLS_LEFT, COLS_RIGHT>> for &mut HMatrix<T, ROWS_LEFT, COLS_LEFT>
{
    type Output = HMatrix<T, ROWS_LEFT, COLS_RIGHT>;

    //noinspection ALL
    #[inline]
    fn mul(self, rhs: &mut HMatrix<T, COLS_LEFT, COLS_RIGHT>) -> Self::Output {
        Mult::<T, ROWS_LEFT, COLS_LEFT, COLS_RIGHT>::mul_ref_h_ref_h(self, rhs)
    }
}

// 26) &mut HMatrix * SMatrix
impl<T: Numeric<T>, const ROWS_LEFT: usize, const COLS_LEFT: usize, const COLS_RIGHT: usize>
    Mul<SMatrix<T, COLS_LEFT, COLS_RIGHT>> for &mut HMatrix<T, ROWS_LEFT, COLS_LEFT>
{
    type Output = HMatrix<T, ROWS_LEFT, COLS_RIGHT>;

    //noinspection ALL
    #[inline]
    fn mul(self, rhs: SMatrix<T, COLS_LEFT, COLS_RIGHT>) -> Self::Output {
        Mult::<T, ROWS_LEFT, COLS_LEFT, COLS_RIGHT>::mul_ref_h_ref_s(self, &rhs)
    }
}

// 27) &mut HMatrix * &SMatrix
impl<T: Numeric<T>, const ROWS_LEFT: usize, const COLS_LEFT: usize, const COLS_RIGHT: usize>
    Mul<&SMatrix<T, COLS_LEFT, COLS_RIGHT>> for &mut HMatrix<T, ROWS_LEFT, COLS_LEFT>
{
    type Output = HMatrix<T, ROWS_LEFT, COLS_RIGHT>;

    //noinspection ALL
    #[inline]
    fn mul(self, rhs: &SMatrix<T, COLS_LEFT, COLS_RIGHT>) -> Self::Output {
        Mult::<T, ROWS_LEFT, COLS_LEFT, COLS_RIGHT>::mul_ref_h_ref_s(self, rhs)
    }
}

// 28) &mut HMatrix * &mut SMatrix
impl<T: Numeric<T>, const ROWS_LEFT: usize, const COLS_LEFT: usize, const COLS_RIGHT: usize>
    Mul<&mut SMatrix<T, COLS_LEFT, COLS_RIGHT>> for &mut HMatrix<T, ROWS_LEFT, COLS_LEFT>
{
    type Output = HMatrix<T, ROWS_LEFT, COLS_RIGHT>;

    //noinspection ALL
    #[inline]
    fn mul(self, rhs: &mut SMatrix<T, COLS_LEFT, COLS_RIGHT>) -> Self::Output {
        Mult::<T, ROWS_LEFT, COLS_LEFT, COLS_RIGHT>::mul_ref_h_ref_s(self, rhs)
    }
}

// ------------------

// A1) SMatrix *= SMatrix
impl<T: Numeric<T>, const ROWS: usize> MulAssign<Self> for SMatrix<T, ROWS, ROWS> {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        let res = Mult::<T, ROWS, ROWS, ROWS>::mul_ref_s_ref_s(self, &rhs);
        self.array_mut().copy_from_slice(res.array());
    }
}

// A2) SMatrix *= &SMatrix
impl<T: Numeric<T>, const ROWS: usize> MulAssign<&Self> for SMatrix<T, ROWS, ROWS> {
    #[inline]
    fn mul_assign(&mut self, rhs: &Self) {
        let res = Mult::<T, ROWS, ROWS, ROWS>::mul_ref_s_ref_s(self, rhs);
        self.array_mut().copy_from_slice(res.array());
    }
}

// A3) SMatrix *= &mut SMatrix
impl<T: Numeric<T>, const ROWS: usize> MulAssign<&mut Self> for SMatrix<T, ROWS, ROWS> {
    #[inline]
    fn mul_assign(&mut self, rhs: &mut Self) {
        let res = Mult::<T, ROWS, ROWS, ROWS>::mul_ref_s_ref_s(self, rhs);
        self.array_mut().copy_from_slice(res.array());
    }
}

// A4) SMatrix *= HMatrix
impl<T: Numeric<T>, const ROWS: usize> MulAssign<HMatrix<T, ROWS, ROWS>>
    for SMatrix<T, ROWS, ROWS>
{
    #[inline]
    fn mul_assign(&mut self, rhs: HMatrix<T, ROWS, ROWS>) {
        let res = Mult::<T, ROWS, ROWS, ROWS>::mul_ref_s_ref_h(self, &rhs);
        self.array_mut().copy_from_slice(res.array());
    }
}

// A5) SMatrix *= &HMatrix
impl<T: Numeric<T>, const ROWS: usize> MulAssign<&HMatrix<T, ROWS, ROWS>>
    for SMatrix<T, ROWS, ROWS>
{
    #[inline]
    fn mul_assign(&mut self, rhs: &HMatrix<T, ROWS, ROWS>) {
        let res = Mult::<T, ROWS, ROWS, ROWS>::mul_ref_s_ref_h(self, rhs);
        self.array_mut().copy_from_slice(res.array());
    }
}

// A6) SMatrix *= &mut HMatrix
impl<T: Numeric<T>, const ROWS: usize> MulAssign<&mut HMatrix<T, ROWS, ROWS>>
for SMatrix<T, ROWS, ROWS>
{
    #[inline]
    fn mul_assign(&mut self, rhs: &mut HMatrix<T, ROWS, ROWS>) {
        let res = Mult::<T, ROWS, ROWS, ROWS>::mul_ref_s_ref_h(self, rhs);
        self.array_mut().copy_from_slice(res.array());
    }
}

// B1) HMatrix *= HMatrix
impl<T: Numeric<T>, const ROWS: usize> MulAssign<Self> for HMatrix<T, ROWS, ROWS> {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        let res = Mult::<T, ROWS, ROWS, ROWS>::mul_ref_h_ref_h(self, &rhs);
        self.array_mut().copy_from_slice(res.array());
    }
}

// B2) HMatrix *= &HMatrix
impl<T: Numeric<T>, const ROWS: usize> MulAssign<&Self> for HMatrix<T, ROWS, ROWS> {
    #[inline]
    fn mul_assign(&mut self, rhs: &Self) {
        let res = Mult::<T, ROWS, ROWS, ROWS>::mul_ref_h_ref_h(self, rhs);
        self.array_mut().copy_from_slice(res.array());
    }
}

// B3) HMatrix *= &mut HMatrix
impl<T: Numeric<T>, const ROWS: usize> MulAssign<&mut Self> for HMatrix<T, ROWS, ROWS> {
    #[inline]
    fn mul_assign(&mut self, rhs: &mut Self) {
        let res = Mult::<T, ROWS, ROWS, ROWS>::mul_ref_h_ref_h(self, rhs);
        self.array_mut().copy_from_slice(res.array());
    }
}

// B4) HMatrix *= SMatrix
impl<T: Numeric<T>, const ROWS: usize> MulAssign<SMatrix<T, ROWS, ROWS>>
    for HMatrix<T, ROWS, ROWS>
{
    #[inline]
    fn mul_assign(&mut self, rhs: SMatrix<T, ROWS, ROWS>) {
        let res = Mult::<T, ROWS, ROWS, ROWS>::mul_ref_h_ref_s(self, &rhs);
        self.array_mut().copy_from_slice(res.array());
    }
}

// B5) HMatrix *= &SMatrix
impl<T: Numeric<T>, const ROWS: usize> MulAssign<&SMatrix<T, ROWS, ROWS>>
    for HMatrix<T, ROWS, ROWS>
{
    #[inline]
    fn mul_assign(&mut self, rhs: &SMatrix<T, ROWS, ROWS>) {
        let res = Mult::<T, ROWS, ROWS, ROWS>::mul_ref_h_ref_s(self, rhs);
        self.array_mut().copy_from_slice(res.array());
    }
}

// B6) HMatrix *= &mut SMatrix
impl<T: Numeric<T>, const ROWS: usize> MulAssign<&mut SMatrix<T, ROWS, ROWS>>
for HMatrix<T, ROWS, ROWS>
{
    #[inline]
    fn mul_assign(&mut self, rhs: &mut SMatrix<T, ROWS, ROWS>) {
        let res = Mult::<T, ROWS, ROWS, ROWS>::mul_ref_h_ref_s(self, rhs);
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
        let a6 = &mut MF::<f32, 4, 4>::new_stack();
        let a7 = &mut MF::<f32, 4, 4>::new_heap();
        a1 *= a2;
        a1 *= a3;
        a1 *= a4;
        a1 *= a5;
        a1 *= a6;
        a1 *= a7;
        let mut b1 = MF::<f64, 8, 8>::new_heap();
        let b2 = MF::<f64, 8, 8>::new_heap();
        let b3 = MF::<f64, 8, 8>::new_stack();
        let b4 = &MF::<f64, 8, 8>::new_heap();
        let b5 = &MF::<f64, 8, 8>::new_stack();
        let b6 = &mut MF::<f64, 8, 8>::new_heap();
        let b7 = &mut MF::<f64, 8, 8>::new_stack();
        b1 *= b2;
        b1 *= b3;
        b1 *= b4;
        b1 *= b5;
        b1 *= b6;
        b1 *= b7;

        /*
        let c1 = &mut MF::<f32, 4, 4>::new_stack();
        let c2 = MF::<f32, 4, 4>::new_stack();
        c1 *= c2;
        */

        let d1 = &mut MF::<f32, 4, 4>::new_stack();
        let d2 = MF::<f32, 4, 4>::new_stack();
        let _d = d1 * d2;

        let e1 = MF::<f32, 4, 4>::new_stack();
        let e2 = &mut MF::<f32, 4, 4>::new_stack();
        let _e = e1 * e2;

        let f1 = &mut MF::<f32, 4, 4>::new_stack();
        let f2 = &mut MF::<f32, 4, 4>::new_stack();
        let _f = f1 * f2;
    }
}
