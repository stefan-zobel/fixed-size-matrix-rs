//! Inplace scalar multiplication implemented as MulAssign

use crate::matrix::*;
use std::ops::MulAssign;

#[inline]
fn mul_assign_scalar<T: Numeric<T>, const ROWS: usize, const COLS: usize>(
    a: &mut [[T; COLS]; ROWS],
    scalar: T,
) {
    for row in a.iter_mut().take(ROWS) {
        for cell in row.iter_mut().take(COLS) {
            *cell *= scalar;
        }
    }
}

// A0) SMatrix *= Numeric<T>
/// Inplace scalar multiplication for [SMatrix](SMatrix) implemented as `MulAssign`.
impl<T: Numeric<T>, const ROWS: usize, const COLS: usize> MulAssign<T> for SMatrix<T, ROWS, COLS> {
    #[inline]
    fn mul_assign(&mut self, rhs: T) {
        mul_assign_scalar(self.array_mut(), rhs);
    }
}

// B0) HMatrix *= Numeric<T>
/// Inplace scalar multiplication for [HMatrix](HMatrix) implemented as `MulAssign`.
impl<T: Numeric<T>, const ROWS: usize, const COLS: usize> MulAssign<T> for HMatrix<T, ROWS, COLS> {
    #[inline]
    fn mul_assign(&mut self, rhs: T) {
        mul_assign_scalar(self.array_mut(), rhs);
    }
}

// A1) &mut SMatrix *= Numeric<T>
/// Inplace scalar multiplication for a `&mut` [SMatrix](SMatrix) implemented as `MulAssign`.
impl<T: Numeric<T>, const ROWS: usize, const COLS: usize> MulAssign<T>
    for &mut SMatrix<T, ROWS, COLS>
{
    #[inline]
    fn mul_assign(&mut self, rhs: T) {
        mul_assign_scalar(self.array_mut(), rhs);
    }
}

// B1) &mut HMatrix *= Numeric<T>
/// Inplace scalar multiplication for a `&mut` [HMatrix](HMatrix) implemented as `MulAssign`.
impl<T: Numeric<T>, const ROWS: usize, const COLS: usize> MulAssign<T>
    for &mut HMatrix<T, ROWS, COLS>
{
    #[inline]
    fn mul_assign(&mut self, rhs: T) {
        mul_assign_scalar(self.array_mut(), rhs);
    }
}

#[cfg(test)]
mod scalar_mul_inplace_tests {
    use super::*;

    #[test]
    fn test_1_smatrix() {
        let mut a = MF::<f32, 2, 2>::new_stack();
        a[0][0] = 2.0f32;
        a[1][1] = 4.0f32;
        a *= 3.0f32;
        println!("SCALAR MulAssign a f32: {:?}", a);
        assert_eq!(a[0][0], 6.0f32);
        assert_eq!(a[1][1], 12.0f32);
        let mut a = MF::<f64, 2, 2>::new_stack();
        a[0][0] = 2.0f64;
        a[1][1] = 4.0f64;
        a *= 3.0f64;
        println!("SCALAR MulAssign a f64: {:?}", a);
        assert_eq!(a[0][0], 6.0f64);
        assert_eq!(a[1][1], 12.0f64);
        let mut a = MF::<i8, 2, 2>::new_stack();
        a[0][0] = 2i8;
        a[1][1] = 4i8;
        a *= 3i8;
        println!("SCALAR MulAssign a i8: {:?}", a);
        assert_eq!(a[0][0], 6i8);
        assert_eq!(a[1][1], 12i8);
        let mut a = MF::<i128, 2, 2>::new_stack();
        a[0][0] = 2i128;
        a[1][1] = 4i128;
        a *= 3i128;
        println!("SCALAR MulAssign a i128: {:?}", a);
        assert_eq!(a[0][0], 6i128);
        assert_eq!(a[1][1], 12i128);
    }

