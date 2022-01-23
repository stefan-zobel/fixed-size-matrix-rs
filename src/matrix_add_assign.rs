//! AddAssign implementations

use crate::matrix::*;
use crate::matrix_add::*;
use std::ops::AddAssign;

#[inline]
fn add_ref_s_ref_s<T: Numeric<T>, const ROWS: usize, const COLS: usize>(
    lhs: &SMatrix<T, ROWS, COLS>,
    rhs: &SMatrix<T, ROWS, COLS>,
) -> SMatrix<T, ROWS, COLS> {
    let mut c = MF::<T, ROWS, COLS>::new_stack();
    add(lhs.array(), rhs.array(), c.array_mut());
    c
}

#[inline]
fn add_ref_s_ref_h<T: Numeric<T>, const ROWS: usize, const COLS: usize>(
    lhs: &SMatrix<T, ROWS, COLS>,
    rhs: &HMatrix<T, ROWS, COLS>,
) -> SMatrix<T, ROWS, COLS> {
    let mut c = MF::<T, ROWS, COLS>::new_stack();
    add(lhs.array(), rhs.array(), c.array_mut());
    c
}

#[inline]
fn add_ref_h_ref_s<T: Numeric<T>, const ROWS: usize, const COLS: usize>(
    lhs: &HMatrix<T, ROWS, COLS>,
    rhs: &SMatrix<T, ROWS, COLS>,
) -> HMatrix<T, ROWS, COLS> {
    let mut c = MF::<T, ROWS, COLS>::new_heap();
    add(lhs.array(), rhs.array(), c.array_mut());
    c
}

#[inline]
fn add_ref_h_ref_h<T: Numeric<T>, const ROWS: usize, const COLS: usize>(
    lhs: &HMatrix<T, ROWS, COLS>,
    rhs: &HMatrix<T, ROWS, COLS>,
) -> HMatrix<T, ROWS, COLS> {
    let mut c = MF::<T, ROWS, COLS>::new_heap();
    add(lhs.array(), rhs.array(), c.array_mut());
    c
}

// A1) SMatrix += SMatrix
impl<T: Numeric<T>, const ROWS: usize> AddAssign<Self> for SMatrix<T, ROWS, ROWS> {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        let res = add_ref_s_ref_s(self, &rhs);
        self.array_mut().copy_from_slice(res.array());
    }
}

// A2) SMatrix += &SMatrix
impl<T: Numeric<T>, const ROWS: usize> AddAssign<&Self> for SMatrix<T, ROWS, ROWS> {
    #[inline]
    fn add_assign(&mut self, rhs: &Self) {
        let res = add_ref_s_ref_s(self, rhs);
        self.array_mut().copy_from_slice(res.array());
    }
}

// A3) SMatrix += &mut SMatrix
impl<T: Numeric<T>, const ROWS: usize> AddAssign<&mut Self> for SMatrix<T, ROWS, ROWS> {
    #[inline]
    fn add_assign(&mut self, rhs: &mut Self) {
        let res = add_ref_s_ref_s(self, rhs);
        self.array_mut().copy_from_slice(res.array());
    }
}

// A4) SMatrix += HMatrix
impl<T: Numeric<T>, const ROWS: usize> AddAssign<HMatrix<T, ROWS, ROWS>>
    for SMatrix<T, ROWS, ROWS>
{
    #[inline]
    fn add_assign(&mut self, rhs: HMatrix<T, ROWS, ROWS>) {
        let res = add_ref_s_ref_h(self, &rhs);
        self.array_mut().copy_from_slice(res.array());
    }
}

// A5) SMatrix += &HMatrix
impl<T: Numeric<T>, const ROWS: usize> AddAssign<&HMatrix<T, ROWS, ROWS>>
    for SMatrix<T, ROWS, ROWS>
{
    #[inline]
    fn add_assign(&mut self, rhs: &HMatrix<T, ROWS, ROWS>) {
        let res = add_ref_s_ref_h(self, rhs);
        self.array_mut().copy_from_slice(res.array());
    }
}

// A6) SMatrix += &mut HMatrix
impl<T: Numeric<T>, const ROWS: usize> AddAssign<&mut HMatrix<T, ROWS, ROWS>>
for SMatrix<T, ROWS, ROWS>
{
    #[inline]
    fn add_assign(&mut self, rhs: &mut HMatrix<T, ROWS, ROWS>) {
        let res = add_ref_s_ref_h(self, rhs);
        self.array_mut().copy_from_slice(res.array());
    }
}

