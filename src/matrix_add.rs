// Copyright 2022 Stefan Zobel
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! Add implementations

use crate::matrix::*;
use std::ops::Add;

#[inline]
pub(crate) fn add<T: Numeric<T>, const ROWS: usize, const COLS: usize>(
    a: &[[T; COLS]; ROWS],
    b: &[[T; COLS]; ROWS],
    c: &mut [[T; COLS]; ROWS],
) {
    for row in 0..ROWS {
        for col in 0..COLS {
            c[row][col] = a[row][col] + b[row][col];
        }
    }
}

//
// lhs: SMatrix, rhs: SMatrix
//

// 1) SMatrix + SMatrix
impl<T: Numeric<T>, const ROWS: usize, const COLS: usize> Add<SMatrix<T, ROWS, COLS>>
    for SMatrix<T, ROWS, COLS>
{
    type Output = SMatrix<T, ROWS, COLS>;

    //noinspection ALL
    #[inline]
    fn add(self, rhs: SMatrix<T, ROWS, COLS>) -> Self::Output {
        let mut c = MF::<T, ROWS, COLS>::new_stack();
        add(self.array(), rhs.array(), c.array_mut());
        c
    }
}

// 2) SMatrix + &SMatrix
impl<T: Numeric<T>, const ROWS: usize, const COLS: usize> Add<&SMatrix<T, ROWS, COLS>>
    for SMatrix<T, ROWS, COLS>
{
    type Output = SMatrix<T, ROWS, COLS>;

    //noinspection ALL
    #[inline]
    fn add(self, rhs: &SMatrix<T, ROWS, COLS>) -> Self::Output {
        let mut c = MF::<T, ROWS, COLS>::new_stack();
        add(self.array(), rhs.array(), c.array_mut());
        c
    }
}

// 3) &SMatrix + SMatrix
impl<T: Numeric<T>, const ROWS: usize, const COLS: usize> Add<SMatrix<T, ROWS, COLS>>
    for &SMatrix<T, ROWS, COLS>
{
    type Output = SMatrix<T, ROWS, COLS>;

    //noinspection ALL
    #[inline]
    fn add(self, rhs: SMatrix<T, ROWS, COLS>) -> Self::Output {
        let mut c = MF::<T, ROWS, COLS>::new_stack();
        add(self.array(), rhs.array(), c.array_mut());
        c
    }
}

// 4) &SMatrix + &Matrix
impl<T: Numeric<T>, const ROWS: usize, const COLS: usize> Add<&SMatrix<T, ROWS, COLS>>
    for &SMatrix<T, ROWS, COLS>
{
    type Output = SMatrix<T, ROWS, COLS>;

    //noinspection ALL
    #[inline]
    fn add(self, rhs: &SMatrix<T, ROWS, COLS>) -> Self::Output {
        let mut c = MF::<T, ROWS, COLS>::new_stack();
        add(self.array(), rhs.array(), c.array_mut());
        c
    }
}

// 5) &mut SMatrix + SMatrix
impl<T: Numeric<T>, const ROWS: usize, const COLS: usize> Add<SMatrix<T, ROWS, COLS>>
    for &mut SMatrix<T, ROWS, COLS>
{
    type Output = SMatrix<T, ROWS, COLS>;

    //noinspection ALL
    #[inline]
    fn add(self, rhs: SMatrix<T, ROWS, COLS>) -> Self::Output {
        let mut c = MF::<T, ROWS, COLS>::new_stack();
        add(self.array(), rhs.array(), c.array_mut());
        c
    }
}

// 6) &mut SMatrix + &SMatrix
impl<T: Numeric<T>, const ROWS: usize, const COLS: usize> Add<&SMatrix<T, ROWS, COLS>>
    for &mut SMatrix<T, ROWS, COLS>
{
    type Output = SMatrix<T, ROWS, COLS>;

    //noinspection ALL
    #[inline]
    fn add(self, rhs: &SMatrix<T, ROWS, COLS>) -> Self::Output {
        let mut c = MF::<T, ROWS, COLS>::new_stack();
        add(self.array(), rhs.array(), c.array_mut());
        c
    }
}

//
// lhs: SMatrix, rhs: HMatrix
//

