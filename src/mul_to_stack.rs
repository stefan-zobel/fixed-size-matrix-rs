//! SMatrix * HMatrix -> SMatrix
//!
//! HMatrix * HMatrix -> SMatrix
//!
//! HMatrix * SMatrix -> SMatrix

use crate::matrix::*;
use crate::matrix_mul::multiply;

impl<T: Numeric<T>, const ROWS_LEFT: usize, const COLS_LEFT: usize>
    SMatrix<T, ROWS_LEFT, COLS_LEFT>
{
    //noinspection ALL
    /// Multiply a stack matrix with a heap matrix and allocate the result on the stack.
    #[inline]
    pub fn mul_val_to_stack<const COLS_RIGHT: usize>(
        &self,
        rhs: HMatrix<T, COLS_LEFT, COLS_RIGHT>,
    ) -> SMatrix<T, ROWS_LEFT, COLS_RIGHT> {
        let mut c = MF::<T, ROWS_LEFT, COLS_RIGHT>::new_stack();
        multiply(self.array(), rhs.array(), c.array_mut());
        c
    }

    //noinspection ALL
    /// Multiply a stack matrix with a heap matrix and allocate the result on the stack.
    #[inline]
    pub fn mul_ref_to_stack<const COLS_RIGHT: usize>(
        &self,
        rhs: &HMatrix<T, COLS_LEFT, COLS_RIGHT>,
    ) -> SMatrix<T, ROWS_LEFT, COLS_RIGHT> {
        let mut c = MF::<T, ROWS_LEFT, COLS_RIGHT>::new_stack();
        multiply(self.array(), rhs.array(), c.array_mut());
        c
    }
}

impl<T: Numeric<T>, const ROWS_LEFT: usize, const COLS_LEFT: usize>
    HMatrix<T, ROWS_LEFT, COLS_LEFT>
{
    //noinspection ALL
    /// Multiply a heap matrix with a heap matrix and allocate the result on the stack.
    #[inline]
    pub fn mul_heapval_to_stack<const COLS_RIGHT: usize>(
        &self,
        rhs: HMatrix<T, COLS_LEFT, COLS_RIGHT>,
    ) -> SMatrix<T, ROWS_LEFT, COLS_RIGHT> {
        let mut c = MF::<T, ROWS_LEFT, COLS_RIGHT>::new_stack();
        multiply(self.array(), rhs.array(), c.array_mut());
        c
    }

    //noinspection ALL
    /// Multiply a heap matrix with a heap matrix and allocate the result on the stack.
    #[inline]
    pub fn mul_heapref_to_stack<const COLS_RIGHT: usize>(
        &self,
        rhs: &HMatrix<T, COLS_LEFT, COLS_RIGHT>,
    ) -> SMatrix<T, ROWS_LEFT, COLS_RIGHT> {
        let mut c = MF::<T, ROWS_LEFT, COLS_RIGHT>::new_stack();
        multiply(self.array(), rhs.array(), c.array_mut());
        c
    }

    /// Multiply a heap matrix with a stack matrix and allocate the result on the stack.
    #[inline]
    pub fn mul_stackval_to_stack<const COLS_RIGHT: usize>(
        &self,
        rhs: SMatrix<T, COLS_LEFT, COLS_RIGHT>,
    ) -> SMatrix<T, ROWS_LEFT, COLS_RIGHT> {
        let mut c = MF::<T, ROWS_LEFT, COLS_RIGHT>::new_stack();
        multiply(self.array(), rhs.array(), c.array_mut());
        c
    }

    /// Multiply a heap matrix with a stack matrix and allocate the result on the stack.
    #[inline]
    pub fn mul_stackref_to_stack<const COLS_RIGHT: usize>(
        &self,
        rhs: &SMatrix<T, COLS_LEFT, COLS_RIGHT>,
    ) -> SMatrix<T, ROWS_LEFT, COLS_RIGHT> {
        let mut c = MF::<T, ROWS_LEFT, COLS_RIGHT>::new_stack();
        multiply(self.array(), rhs.array(), c.array_mut());
        c
    }
}

#[cfg(test)]
mod mul_to_heap_tests {
    use super::*;

    #[test]
    fn test_smatrix_mul_to_stack() {
        let a = MF::<f64, 1, 100>::new_stack();
        let by_value = MF::<f64, 100, 1>::new_heap();
        let by_ref = &MF::<f64, 100, 1>::new_heap();
        let by_mut_ref = &mut MF::<f64, 100, 1>::new_heap();

        let _c = a.mul_val_to_stack(by_value);
        let _d = a.mul_ref_to_stack(by_ref);
        let _e = a.mul_ref_to_stack(by_mut_ref);
    }

    #[test]
    fn test_hmatrix_mul_hmatrix_to_stack() {
        let a = MF::<f64, 1, 100>::new_heap();
        let by_value = MF::<f64, 100, 1>::new_heap();
        let by_ref = &MF::<f64, 100, 1>::new_heap();
        let by_mut_ref = &mut MF::<f64, 100, 1>::new_heap();

        let _c = a.mul_heapval_to_stack(by_value);
        let _d = a.mul_heapref_to_stack(by_ref);
        let _e = a.mul_heapref_to_stack(by_mut_ref);
    }

    #[test]
    fn test_hmatrix_mul_smatrix_to_stack() {
        let a = MF::<f64, 1, 100>::new_heap();
        let by_value = MF::<f64, 100, 1>::new_stack();
        let by_ref = &MF::<f64, 100, 1>::new_stack();
        let by_mut_ref = &mut MF::<f64, 100, 1>::new_stack();

        let _c = a.mul_stackval_to_stack(by_value);
        let _d = a.mul_stackref_to_stack(by_ref);
        let _e = a.mul_stackref_to_stack(by_mut_ref);
    }
}