// B1) HMatrix += HMatrix
impl<T: Numeric<T>, const ROWS: usize> AddAssign<Self> for HMatrix<T, ROWS, ROWS> {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        let res = add_ref_h_ref_h(self, &rhs);
        self.array_mut().copy_from_slice(res.array());
    }
}

// B2) HMatrix += &HMatrix
impl<T: Numeric<T>, const ROWS: usize> AddAssign<&Self> for HMatrix<T, ROWS, ROWS> {
    #[inline]
    fn add_assign(&mut self, rhs: &Self) {
        let res = add_ref_h_ref_h(self, rhs);
        self.array_mut().copy_from_slice(res.array());
    }
}

// B3) HMatrix += &mut HMatrix
impl<T: Numeric<T>, const ROWS: usize> AddAssign<&mut Self> for HMatrix<T, ROWS, ROWS> {
    #[inline]
    fn add_assign(&mut self, rhs: &mut Self) {
        let res = add_ref_h_ref_h(self, rhs);
        self.array_mut().copy_from_slice(res.array());
    }
}

// B4) HMatrix += SMatrix
impl<T: Numeric<T>, const ROWS: usize> AddAssign<SMatrix<T, ROWS, ROWS>>
    for HMatrix<T, ROWS, ROWS>
{
    #[inline]
    fn add_assign(&mut self, rhs: SMatrix<T, ROWS, ROWS>) {
        let res = add_ref_h_ref_s(self, &rhs);
        self.array_mut().copy_from_slice(res.array());
    }
}

// B5) HMatrix += &SMatrix
impl<T: Numeric<T>, const ROWS: usize> AddAssign<&SMatrix<T, ROWS, ROWS>>
    for HMatrix<T, ROWS, ROWS>
{
    #[inline]
    fn add_assign(&mut self, rhs: &SMatrix<T, ROWS, ROWS>) {
        let res = add_ref_h_ref_s(self, rhs);
        self.array_mut().copy_from_slice(res.array());
    }
}

// B6) HMatrix += &mut SMatrix
impl<T: Numeric<T>, const ROWS: usize> AddAssign<&mut SMatrix<T, ROWS, ROWS>>
for HMatrix<T, ROWS, ROWS>
{
    #[inline]
    fn add_assign(&mut self, rhs: &mut SMatrix<T, ROWS, ROWS>) {
        let res = add_ref_h_ref_s(self, rhs);
        self.array_mut().copy_from_slice(res.array());
    }
}

#[cfg(test)]
mod add_assign_tests {
    use super::*;

    #[test]
    fn test_add_assign() {
        let mut a1 = MF::<f32, 4, 4>::new_stack();
        let a2 = MF::<f32, 4, 4>::new_stack();
        let a3 = MF::<f32, 4, 4>::new_heap();
        let a4 = &MF::<f32, 4, 4>::new_stack();
        let a5 = &MF::<f32, 4, 4>::new_heap();
        let a6 = &mut MF::<f32, 4, 4>::new_stack();
        let a7 = &mut MF::<f32, 4, 4>::new_heap();
        a1 += a2;
        a1 += a3;
        a1 += a4;
        a1 += a5;
        a1 += a6;
        a1 += a7;
        let mut b1 = MF::<f64, 8, 8>::new_heap();
        let b2 = MF::<f64, 8, 8>::new_heap();
        let b3 = MF::<f64, 8, 8>::new_stack();
        let b4 = &MF::<f64, 8, 8>::new_heap();
        let b5 = &MF::<f64, 8, 8>::new_stack();
        let b6 = &mut MF::<f64, 8, 8>::new_heap();
        let b7 = &mut MF::<f64, 8, 8>::new_stack();
        b1 += b2;
        b1 += b3;
        b1 += b4;
        b1 += b5;
        b1 += b6;
        b1 += b7;

        // this doesn't (yet) compile
        //        let c1 = &mut MF::<f32, 4, 4>::new_stack();
        //        let c2 = MF::<f32, 4, 4>::new_stack();
        //        c1 += c2;

        let d1 = &mut MF::<f32, 4, 4>::new_stack();
        let d2 = MF::<f32, 4, 4>::new_stack();
        let _d = d1 + d2;

        let e1 = MF::<f32, 4, 4>::new_stack();
        let e2 = &mut MF::<f32, 4, 4>::new_stack();
        let _e = e1 + e2;

        let f1 = &mut MF::<f32, 4, 4>::new_stack();
        let f2 = &mut MF::<f32, 4, 4>::new_stack();
        let _f = f1 + f2;
    }
}