// 7) SMatrix + HMatrix
impl<T: Numeric<T>, const ROWS: usize, const COLS: usize> Add<HMatrix<T, ROWS, COLS>>
    for SMatrix<T, ROWS, COLS>
{
    type Output = SMatrix<T, ROWS, COLS>;

    //noinspection ALL
    #[inline]
    fn add(self, rhs: HMatrix<T, ROWS, COLS>) -> Self::Output {
        let mut c = MF::<T, ROWS, COLS>::new_stack();
        add(self.array(), rhs.array(), c.array_mut());
        c
    }
}

// 8) SMatrix + &HMatrix
impl<T: Numeric<T>, const ROWS: usize, const COLS: usize> Add<&HMatrix<T, ROWS, COLS>>
    for SMatrix<T, ROWS, COLS>
{
    type Output = SMatrix<T, ROWS, COLS>;

    //noinspection ALL
    #[inline]
    fn add(self, rhs: &HMatrix<T, ROWS, COLS>) -> Self::Output {
        let mut c = MF::<T, ROWS, COLS>::new_stack();
        add(self.array(), rhs.array(), c.array_mut());
        c
    }
}

// 9) &SMatrix + HMatrix
impl<T: Numeric<T>, const ROWS: usize, const COLS: usize> Add<HMatrix<T, ROWS, COLS>>
    for &SMatrix<T, ROWS, COLS>
{
    type Output = SMatrix<T, ROWS, COLS>;

    //noinspection ALL
    #[inline]
    fn add(self, rhs: HMatrix<T, ROWS, COLS>) -> Self::Output {
        let mut c = MF::<T, ROWS, COLS>::new_stack();
        add(self.array(), rhs.array(), c.array_mut());
        c
    }
}

// 10) &SMatrix + &HMatrix
impl<T: Numeric<T>, const ROWS: usize, const COLS: usize> Add<&HMatrix<T, ROWS, COLS>>
    for &SMatrix<T, ROWS, COLS>
{
    type Output = SMatrix<T, ROWS, COLS>;

    //noinspection ALL
    #[inline]
    fn add(self, rhs: &HMatrix<T, ROWS, COLS>) -> Self::Output {
        let mut c = MF::<T, ROWS, COLS>::new_stack();
        add(self.array(), rhs.array(), c.array_mut());
        c
    }
}

// 11) &mut SMatrix + HMatrix
impl<T: Numeric<T>, const ROWS: usize, const COLS: usize> Add<HMatrix<T, ROWS, COLS>>
    for &mut SMatrix<T, ROWS, COLS>
{
    type Output = SMatrix<T, ROWS, COLS>;

    //noinspection ALL
    #[inline]
    fn add(self, rhs: HMatrix<T, ROWS, COLS>) -> Self::Output {
        let mut c = MF::<T, ROWS, COLS>::new_stack();
        add(self.array(), rhs.array(), c.array_mut());
        c
    }
}

// 12) &mut SMatrix + &HMatrix
impl<T: Numeric<T>, const ROWS: usize, const COLS: usize> Add<&HMatrix<T, ROWS, COLS>>
    for &mut SMatrix<T, ROWS, COLS>
{
    type Output = SMatrix<T, ROWS, COLS>;

    //noinspection ALL
    #[inline]
    fn add(self, rhs: &HMatrix<T, ROWS, COLS>) -> Self::Output {
        let mut c = MF::<T, ROWS, COLS>::new_stack();
        add(self.array(), rhs.array(), c.array_mut());
        c
    }
}

//
// lhs: HMatrix, rhs: SMatrix
//

// 13) HMatrix + SMatrix
impl<T: Numeric<T>, const ROWS: usize, const COLS: usize> Add<SMatrix<T, ROWS, COLS>>
    for HMatrix<T, ROWS, COLS>
{
    type Output = HMatrix<T, ROWS, COLS>;

    //noinspection ALL
    #[inline]
    fn add(self, rhs: SMatrix<T, ROWS, COLS>) -> Self::Output {
        let mut c = MF::<T, ROWS, COLS>::new_heap();
        add(self.array(), rhs.array(), c.array_mut());
        c
    }
}

