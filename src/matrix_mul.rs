//! Mul implementations

use crate::matrix::*;
use std::ops::Mul;

#[inline]
pub(crate) fn multiply<
    T: Numeric<T>,
    const ROWS_LEFT: usize,
    const COLS_LEFT: usize,
    const COLS_RIGHT: usize,
>(
    a: &[[T; COLS_LEFT]; ROWS_LEFT],
    b: &[[T; COLS_RIGHT]; COLS_LEFT],
    c: &mut [[T; COLS_RIGHT]; ROWS_LEFT],
) {
    for row_a in 0..ROWS_LEFT {
        for col_b in 0..COLS_RIGHT {
            for col_a in 0..COLS_LEFT {
                c[row_a][col_b] += a[row_a][col_a] * b[col_a][col_b];
            }
        }
    }
}

// 1) SMatrix * SMatrix
impl<T: Numeric<T>, const ROWS_LEFT: usize, const COLS_LEFT: usize, const COLS_RIGHT: usize>
    Mul<SMatrix<T, COLS_LEFT, COLS_RIGHT>> for SMatrix<T, ROWS_LEFT, COLS_LEFT>
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

// 2) SMatrix * &SMatrix
impl<T: Numeric<T>, const ROWS_LEFT: usize, const COLS_LEFT: usize, const COLS_RIGHT: usize>
    Mul<&SMatrix<T, COLS_LEFT, COLS_RIGHT>> for SMatrix<T, ROWS_LEFT, COLS_LEFT>
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

// 3) &SMatrix * SMatrix
impl<T: Numeric<T>, const ROWS_LEFT: usize, const COLS_LEFT: usize, const COLS_RIGHT: usize>
    Mul<SMatrix<T, COLS_LEFT, COLS_RIGHT>> for &SMatrix<T, ROWS_LEFT, COLS_LEFT>
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

// 4) &SMatrix * &SMatrix
impl<T: Numeric<T>, const ROWS_LEFT: usize, const COLS_LEFT: usize, const COLS_RIGHT: usize>
    Mul<&SMatrix<T, COLS_LEFT, COLS_RIGHT>> for &SMatrix<T, ROWS_LEFT, COLS_LEFT>
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

// 5) SMatrix * HMatrix
impl<T: Numeric<T>, const ROWS_LEFT: usize, const COLS_LEFT: usize, const COLS_RIGHT: usize>
    Mul<HMatrix<T, COLS_LEFT, COLS_RIGHT>> for SMatrix<T, ROWS_LEFT, COLS_LEFT>
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

// 6) SMatrix * &HMatrix
impl<T: Numeric<T>, const ROWS_LEFT: usize, const COLS_LEFT: usize, const COLS_RIGHT: usize>
    Mul<&HMatrix<T, COLS_LEFT, COLS_RIGHT>> for SMatrix<T, ROWS_LEFT, COLS_LEFT>
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

// 7) &SMatrix * HMatrix
impl<T: Numeric<T>, const ROWS_LEFT: usize, const COLS_LEFT: usize, const COLS_RIGHT: usize>
    Mul<HMatrix<T, COLS_LEFT, COLS_RIGHT>> for &SMatrix<T, ROWS_LEFT, COLS_LEFT>
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

// 8) &SMatrix * &HMatrix
impl<T: Numeric<T>, const ROWS_LEFT: usize, const COLS_LEFT: usize, const COLS_RIGHT: usize>
    Mul<&HMatrix<T, COLS_LEFT, COLS_RIGHT>> for &SMatrix<T, ROWS_LEFT, COLS_LEFT>
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

// 9) HMatrix * HMatrix
impl<T: Numeric<T>, const ROWS_LEFT: usize, const COLS_LEFT: usize, const COLS_RIGHT: usize>
    Mul<HMatrix<T, COLS_LEFT, COLS_RIGHT>> for HMatrix<T, ROWS_LEFT, COLS_LEFT>
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

// 10) HMatrix * &HMatrix
impl<T: Numeric<T>, const ROWS_LEFT: usize, const COLS_LEFT: usize, const COLS_RIGHT: usize>
    Mul<&HMatrix<T, COLS_LEFT, COLS_RIGHT>> for HMatrix<T, ROWS_LEFT, COLS_LEFT>
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

// 11) &HMatrix * HMatrix
impl<T: Numeric<T>, const ROWS_LEFT: usize, const COLS_LEFT: usize, const COLS_RIGHT: usize>
    Mul<HMatrix<T, COLS_LEFT, COLS_RIGHT>> for &HMatrix<T, ROWS_LEFT, COLS_LEFT>
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

// 12) &HMatrix * &HMatrix
impl<T: Numeric<T>, const ROWS_LEFT: usize, const COLS_LEFT: usize, const COLS_RIGHT: usize>
    Mul<&HMatrix<T, COLS_LEFT, COLS_RIGHT>> for &HMatrix<T, ROWS_LEFT, COLS_LEFT>
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

// 13) HMatrix * SMatrix
impl<T: Numeric<T>, const ROWS_LEFT: usize, const COLS_LEFT: usize, const COLS_RIGHT: usize>
    Mul<SMatrix<T, COLS_LEFT, COLS_RIGHT>> for HMatrix<T, ROWS_LEFT, COLS_LEFT>
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

// 14) HMatrix * &SMatrix
impl<T: Numeric<T>, const ROWS_LEFT: usize, const COLS_LEFT: usize, const COLS_RIGHT: usize>
    Mul<&SMatrix<T, COLS_LEFT, COLS_RIGHT>> for HMatrix<T, ROWS_LEFT, COLS_LEFT>
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