    #[test]
    fn test_1_hmatrix() {
        let mut a = MF::<f32, 2, 2>::new_heap();
        a[0][0] = 2.0f32;
        a[1][1] = 4.0f32;
        a *= 3.0f32;
        println!("SCALAR MulAssign a f32: {:?}", a);
        assert_eq!(a[0][0], 6.0f32);
        assert_eq!(a[1][1], 12.0f32);
        let mut a = MF::<f64, 2, 2>::new_heap();
        a[0][0] = 2.0f64;
        a[1][1] = 4.0f64;
        a *= 3.0f64;
        println!("SCALAR MulAssign a f64: {:?}", a);
        assert_eq!(a[0][0], 6.0f64);
        assert_eq!(a[1][1], 12.0f64);
        let mut a = MF::<i8, 2, 2>::new_heap();
        a[0][0] = 2i8;
        a[1][1] = 4i8;
        a *= 3i8;
        println!("SCALAR MulAssign a i8: {:?}", a);
        assert_eq!(a[0][0], 6i8);
        assert_eq!(a[1][1], 12i8);
        let mut a = MF::<i128, 2, 2>::new_heap();
        a[0][0] = 2i128;
        a[1][1] = 4i128;
        a *= 3i128;
        println!("SCALAR MulAssign a i128: {:?}", a);
        assert_eq!(a[0][0], 6i128);
        assert_eq!(a[1][1], 12i128);
    }

    #[test]
    fn test_2_smatrix() {
        let mut a = &mut MF::<f32, 2, 2>::new_stack();
        a[0][0] = 2.0f32;
        a[1][1] = 4.0f32;
        a *= 3.0f32;
        println!("SCALAR MulAssign a f32: {:?}", a);
        assert_eq!(a[0][0], 6.0f32);
        assert_eq!(a[1][1], 12.0f32);
        let mut a = &mut MF::<f64, 2, 2>::new_stack();
        a[0][0] = 2.0f64;
        a[1][1] = 4.0f64;
        a *= 3.0f64;
        println!("SCALAR MulAssign a f64: {:?}", a);
        assert_eq!(a[0][0], 6.0f64);
        assert_eq!(a[1][1], 12.0f64);
        let mut a = &mut MF::<i8, 2, 2>::new_stack();
        a[0][0] = 2i8;
        a[1][1] = 4i8;
        a *= 3i8;
        println!("SCALAR MulAssign a i8: {:?}", a);
        assert_eq!(a[0][0], 6i8);
        assert_eq!(a[1][1], 12i8);
        let mut a = &mut MF::<i128, 2, 2>::new_stack();
        a[0][0] = 2i128;
        a[1][1] = 4i128;
        a *= 3i128;
        println!("SCALAR MulAssign a i128: {:?}", a);
        assert_eq!(a[0][0], 6i128);
        assert_eq!(a[1][1], 12i128);
    }

    #[test]
    fn test_2_hmatrix() {
        let mut a = &mut MF::<f32, 2, 2>::new_heap();
        a[0][0] = 2.0f32;
        a[1][1] = 4.0f32;
        a *= 3.0f32;
        println!("SCALAR MulAssign a f32: {:?}", a);
        assert_eq!(a[0][0], 6.0f32);
        assert_eq!(a[1][1], 12.0f32);
        let mut a = &mut MF::<f64, 2, 2>::new_heap();
        a[0][0] = 2.0f64;
        a[1][1] = 4.0f64;
        a *= 3.0f64;
        println!("SCALAR MulAssign a f64: {:?}", a);
        assert_eq!(a[0][0], 6.0f64);
        assert_eq!(a[1][1], 12.0f64);
        let mut a = &mut MF::<i8, 2, 2>::new_heap();
        a[0][0] = 2i8;
        a[1][1] = 4i8;
        a *= 3i8;
        println!("SCALAR MulAssign a i8: {:?}", a);
        assert_eq!(a[0][0], 6i8);
        assert_eq!(a[1][1], 12i8);
        let mut a = &mut MF::<i128, 2, 2>::new_heap();
        a[0][0] = 2i128;
        a[1][1] = 4i128;
        a *= 3i128;
        println!("SCALAR MulAssign a i128: {:?}", a);
        assert_eq!(a[0][0], 6i128);
        assert_eq!(a[1][1], 12i128);
    }
}