// 14) HMatrix + &SMatrix
impl<T: Numeric<T>, const ROWS: usize, const COLS: usize> Add<&SMatrix<T, ROWS, COLS>>
    for HMatrix<T, ROWS, COLS>
{
    type Output = HMatrix<T, ROWS, COLS>;

    //noinspection ALL
    #[inline]
    fn add(self, rhs: &SMatrix<T, ROWS, COLS>) -> Self::Output {
        let mut c = MF::<T, ROWS, COLS>::new_heap();
        add(self.array(), rhs.array(), c.array_mut());
        c
    }
}

// 15) &HMatrix + SMatrix
impl<T: Numeric<T>, const ROWS: usize, const COLS: usize> Add<SMatrix<T, ROWS, COLS>>
    for &HMatrix<T, ROWS, COLS>
{
    type Output = HMatrix<T, ROWS, COLS>;

    //noinspection ALL
    #[inline]
    fn add(self, rhs: SMatrix<T, ROWS, COLS>) -> Self::Output {
        let mut c = MF::<T, ROWS, COLS>::new_heap();
        add(self.array(), rhs.array(), c.array_mut());
        c
    }
}

// 16) &HMatrix + &SMatrix
impl<T: Numeric<T>, const ROWS: usize, const COLS: usize> Add<&SMatrix<T, ROWS, COLS>>
    for &HMatrix<T, ROWS, COLS>
{
    type Output = HMatrix<T, ROWS, COLS>;

    //noinspection ALL
    #[inline]
    fn add(self, rhs: &SMatrix<T, ROWS, COLS>) -> Self::Output {
        let mut c = MF::<T, ROWS, COLS>::new_heap();
        add(self.array(), rhs.array(), c.array_mut());
        c
    }
}

// 17) &mut HMatrix + SMatrix
impl<T: Numeric<T>, const ROWS: usize, const COLS: usize> Add<SMatrix<T, ROWS, COLS>>
    for &mut HMatrix<T, ROWS, COLS>
{
    type Output = HMatrix<T, ROWS, COLS>;

    //noinspection ALL
    #[inline]
    fn add(self, rhs: SMatrix<T, ROWS, COLS>) -> Self::Output {
        let mut c = MF::<T, ROWS, COLS>::new_heap();
        add(self.array(), rhs.array(), c.array_mut());
        c
    }
}

// 18) &mut HMatrix + &SMatrix
impl<T: Numeric<T>, const ROWS: usize, const COLS: usize> Add<&SMatrix<T, ROWS, COLS>>
    for &mut HMatrix<T, ROWS, COLS>
{
    type Output = HMatrix<T, ROWS, COLS>;

    //noinspection ALL
    #[inline]
    fn add(self, rhs: &SMatrix<T, ROWS, COLS>) -> Self::Output {
        let mut c = MF::<T, ROWS, COLS>::new_heap();
        add(self.array(), rhs.array(), c.array_mut());
        c
    }
}

//
// lhs: HMatrix, rhs: HMatrix
//

// 19) HMatrix + HMatrix
impl<T: Numeric<T>, const ROWS: usize, const COLS: usize> Add<HMatrix<T, ROWS, COLS>>
    for HMatrix<T, ROWS, COLS>
{
    type Output = HMatrix<T, ROWS, COLS>;

    //noinspection ALL
    #[inline]
    fn add(self, rhs: HMatrix<T, ROWS, COLS>) -> Self::Output {
        let mut c = MF::<T, ROWS, COLS>::new_heap();
        add(self.array(), rhs.array(), c.array_mut());
        c
    }
}

// 20) HMatrix + &HMatrix
impl<T: Numeric<T>, const ROWS: usize, const COLS: usize> Add<&HMatrix<T, ROWS, COLS>>
    for HMatrix<T, ROWS, COLS>
{
    type Output = HMatrix<T, ROWS, COLS>;

    //noinspection ALL
    #[inline]
    fn add(self, rhs: &HMatrix<T, ROWS, COLS>) -> Self::Output {
        let mut c = MF::<T, ROWS, COLS>::new_heap();
        add(self.array(), rhs.array(), c.array_mut());
        c
    }
}

// 21) &HMatrix + HMatrix
impl<T: Numeric<T>, const ROWS: usize, const COLS: usize> Add<HMatrix<T, ROWS, COLS>>
    for &HMatrix<T, ROWS, COLS>
{
    type Output = HMatrix<T, ROWS, COLS>;

    //noinspection ALL
    #[inline]
    fn add(self, rhs: HMatrix<T, ROWS, COLS>) -> Self::Output {
        let mut c = MF::<T, ROWS, COLS>::new_heap();
        add(self.array(), rhs.array(), c.array_mut());
        c
    }
}

