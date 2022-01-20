//! Index and IndexMut implementations

use crate::matrix::*;
use std::ops::{Index, IndexMut};

// A1) Index for SMatrix
impl<T: Numeric<T>, const ROWS: usize, const COLS: usize> Index<usize> for SMatrix<T, ROWS, COLS> {
    type Output = [T; COLS];

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.array()[index]
    }
}

// A1-Mut) IndexMut for SMatrix
impl<T: Numeric<T>, const ROWS: usize, const COLS: usize> IndexMut<usize>
    for SMatrix<T, ROWS, COLS>
{
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.array_mut()[index]
    }
}

// A2) Index for &SMatrix (shared)
impl<'a, T: Numeric<T>, const ROWS: usize, const COLS: usize> Index<usize>
    for &'a SMatrix<T, ROWS, COLS>
{
    type Output = [T; COLS];

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.array()[index]
    }
}

// A3) Index for &mut SMatrix (exclusive)
impl<'a, T: Numeric<T>, const ROWS: usize, const COLS: usize> Index<usize>
    for &'a mut SMatrix<T, ROWS, COLS>
{
    type Output = [T; COLS];

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.array()[index]
    }
}

// A3-Mut) IndexMut for &mut SMatrix (exclusive)
impl<'a, T: Numeric<T>, const ROWS: usize, const COLS: usize> IndexMut<usize>
    for &'a mut SMatrix<T, ROWS, COLS>
{
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.array_mut()[index]
    }
}

// B1) Index for HMatrix
impl<T: Numeric<T>, const ROWS: usize, const COLS: usize> Index<usize> for HMatrix<T, ROWS, COLS> {
    type Output = [T; COLS];

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.array()[index]
    }
}

// B1-Mut) IndexMut for HMatrix
impl<T: Numeric<T>, const ROWS: usize, const COLS: usize> IndexMut<usize>
    for HMatrix<T, ROWS, COLS>
{
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.array_mut()[index]
    }
}

// B2) Index for &HMatrix (shared)
impl<'a, T: Numeric<T>, const ROWS: usize, const COLS: usize> Index<usize>
    for &'a HMatrix<T, ROWS, COLS>
{
    type Output = [T; COLS];

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.array()[index]
    }
}

// B3) Index for &mut HMatrix (exclusive)
impl<'a, T: Numeric<T>, const ROWS: usize, const COLS: usize> Index<usize>
    for &'a mut HMatrix<T, ROWS, COLS>
{
    type Output = [T; COLS];

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.array()[index]
    }
}

// B3-Mut) IndexMut for &mut HMatrix (exclusive)
impl<'a, T: Numeric<T>, const ROWS: usize, const COLS: usize> IndexMut<usize>
    for &'a mut HMatrix<T, ROWS, COLS>
{
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.array_mut()[index]
    }
}

#[cfg(test)]
mod index_tests {
    use super::*;

    #[test]
    fn test_one_index() {
        let a = MF::<f64, 10, 10>::new_stack();
        let mut row = a[0];
        println!("row = {:?}", row);
        let a = MF::<f64, 10, 10>::new_heap();
        row = a[0];
        println!("row = {:?}", row);
        // shared ref
        let b1 = &MF::<f64, 10, 10>::new_stack();
        let b2 = &MF::<f64, 10, 10>::new_heap();
        let row1 = b1[0];
        let row2 = b2[0];
        println!("row1 = {:?}", row1);
        println!("row2 = {:?}", row2);
        // mutable ref
        let c1 = &mut MF::<f64, 10, 10>::new_stack();
        let c2 = &mut MF::<f64, 10, 10>::new_heap();
        let row1 = c1[0];
        let row2 = c2[0];
        println!("row1 = {:?}", row1);
        println!("row2 = {:?}", row2);
    }

    #[test]
    fn test_two_indices() {
        let a = MF::<f64, 10, 10>::new_stack();
        let mut elem = a[0][0];
        println!("a[0][0] = {}", elem);
        let a = MF::<f64, 10, 10>::new_heap();
        elem = a[0][0];
        println!("a[0][0] = {}", elem);
        // shared ref
        let b1 = &MF::<f64, 10, 10>::new_stack();
        let b2 = &MF::<f64, 10, 10>::new_heap();
        let elem1 = b1[0][0];
        let elem2 = b2[0][0];
        println!("e1[0][0] = {}", elem1);
        println!("e2[0][0] = {}", elem2);
        // mutable ref
        let c1 = &mut MF::<f64, 10, 10>::new_stack();
        let c2 = &mut MF::<f64, 10, 10>::new_heap();
        let elem1 = c1[0][0];
        let elem2 = c2[0][0];
        println!("e1[0][0] = {}", elem1);
        println!("e2[0][0] = {}", elem2);
    }

    #[test]
    fn test_one_index_assign() {
        let row = [9.5f64; 10];
        let mut a = MF::<f64, 10, 10>::new_stack();
        let mut b = MF::<f64, 10, 10>::new_heap();
        a[0] = row;
        b[0] = row;
        let row1 = a[0];
        let row2 = b[0];
        println!("row after assign = {:?}", row1);
        println!("row after assign = {:?}", row2);
        // mutable ref
        let mut c1 = &mut MF::<f64, 10, 10>::new_stack();
        let mut c2 = &mut MF::<f64, 10, 10>::new_heap();
        c1[0] = row;
        c2[0] = row;
        let row1 = c1[0];
        let row2 = c2[0];
        println!("row1 = {:?}", row1);
        println!("row2 = {:?}", row2);
        // shared ref
        // this doesn't work by design
        /*
        let mut b1 = &MF::<f64, 10, 10>::new_stack();
        let mut b2 = &MF::<f64, 10, 10>::new_heap();
        b1[0] = row;
        b2[0] = row;
        let row1 = b1[0];
        let row2 = b2[0];
        println!("row1 = {:?}", row1);
        println!("row2 = {:?}", row2);
        */
    }

    #[test]
    fn test_two_indices_assign() {
        let mut a = MF::<f64, 10, 10>::new_stack();
        let mut b = MF::<f64, 10, 10>::new_heap();
        a[0][0] = 199.99;
        b[0][0] = 199.99;
        let elem1 = a[0][0];
        let elem2 = a[0][0];
        println!("e1 - a[0][0] = {}", elem1);
        println!("e2 - a[0][0] = {}", elem2);
        // mutable ref
        let mut c1 = &mut MF::<f64, 10, 10>::new_stack();
        let mut c2 = &mut MF::<f64, 10, 10>::new_heap();
        c1[0][0] = 199.99;
        c2[0][0] = 199.99;
        let elem1 = c1[0][0];
        let elem2 = c2[0][0];
        println!("e1[0][0] = {}", elem1);
        println!("e2[0][0] = {}", elem2);
    }
}
