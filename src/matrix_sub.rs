//! Sub implementations

use crate::matrix::*;
use std::ops::Sub;

#[inline]
pub(crate) fn sub<T: Numeric<T>, const ROWS: usize, const COLS: usize>(
    a: &[[T; COLS]; ROWS],
    b: &[[T; COLS]; ROWS],
    c: &mut [[T; COLS]; ROWS],
) {
    for row in 0..ROWS {
        for col in 0..COLS {
            c[row][col] = a[row][col] - b[row][col];
        }
    }
}

//
// lhs: SMatrix, rhs: SMatrix
//

// 1) SMatrix - SMatrix
impl<T: Numeric<T>, const ROWS: usize, const COLS: usize> Sub<SMatrix<T, ROWS, COLS>>
    for SMatrix<T, ROWS, COLS>
{
    type Output = SMatrix<T, ROWS, COLS>;

    //noinspection ALL
    #[inline]
    fn sub(self, rhs: SMatrix<T, ROWS, COLS>) -> Self::Output {
        let mut c = MF::<T, ROWS, COLS>::new_stack();
        sub(self.array(), rhs.array(), c.array_mut());
        c
    }
}

// 2) SMatrix - &SMatrix
impl<T: Numeric<T>, const ROWS: usize, const COLS: usize> Sub<&SMatrix<T, ROWS, COLS>>
    for SMatrix<T, ROWS, COLS>
{
    type Output = SMatrix<T, ROWS, COLS>;

    //noinspection ALL
    #[inline]
    fn sub(self, rhs: &SMatrix<T, ROWS, COLS>) -> Self::Output {
        let mut c = MF::<T, ROWS, COLS>::new_stack();
        sub(self.array(), rhs.array(), c.array_mut());
        c
    }
}

// 3) &SMatrix - SMatrix
impl<T: Numeric<T>, const ROWS: usize, const COLS: usize> Sub<SMatrix<T, ROWS, COLS>>
    for &SMatrix<T, ROWS, COLS>
{
    type Output = SMatrix<T, ROWS, COLS>;

    //noinspection ALL
    #[inline]
    fn sub(self, rhs: SMatrix<T, ROWS, COLS>) -> Self::Output {
        let mut c = MF::<T, ROWS, COLS>::new_stack();
        sub(self.array(), rhs.array(), c.array_mut());
        c
    }
}

// 4) &SMatrix - &Matrix
impl<T: Numeric<T>, const ROWS: usize, const COLS: usize> Sub<&SMatrix<T, ROWS, COLS>>
    for &SMatrix<T, ROWS, COLS>
{
    type Output = SMatrix<T, ROWS, COLS>;

    //noinspection ALL
    #[inline]
    fn sub(self, rhs: &SMatrix<T, ROWS, COLS>) -> Self::Output {
        let mut c = MF::<T, ROWS, COLS>::new_stack();
        sub(self.array(), rhs.array(), c.array_mut());
        c
    }
}

// 5) &mut SMatrix - SMatrix
impl<T: Numeric<T>, const ROWS: usize, const COLS: usize> Sub<SMatrix<T, ROWS, COLS>>
    for &mut SMatrix<T, ROWS, COLS>
{
    type Output = SMatrix<T, ROWS, COLS>;

    //noinspection ALL
    #[inline]
    fn sub(self, rhs: SMatrix<T, ROWS, COLS>) -> Self::Output {
        let mut c = MF::<T, ROWS, COLS>::new_stack();
        sub(self.array(), rhs.array(), c.array_mut());
        c
    }
}

// 6) &mut SMatrix - &SMatrix
impl<T: Numeric<T>, const ROWS: usize, const COLS: usize> Sub<&SMatrix<T, ROWS, COLS>>
    for &mut SMatrix<T, ROWS, COLS>
{
    type Output = SMatrix<T, ROWS, COLS>;

    //noinspection ALL
    #[inline]
    fn sub(self, rhs: &SMatrix<T, ROWS, COLS>) -> Self::Output {
        let mut c = MF::<T, ROWS, COLS>::new_stack();
        sub(self.array(), rhs.array(), c.array_mut());
        c
    }
}

//
// lhs: SMatrix, rhs: HMatrix
//

// 7) SMatrix - HMatrix
impl<T: Numeric<T>, const ROWS: usize, const COLS: usize> Sub<HMatrix<T, ROWS, COLS>>
    for SMatrix<T, ROWS, COLS>
{
    type Output = SMatrix<T, ROWS, COLS>;

    //noinspection ALL
    #[inline]
    fn sub(self, rhs: HMatrix<T, ROWS, COLS>) -> Self::Output {
        let mut c = MF::<T, ROWS, COLS>::new_stack();
        sub(self.array(), rhs.array(), c.array_mut());
        c
    }
}