// 22) &HMatrix + &HMatrix
impl<T: Numeric<T>, const ROWS: usize, const COLS: usize> Add<&HMatrix<T, ROWS, COLS>>
    for &HMatrix<T, ROWS, COLS>
{
    type Output = HMatrix<T, ROWS, COLS>;

    //noinspection ALL
    #[inline]
    fn add(self, rhs: &HMatrix<T, ROWS, COLS>) -> Self::Output {
        let mut c = MF::<T, ROWS, COLS>::new_heap();
        add(self.array(), rhs.array(), c.array_mut());
        c
    }
}

// 23) &mut HMatrix + HMatrix
impl<T: Numeric<T>, const ROWS: usize, const COLS: usize> Add<HMatrix<T, ROWS, COLS>>
    for &mut HMatrix<T, ROWS, COLS>
{
    type Output = HMatrix<T, ROWS, COLS>;

    //noinspection ALL
    #[inline]
    fn add(self, rhs: HMatrix<T, ROWS, COLS>) -> Self::Output {
        let mut c = MF::<T, ROWS, COLS>::new_heap();
        add(self.array(), rhs.array(), c.array_mut());
        c
    }
}

// 24) &mut HMatrix + &HMatrix
impl<T: Numeric<T>, const ROWS: usize, const COLS: usize> Add<&HMatrix<T, ROWS, COLS>>
    for &mut HMatrix<T, ROWS, COLS>
{
    type Output = HMatrix<T, ROWS, COLS>;

    //noinspection ALL
    #[inline]
    fn add(self, rhs: &HMatrix<T, ROWS, COLS>) -> Self::Output {
        let mut c = MF::<T, ROWS, COLS>::new_heap();
        add(self.array(), rhs.array(), c.array_mut());
        c
    }
}

//
// lhs: value, rhs: &mut
//

// 25) SMatrix + &mut SMatrix
impl<T: Numeric<T>, const ROWS: usize, const COLS: usize> Add<&mut SMatrix<T, ROWS, COLS>>
    for SMatrix<T, ROWS, COLS>
{
    type Output = SMatrix<T, ROWS, COLS>;

    //noinspection ALL
    #[inline]
    fn add(self, rhs: &mut SMatrix<T, ROWS, COLS>) -> Self::Output {
        let mut c = MF::<T, ROWS, COLS>::new_stack();
        add(self.array(), rhs.array(), c.array_mut());
        c
    }
}

// 26) SMatrix + &mut HMatrix
impl<T: Numeric<T>, const ROWS: usize, const COLS: usize> Add<&mut HMatrix<T, ROWS, COLS>>
    for SMatrix<T, ROWS, COLS>
{
    type Output = SMatrix<T, ROWS, COLS>;

    //noinspection ALL
    #[inline]
    fn add(self, rhs: &mut HMatrix<T, ROWS, COLS>) -> Self::Output {
        let mut c = MF::<T, ROWS, COLS>::new_stack();
        add(self.array(), rhs.array(), c.array_mut());
        c
    }
}

// 27) HMatrix + &mut HMatrix
impl<T: Numeric<T>, const ROWS: usize, const COLS: usize> Add<&mut HMatrix<T, ROWS, COLS>>
    for HMatrix<T, ROWS, COLS>
{
    type Output = HMatrix<T, ROWS, COLS>;

    //noinspection ALL
    #[inline]
    fn add(self, rhs: &mut HMatrix<T, ROWS, COLS>) -> Self::Output {
        let mut c = MF::<T, ROWS, COLS>::new_heap();
        add(self.array(), rhs.array(), c.array_mut());
        c
    }
}

// 28) HMatrix + &mut SMatrix
impl<T: Numeric<T>, const ROWS: usize, const COLS: usize> Add<&mut SMatrix<T, ROWS, COLS>>
    for HMatrix<T, ROWS, COLS>
{
    type Output = HMatrix<T, ROWS, COLS>;

    //noinspection ALL
    #[inline]
    fn add(self, rhs: &mut SMatrix<T, ROWS, COLS>) -> Self::Output {
        let mut c = MF::<T, ROWS, COLS>::new_heap();
        add(self.array(), rhs.array(), c.array_mut());
        c
    }
}