// 15) &HMatrix * SMatrix
impl<T: Numeric<T>, const ROWS_LEFT: usize, const COLS_LEFT: usize, const COLS_RIGHT: usize>
    Mul<SMatrix<T, COLS_LEFT, COLS_RIGHT>> for &HMatrix<T, ROWS_LEFT, COLS_LEFT>
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

// 16) &HMatrix * &SMatrix
impl<T: Numeric<T>, const ROWS_LEFT: usize, const COLS_LEFT: usize, const COLS_RIGHT: usize>
    Mul<&SMatrix<T, COLS_LEFT, COLS_RIGHT>> for &HMatrix<T, ROWS_LEFT, COLS_LEFT>
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

// 29) SMatrix * &mut SMatrix
impl<T: Numeric<T>, const ROWS_LEFT: usize, const COLS_LEFT: usize, const COLS_RIGHT: usize>
    Mul<&mut SMatrix<T, COLS_LEFT, COLS_RIGHT>> for SMatrix<T, ROWS_LEFT, COLS_LEFT>
{
    type Output = SMatrix<T, ROWS_LEFT, COLS_RIGHT>;

    //noinspection ALL
    #[inline]
    fn mul(self, rhs: &mut SMatrix<T, COLS_LEFT, COLS_RIGHT>) -> Self::Output {
        let mut c = MF::<T, ROWS_LEFT, COLS_RIGHT>::new_stack();
        multiply(self.array(), rhs.array(), c.array_mut());
        c
    }
}

// 30) SMatrix * &mut HMatrix
impl<T: Numeric<T>, const ROWS_LEFT: usize, const COLS_LEFT: usize, const COLS_RIGHT: usize>
    Mul<&mut HMatrix<T, COLS_LEFT, COLS_RIGHT>> for SMatrix<T, ROWS_LEFT, COLS_LEFT>
{
    type Output = HMatrix<T, ROWS_LEFT, COLS_RIGHT>;

    //noinspection ALL
    #[inline]
    fn mul(self, rhs: &mut HMatrix<T, COLS_LEFT, COLS_RIGHT>) -> Self::Output {
        let mut c = MF::<T, ROWS_LEFT, COLS_RIGHT>::new_heap();
        multiply(self.array(), rhs.array(), c.array_mut());
        c
    }
}

// 31) HMatrix * &mut HMatrix
impl<T: Numeric<T>, const ROWS_LEFT: usize, const COLS_LEFT: usize, const COLS_RIGHT: usize>
    Mul<&mut HMatrix<T, COLS_LEFT, COLS_RIGHT>> for HMatrix<T, ROWS_LEFT, COLS_LEFT>
{
    type Output = HMatrix<T, ROWS_LEFT, COLS_RIGHT>;

    //noinspection ALL
    #[inline]
    fn mul(self, rhs: &mut HMatrix<T, COLS_LEFT, COLS_RIGHT>) -> Self::Output {
        let mut c = MF::<T, ROWS_LEFT, COLS_RIGHT>::new_heap();
        multiply(self.array(), rhs.array(), c.array_mut());
        c
    }
}

// 32) HMatrix * &mut SMatrix
impl<T: Numeric<T>, const ROWS_LEFT: usize, const COLS_LEFT: usize, const COLS_RIGHT: usize>
    Mul<&mut SMatrix<T, COLS_LEFT, COLS_RIGHT>> for HMatrix<T, ROWS_LEFT, COLS_LEFT>
{
    type Output = HMatrix<T, ROWS_LEFT, COLS_RIGHT>;

    //noinspection ALL
    #[inline]
    fn mul(self, rhs: &mut SMatrix<T, COLS_LEFT, COLS_RIGHT>) -> Self::Output {
        let mut c = MF::<T, ROWS_LEFT, COLS_RIGHT>::new_heap();
        multiply(self.array(), rhs.array(), c.array_mut());
        c
    }
}

#[cfg(test)]
mod mul_tests {
    use super::*;

    #[test]
    fn test_mul() {
        let a1 = MF::<f32, 4, 4>::new_stack();
        let a2 = MF::<f32, 4, 2>::new_stack();
        let a3 = MF::<f32, 4, 100>::new_heap();
        let b1 = MF::<f64, 100, 100>::new_heap();
        let b2 = MF::<f64, 100, 50>::new_heap();
        let b3 = MF::<f64, 100, 2>::new_stack();
        // stack * stack
        let _a = &a1 * &a2;
        let _a = &a1 * a2.clone();
        let _a = a1.clone() * &a2;
        let _a = a1 * a2;
        // stack * heap
        let a1 = MF::<f32, 4, 4>::new_stack();
        let _aa = &a1 * &a3;
        let _aa = &a1 * a3.clone();
        let _aa = a1.clone() * &a3;
        let _aa = a1 * a3;
        // heap * heap
        let _b = &b1 * &b2;
        let _b = &b1 * b2.clone();
        let _b = b1.clone() * &b2;
        let _b = b1 * b2;
        // heap * stack
        let b1 = MF::<f64, 100, 100>::new_heap();
        let _bb = &b1 * &b3;
        let _bb = &b1 * b3.clone();
        let _bb = b1.clone() * &b3;
        let _bb = b1 * b3;
        // ...
        let mut c1 = MF::<f32, 4, 4>::new_stack();
        let c2 = MF::<f32, 4, 2>::new_stack();
        let _c = &mut c1 * c2;
    }
}