// 8) SMatrix - &HMatrix
impl<T: Numeric<T>, const ROWS: usize, const COLS: usize> Sub<&HMatrix<T, ROWS, COLS>>
    for SMatrix<T, ROWS, COLS>
{
    type Output = SMatrix<T, ROWS, COLS>;

    //noinspection ALL
    #[inline]
    fn sub(self, rhs: &HMatrix<T, ROWS, COLS>) -> Self::Output {
        let mut c = MF::<T, ROWS, COLS>::new_stack();
        sub(self.array(), rhs.array(), c.array_mut());
        c
    }
}

// 9) &SMatrix - HMatrix
impl<T: Numeric<T>, const ROWS: usize, const COLS: usize> Sub<HMatrix<T, ROWS, COLS>>
    for &SMatrix<T, ROWS, COLS>
{
    type Output = SMatrix<T, ROWS, COLS>;

    //noinspection ALL
    #[inline]
    fn sub(self, rhs: HMatrix<T, ROWS, COLS>) -> Self::Output {
        let mut c = MF::<T, ROWS, COLS>::new_stack();
        sub(self.array(), rhs.array(), c.array_mut());
        c
    }
}

// 10) &SMatrix - &HMatrix
impl<T: Numeric<T>, const ROWS: usize, const COLS: usize> Sub<&HMatrix<T, ROWS, COLS>>
    for &SMatrix<T, ROWS, COLS>
{
    type Output = SMatrix<T, ROWS, COLS>;

    //noinspection ALL
    #[inline]
    fn sub(self, rhs: &HMatrix<T, ROWS, COLS>) -> Self::Output {
        let mut c = MF::<T, ROWS, COLS>::new_stack();
        sub(self.array(), rhs.array(), c.array_mut());
        c
    }
}

// 11) &mut SMatrix - HMatrix
impl<T: Numeric<T>, const ROWS: usize, const COLS: usize> Sub<HMatrix<T, ROWS, COLS>>
    for &mut SMatrix<T, ROWS, COLS>
{
    type Output = SMatrix<T, ROWS, COLS>;

    //noinspection ALL
    #[inline]
    fn sub(self, rhs: HMatrix<T, ROWS, COLS>) -> Self::Output {
        let mut c = MF::<T, ROWS, COLS>::new_stack();
        sub(self.array(), rhs.array(), c.array_mut());
        c
    }
}

// 12) &mut SMatrix - &HMatrix
impl<T: Numeric<T>, const ROWS: usize, const COLS: usize> Sub<&HMatrix<T, ROWS, COLS>>
    for &mut SMatrix<T, ROWS, COLS>
{
    type Output = SMatrix<T, ROWS, COLS>;

    //noinspection ALL
    #[inline]
    fn sub(self, rhs: &HMatrix<T, ROWS, COLS>) -> Self::Output {
        let mut c = MF::<T, ROWS, COLS>::new_stack();
        sub(self.array(), rhs.array(), c.array_mut());
        c
    }
}

//
// lhs: HMatrix, rhs: SMatrix
//

// 13) HMatrix - SMatrix
impl<T: Numeric<T>, const ROWS: usize, const COLS: usize> Sub<SMatrix<T, ROWS, COLS>>
    for HMatrix<T, ROWS, COLS>
{
    type Output = HMatrix<T, ROWS, COLS>;

    //noinspection ALL
    #[inline]
    fn sub(self, rhs: SMatrix<T, ROWS, COLS>) -> Self::Output {
        let mut c = MF::<T, ROWS, COLS>::new_heap();
        sub(self.array(), rhs.array(), c.array_mut());
        c
    }
}

// 14) HMatrix - &SMatrix
impl<T: Numeric<T>, const ROWS: usize, const COLS: usize> Sub<&SMatrix<T, ROWS, COLS>>
    for HMatrix<T, ROWS, COLS>
{
    type Output = HMatrix<T, ROWS, COLS>;

    //noinspection ALL
    #[inline]
    fn sub(self, rhs: &SMatrix<T, ROWS, COLS>) -> Self::Output {
        let mut c = MF::<T, ROWS, COLS>::new_heap();
        sub(self.array(), rhs.array(), c.array_mut());
        c
    }
}

// 15) &HMatrix - SMatrix
impl<T: Numeric<T>, const ROWS: usize, const COLS: usize> Sub<SMatrix<T, ROWS, COLS>>
    for &HMatrix<T, ROWS, COLS>
{
    type Output = HMatrix<T, ROWS, COLS>;

    //noinspection ALL
    #[inline]
    fn sub(self, rhs: SMatrix<T, ROWS, COLS>) -> Self::Output {
        let mut c = MF::<T, ROWS, COLS>::new_heap();
        sub(self.array(), rhs.array(), c.array_mut());
        c
    }
}

// 16) &HMatrix - &Matrix
impl<T: Numeric<T>, const ROWS: usize, const COLS: usize> Sub<&SMatrix<T, ROWS, COLS>>
    for &HMatrix<T, ROWS, COLS>
{
    type Output = HMatrix<T, ROWS, COLS>;

    //noinspection ALL
    #[inline]
    fn sub(self, rhs: &SMatrix<T, ROWS, COLS>) -> Self::Output {
        let mut c = MF::<T, ROWS, COLS>::new_heap();
        sub(self.array(), rhs.array(), c.array_mut());
        c
    }
}