//
// lhs: &mut, rhs: &mut
//

// 29) &mut SMatrix + &mut SMatrix
impl<T: Numeric<T>, const ROWS: usize, const COLS: usize> Add<&mut SMatrix<T, ROWS, COLS>>
    for &mut SMatrix<T, ROWS, COLS>
{
    type Output = SMatrix<T, ROWS, COLS>;

    //noinspection ALL
    #[inline]
    fn add(self, rhs: &mut SMatrix<T, ROWS, COLS>) -> Self::Output {
        let mut c = MF::<T, ROWS, COLS>::new_stack();
        add(self.array(), rhs.array(), c.array_mut());
        c
    }
}

// 30) &mut SMatrix + &mut HMatrix
impl<T: Numeric<T>, const ROWS: usize, const COLS: usize> Add<&mut HMatrix<T, ROWS, COLS>>
    for &mut SMatrix<T, ROWS, COLS>
{
    type Output = SMatrix<T, ROWS, COLS>;

    //noinspection ALL
    #[inline]
    fn add(self, rhs: &mut HMatrix<T, ROWS, COLS>) -> Self::Output {
        let mut c = MF::<T, ROWS, COLS>::new_stack();
        add(self.array(), rhs.array(), c.array_mut());
        c
    }
}

// 31) &mut HMatrix + &mut HMatrix
impl<T: Numeric<T>, const ROWS: usize, const COLS: usize> Add<&mut HMatrix<T, ROWS, COLS>>
    for &mut HMatrix<T, ROWS, COLS>
{
    type Output = HMatrix<T, ROWS, COLS>;

    //noinspection ALL
    #[inline]
    fn add(self, rhs: &mut HMatrix<T, ROWS, COLS>) -> Self::Output {
        let mut c = MF::<T, ROWS, COLS>::new_heap();
        add(self.array(), rhs.array(), c.array_mut());
        c
    }
}

// 32) &mut HMatrix + &mut SMatrix
impl<T: Numeric<T>, const ROWS: usize, const COLS: usize> Add<&mut SMatrix<T, ROWS, COLS>>
    for &mut HMatrix<T, ROWS, COLS>
{
    type Output = HMatrix<T, ROWS, COLS>;

    //noinspection ALL
    #[inline]
    fn add(self, rhs: &mut SMatrix<T, ROWS, COLS>) -> Self::Output {
        let mut c = MF::<T, ROWS, COLS>::new_heap();
        add(self.array(), rhs.array(), c.array_mut());
        c
    }
}

#[cfg(test)]
mod add_tests {
    use super::*;

    #[test]
    fn test_add() {
        let a1 = MF::<f32, 4, 4>::new_stack();
        let a2 = MF::<f32, 4, 4>::new_stack();
        let a3 = MF::<f32, 4, 4>::new_heap();
        let b1 = MF::<f64, 100, 100>::new_heap();
        let b2 = MF::<f64, 100, 100>::new_heap();
        let b3 = MF::<f64, 100, 100>::new_stack();
        // stack * stack
        let _a = &a1 + &a2;
        let _a = &a1 + a2.clone();
        let _a = a1.clone() + &a2;
        let _a = a1 + a2;
        // stack * heap
        let a1 = MF::<f32, 4, 4>::new_stack();
        let _aa = &a1 + &a3;
        let _aa = &a1 + a3.clone();
        let _aa = a1.clone() + &a3;
        let _aa = a1 + a3;
        // heap * heap
        let _b = &b1 + &b2;
        let _b = &b1 + b2.clone();
        let _b = b1.clone() + &b2;
        let _b = b1 + b2;
        // heap * stack
        let b1 = MF::<f64, 100, 100>::new_heap();
        let _bb = &b1 + &b3;
        let _bb = &b1 + b3.clone();
        let _bb = b1.clone() + &b3;
        let _bb = b1 + b3;
        // &mut + value
        let mut c1 = MF::<f32, 4, 4>::new_stack();
        let c2 = MF::<f32, 4, 4>::new_stack();
        let _c = &mut c1 + c2;
    }
}
