//! Basic matrix arithmetic using const generics

use std::marker::PhantomData;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

pub trait Numeric:
    Copy + Default + Add + AddAssign + Div + DivAssign + Mul + MulAssign + Neg + Sub + SubAssign
{
}

impl Numeric for f32 {}
impl Numeric for f64 {}
impl Numeric for i8 {}
impl Numeric for i16 {}
impl Numeric for i32 {}
impl Numeric for i64 {}
impl Numeric for i128 {}

/// A Matrix allocated on the stack
#[derive(Debug, Clone)]
pub struct SMatrix<T: Numeric, const ROWS: usize, const COLS: usize> {
    #[allow(unused)]
    rows: usize,
    #[allow(unused)]
    cols: usize,
    a: [[T; COLS]; ROWS],
}

/// A Matrix allocated on the heap
#[derive(Debug, Clone)]
pub struct HMatrix<T: Numeric, const ROWS: usize, const COLS: usize> {
    #[allow(unused)]
    rows: usize,
    #[allow(unused)]
    cols: usize,
    a: Box<[[T; COLS]; ROWS]>,
}

// Array access SMatrix
impl<'a, T: Numeric, const ROWS: usize, const COLS: usize> SMatrix<T, ROWS, COLS> {
    #[inline]
    pub(crate) fn array(&'a self) -> &'a [[T; COLS]; ROWS] {
        &self.a
    }

    #[inline]
    pub(crate) fn array_mut(&'a mut self) -> &'a mut [[T; COLS]; ROWS] {
        &mut self.a
    }
}

// Array access HMatrix
impl<'a, T: Numeric, const ROWS: usize, const COLS: usize> HMatrix<T, ROWS, COLS> {
    #[inline]
    pub(crate) fn array(&'a self) -> &'a [[T; COLS]; ROWS] {
        self.a.as_ref()
    }

    #[inline]
    pub(crate) fn array_mut(&'a mut self) -> &'a mut [[T; COLS]; ROWS] {
        self.a.as_mut()
    }
}

// Matrix Factory
pub struct MF<T: Numeric, const ROWS: usize, const COLS: usize> {
    phantom: PhantomData<T>,
}

impl<T: Numeric, const ROWS: usize, const COLS: usize> MF<T, ROWS, COLS> {
    #[inline]
    pub fn new_stack() -> SMatrix<T, ROWS, COLS> {
        SMatrix {
            rows: ROWS,
            cols: COLS,
            a: [[T::default(); COLS]; ROWS],
        }
    }

    #[inline]
    pub fn new_heap() -> HMatrix<T, ROWS, COLS> {
        let slice = vec![[T::default(); COLS]; ROWS].into_boxed_slice();
        let ptr = Box::into_raw(slice) as *mut [[T; COLS]; ROWS];
        let box_ = unsafe { Box::from_raw(ptr) };
        HMatrix {
            rows: ROWS,
            cols: COLS,
            a: box_,
        }
    }
}

#[cfg(test)]
mod types_tests {
    use super::*;

    #[test]
    fn test_create() {
        let _a = MF::<f32, 4, 4>::new_stack();
        let _b = MF::<f64, 1500, 1500>::new_heap();
    }
}
