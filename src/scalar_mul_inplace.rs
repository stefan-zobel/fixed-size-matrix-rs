//! Inplace scalar multiplication implemented as MulAssign

use crate::matrix::*;
use std::ops::MulAssign;

#[inline]
fn mul_assign_scalar<T: Numeric<T>, const ROWS: usize, const COLS: usize>(
    a: &mut [[T; COLS]; ROWS],
    scalar: T,
) {
    for row in 0..ROWS {
        for col in 0..COLS {
            a[row][col] *= scalar;
        }
    }
}

// SMatrix *= Numeric<T>
impl<T: Numeric<T>, const ROWS: usize, const COLS: usize> MulAssign<T> for SMatrix<T, ROWS, COLS> {
    #[inline]
    fn mul_assign(&mut self, rhs: T) {
        mul_assign_scalar(self.array_mut(), rhs);
    }
}

// HMatrix *= Numeric<T>
impl<T: Numeric<T>, const ROWS: usize, const COLS: usize> MulAssign<T> for HMatrix<T, ROWS, COLS> {
    #[inline]
    fn mul_assign(&mut self, rhs: T) {
        mul_assign_scalar(self.array_mut(), rhs);
    }
}

// &mut SMatrix *= Numeric<T>
impl<T: Numeric<T>, const ROWS: usize, const COLS: usize> MulAssign<T>
    for &mut SMatrix<T, ROWS, COLS>
{
    #[inline]
    fn mul_assign(&mut self, rhs: T) {
        mul_assign_scalar(self.array_mut(), rhs);
    }
}

// &mut HMatrix *= Numeric<T>
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
        let mut a = MF::<f64, 2, 2>::new_stack();
        a[0][0] = 2.0f64;
        a[1][1] = 4.0f64;
        a *= 3.0f64;
        println!("SCALAR MulAssign a f64: {:?}", a);
        let mut a = MF::<i8, 2, 2>::new_stack();
        a[0][0] = 2i8;
        a[1][1] = 4i8;
        a *= 3i8;
        println!("SCALAR MulAssign a i8: {:?}", a);
        let mut a = MF::<i128, 2, 2>::new_stack();
        a[0][0] = 2i128;
        a[1][1] = 4i128;
        a *= 3i128;
        println!("SCALAR MulAssign a i128: {:?}", a);
    }

    #[test]
    fn test_1_hmatrix() {
        let mut a = MF::<f32, 2, 2>::new_heap();
        a[0][0] = 2.0f32;
        a[1][1] = 4.0f32;
        a *= 3.0f32;
        println!("SCALAR MulAssign a f32: {:?}", a);
        let mut a = MF::<f64, 2, 2>::new_heap();
        a[0][0] = 2.0f64;
        a[1][1] = 4.0f64;
        a *= 3.0f64;
        println!("SCALAR MulAssign a f64: {:?}", a);
        let mut a = MF::<i8, 2, 2>::new_heap();
        a[0][0] = 2i8;
        a[1][1] = 4i8;
        a *= 3i8;
        println!("SCALAR MulAssign a i8: {:?}", a);
        let mut a = MF::<i128, 2, 2>::new_heap();
        a[0][0] = 2i128;
        a[1][1] = 4i128;
        a *= 3i128;
        println!("SCALAR MulAssign a i128: {:?}", a);
    }

    #[test]
    fn test_2_smatrix() {
        let mut a = &mut MF::<f32, 2, 2>::new_stack();
        a[0][0] = 2.0f32;
        a[1][1] = 4.0f32;
        a *= 3.0f32;
        println!("SCALAR MulAssign a f32: {:?}", a);
        let mut a = &mut MF::<f64, 2, 2>::new_stack();
        a[0][0] = 2.0f64;
        a[1][1] = 4.0f64;
        a *= 3.0f64;
        println!("SCALAR MulAssign a f64: {:?}", a);
        let mut a = &mut MF::<i8, 2, 2>::new_stack();
        a[0][0] = 2i8;
        a[1][1] = 4i8;
        a *= 3i8;
        println!("SCALAR MulAssign a i8: {:?}", a);
        let mut a = &mut MF::<i128, 2, 2>::new_stack();
        a[0][0] = 2i128;
        a[1][1] = 4i128;
        a *= 3i128;
        println!("SCALAR MulAssign a i128: {:?}", a);
    }

    #[test]
    fn test_2_hmatrix() {
        let mut a = &mut MF::<f32, 2, 2>::new_heap();
        a[0][0] = 2.0f32;
        a[1][1] = 4.0f32;
        a *= 3.0f32;
        println!("SCALAR MulAssign a f32: {:?}", a);
        let mut a = &mut MF::<f64, 2, 2>::new_heap();
        a[0][0] = 2.0f64;
        a[1][1] = 4.0f64;
        a *= 3.0f64;
        println!("SCALAR MulAssign a f64: {:?}", a);
        let mut a = &mut MF::<i8, 2, 2>::new_heap();
        a[0][0] = 2i8;
        a[1][1] = 4i8;
        a *= 3i8;
        println!("SCALAR MulAssign a i8: {:?}", a);
        let mut a = &mut MF::<i128, 2, 2>::new_heap();
        a[0][0] = 2i128;
        a[1][1] = 4i128;
        a *= 3i128;
        println!("SCALAR MulAssign a i128: {:?}", a);
    }
}
