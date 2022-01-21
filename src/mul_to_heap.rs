//! SMatrix * SMatrix -> HMatrix

use crate::matrix::*;
use crate::matrix_mul::multiply;

impl<T: Numeric<T>, const ROWS_LEFT: usize, const COLS_LEFT: usize>
    SMatrix<T, ROWS_LEFT, COLS_LEFT>
{
    #[inline]
    pub fn mul_val_to_heap<const COLS_RIGHT: usize>(
        &self,
        rhs: SMatrix<T, COLS_LEFT, COLS_RIGHT>,
    ) -> HMatrix<T, ROWS_LEFT, COLS_RIGHT> {
        let mut c = MF::<T, ROWS_LEFT, COLS_RIGHT>::new_heap();
        multiply(self.array(), rhs.array(), c.array_mut());
        c
    }

    #[inline]
    pub fn mul_ref_to_heap<const COLS_RIGHT: usize>(
        &self,
        rhs: &SMatrix<T, COLS_LEFT, COLS_RIGHT>,
    ) -> HMatrix<T, ROWS_LEFT, COLS_RIGHT> {
        let mut c = MF::<T, ROWS_LEFT, COLS_RIGHT>::new_heap();
        multiply(self.array(), rhs.array(), c.array_mut());
        c
    }
}

#[cfg(test)]
mod mul_to_heap_tests {
    use super::*;

    #[test]
    fn test_mul_to_heap() {
        let a = MF::<f64, 100, 1>::new_stack();
        let by_value = MF::<f64, 1, 100>::new_stack();
        let by_ref = &MF::<f64, 1, 100>::new_stack();
        let by_mut_ref = &mut MF::<f64, 1, 100>::new_stack();

        let _c = a.mul_val_to_heap(by_value);
        let _d = a.mul_ref_to_heap(by_ref);
        let _e = a.mul_ref_to_heap(by_mut_ref);
    }
}
