//! SubAssign implementations

use crate::matrix::*;
use crate::matrix_sub::*;
use std::ops::SubAssign;

#[inline]
fn sub_ref_s_ref_s<T: Numeric<T>, const ROWS: usize, const COLS: usize>(
    lhs: &SMatrix<T, ROWS, COLS>,
    rhs: &SMatrix<T, ROWS, COLS>,
) -> SMatrix<T, ROWS, COLS> {
    let mut c = MF::<T, ROWS, COLS>::new_stack();
    sub(lhs.array(), rhs.array(), c.array_mut());
    c
}

#[inline]
fn sub_ref_s_ref_h<T: Numeric<T>, const ROWS: usize, const COLS: usize>(
    lhs: &SMatrix<T, ROWS, COLS>,
    rhs: &HMatrix<T, ROWS, COLS>,
) -> SMatrix<T, ROWS, COLS> {
    let mut c = MF::<T, ROWS, COLS>::new_stack();
    sub(lhs.array(), rhs.array(), c.array_mut());
    c
}

#[inline]
fn sub_ref_h_ref_s<T: Numeric<T>, const ROWS: usize, const COLS: usize>(
    lhs: &HMatrix<T, ROWS, COLS>,
    rhs: &SMatrix<T, ROWS, COLS>,
) -> HMatrix<T, ROWS, COLS> {
    let mut c = MF::<T, ROWS, COLS>::new_heap();
    sub(lhs.array(), rhs.array(), c.array_mut());
    c
}

#[inline]
fn sub_ref_h_ref_h<T: Numeric<T>, const ROWS: usize, const COLS: usize>(
    lhs: &HMatrix<T, ROWS, COLS>,
    rhs: &HMatrix<T, ROWS, COLS>,
) -> HMatrix<T, ROWS, COLS> {
    let mut c = MF::<T, ROWS, COLS>::new_heap();
    sub(lhs.array(), rhs.array(), c.array_mut());
    c
}

// A1) SMatrix -= SMatrix
impl<T: Numeric<T>, const ROWS: usize> SubAssign<Self> for SMatrix<T, ROWS, ROWS> {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        let res = sub_ref_s_ref_s(self, &rhs);
        self.array_mut().copy_from_slice(res.array());
    }
}

// A2) SMatrix -= &SMatrix
impl<T: Numeric<T>, const ROWS: usize> SubAssign<&Self> for SMatrix<T, ROWS, ROWS> {
    #[inline]
    fn sub_assign(&mut self, rhs: &Self) {
        let res = sub_ref_s_ref_s(self, rhs);
        self.array_mut().copy_from_slice(res.array());
    }
}

// A3) SMatrix -= HMatrix
impl<T: Numeric<T>, const ROWS: usize> SubAssign<HMatrix<T, ROWS, ROWS>>
    for SMatrix<T, ROWS, ROWS>
{
    #[inline]
    fn sub_assign(&mut self, rhs: HMatrix<T, ROWS, ROWS>) {
        let res = sub_ref_s_ref_h(self, &rhs);
        self.array_mut().copy_from_slice(res.array());
    }
}

// A4) SMatrix -= &HMatrix
impl<T: Numeric<T>, const ROWS: usize> SubAssign<&HMatrix<T, ROWS, ROWS>>
    for SMatrix<T, ROWS, ROWS>
{
    #[inline]
    fn sub_assign(&mut self, rhs: &HMatrix<T, ROWS, ROWS>) {
        let res = sub_ref_s_ref_h(self, rhs);
        self.array_mut().copy_from_slice(res.array());
    }
}

// B1) HMatrix -= HMatrix
impl<T: Numeric<T>, const ROWS: usize> SubAssign<Self> for HMatrix<T, ROWS, ROWS> {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        let res = sub_ref_h_ref_h(self, &rhs);
        self.array_mut().copy_from_slice(res.array());
    }
}

// B2) HMatrix -= &HMatrix
impl<T: Numeric<T>, const ROWS: usize> SubAssign<&Self> for HMatrix<T, ROWS, ROWS> {
    #[inline]
    fn sub_assign(&mut self, rhs: &Self) {
        let res = sub_ref_h_ref_h(self, rhs);
        self.array_mut().copy_from_slice(res.array());
    }
}

// B3) HMatrix -= SMatrix
impl<T: Numeric<T>, const ROWS: usize> SubAssign<SMatrix<T, ROWS, ROWS>>
    for HMatrix<T, ROWS, ROWS>
{
    #[inline]
    fn sub_assign(&mut self, rhs: SMatrix<T, ROWS, ROWS>) {
        let res = sub_ref_h_ref_s(self, &rhs);
        self.array_mut().copy_from_slice(res.array());
    }
}

// B4) HMatrix -= &SMatrix
impl<T: Numeric<T>, const ROWS: usize> SubAssign<&SMatrix<T, ROWS, ROWS>>
    for HMatrix<T, ROWS, ROWS>
{
    #[inline]
    fn sub_assign(&mut self, rhs: &SMatrix<T, ROWS, ROWS>) {
        let res = sub_ref_h_ref_s(self, rhs);
        self.array_mut().copy_from_slice(res.array());
    }
}