// 17) &mut HMatrix - SMatrix
impl<T: Numeric<T>, const ROWS: usize, const COLS: usize> Sub<SMatrix<T, ROWS, COLS>>
    for &mut HMatrix<T, ROWS, COLS>
{
    type Output = HMatrix<T, ROWS, COLS>;

    //noinspection ALL
    #[inline]
    fn sub(self, rhs: SMatrix<T, ROWS, COLS>) -> Self::Output {
        let mut c = MF::<T, ROWS, COLS>::new_heap();
        sub(self.array(), rhs.array(), c.array_mut());
        c
    }
}

// 18) &mut HMatrix - &SMatrix
impl<T: Numeric<T>, const ROWS: usize, const COLS: usize> Sub<&SMatrix<T, ROWS, COLS>>
    for &mut HMatrix<T, ROWS, COLS>
{
    type Output = HMatrix<T, ROWS, COLS>;

    //noinspection ALL
    #[inline]
    fn sub(self, rhs: &SMatrix<T, ROWS, COLS>) -> Self::Output {
        let mut c = MF::<T, ROWS, COLS>::new_heap();
        sub(self.array(), rhs.array(), c.array_mut());
        c
    }
}

//
// lhs: HMatrix, rhs: HMatrix
//

// 19) HMatrix - HMatrix
impl<T: Numeric<T>, const ROWS: usize, const COLS: usize> Sub<HMatrix<T, ROWS, COLS>>
    for HMatrix<T, ROWS, COLS>
{
    type Output = HMatrix<T, ROWS, COLS>;

    //noinspection ALL
    #[inline]
    fn sub(self, rhs: HMatrix<T, ROWS, COLS>) -> Self::Output {
        let mut c = MF::<T, ROWS, COLS>::new_heap();
        sub(self.array(), rhs.array(), c.array_mut());
        c
    }
}

// 20) HMatrix - &HMatrix
impl<T: Numeric<T>, const ROWS: usize, const COLS: usize> Sub<&HMatrix<T, ROWS, COLS>>
    for HMatrix<T, ROWS, COLS>
{
    type Output = HMatrix<T, ROWS, COLS>;

    //noinspection ALL
    #[inline]
    fn sub(self, rhs: &HMatrix<T, ROWS, COLS>) -> Self::Output {
        let mut c = MF::<T, ROWS, COLS>::new_heap();
        sub(self.array(), rhs.array(), c.array_mut());
        c
    }
}

// 21) &HMatrix - HMatrix
impl<T: Numeric<T>, const ROWS: usize, const COLS: usize> Sub<HMatrix<T, ROWS, COLS>>
    for &HMatrix<T, ROWS, COLS>
{
    type Output = HMatrix<T, ROWS, COLS>;

    //noinspection ALL
    #[inline]
    fn sub(self, rhs: HMatrix<T, ROWS, COLS>) -> Self::Output {
        let mut c = MF::<T, ROWS, COLS>::new_heap();
        sub(self.array(), rhs.array(), c.array_mut());
        c
    }
}

// 22) &HMatrix - &HMatrix
impl<T: Numeric<T>, const ROWS: usize, const COLS: usize> Sub<&HMatrix<T, ROWS, COLS>>
    for &HMatrix<T, ROWS, COLS>
{
    type Output = HMatrix<T, ROWS, COLS>;

    //noinspection ALL
    #[inline]
    fn sub(self, rhs: &HMatrix<T, ROWS, COLS>) -> Self::Output {
        let mut c = MF::<T, ROWS, COLS>::new_heap();
        sub(self.array(), rhs.array(), c.array_mut());
        c
    }
}

// 23) &mut HMatrix - HMatrix
impl<T: Numeric<T>, const ROWS: usize, const COLS: usize> Sub<HMatrix<T, ROWS, COLS>>
    for &mut HMatrix<T, ROWS, COLS>
{
    type Output = HMatrix<T, ROWS, COLS>;

    //noinspection ALL
    #[inline]
    fn sub(self, rhs: HMatrix<T, ROWS, COLS>) -> Self::Output {
        let mut c = MF::<T, ROWS, COLS>::new_heap();
        sub(self.array(), rhs.array(), c.array_mut());
        c
    }
}

// 24) &mut HMatrix - &HMatrix
impl<T: Numeric<T>, const ROWS: usize, const COLS: usize> Sub<&HMatrix<T, ROWS, COLS>>
    for &mut HMatrix<T, ROWS, COLS>
{
    type Output = HMatrix<T, ROWS, COLS>;

    //noinspection ALL
    #[inline]
    fn sub(self, rhs: &HMatrix<T, ROWS, COLS>) -> Self::Output {
        let mut c = MF::<T, ROWS, COLS>::new_heap();
        sub(self.array(), rhs.array(), c.array_mut());
        c
    }
}
