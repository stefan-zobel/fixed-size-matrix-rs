//! Basic matrix arithmetic using const generics

use std::fmt::Debug;
use std::marker::PhantomData;
use std::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};
use std::str::FromStr;

/// All types for which the operators `+`, `+=`, `*`, `*=`,
/// `-` (unary negation), `-` (binary minus) and `-=` are
/// defined and where the output type of `+=`, `*=` and `-=`
/// is `T` again.
pub trait Arithmetic<T>:
    Sized
    + Add<Output = T>
    + AddAssign
    + Mul<Output = T>
    + MulAssign
    + Neg<Output = T>
    + Sub<Output = T>
    + SubAssign
    + FromStr
{
}

impl<
        T: Add<Output = T>
            + AddAssign
            + Mul<Output = T>
            + MulAssign
            + Neg<Output = T>
            + Sub<Output = T>
            + SubAssign
            + FromStr
    > Arithmetic<T> for T
{
}

/// All types which are `Copy` and `Default` in addition
/// to being [Arithmetic](Arithmetic).
pub trait Numeric<T>: Copy + Default + Arithmetic<T> {}
impl<T: Copy + Default + Arithmetic<T>> Numeric<T> for T {}

/// A Matrix allocated on the stack
#[derive(Debug, Clone)]
pub struct SMatrix<T: Numeric<T>, const ROWS: usize, const COLS: usize> {
    #[allow(unused)]
    rows: usize,
    #[allow(unused)]
    cols: usize,
    a: [[T; COLS]; ROWS],
}

/// A Matrix allocated on the heap
#[derive(Debug, Clone)]
pub struct HMatrix<T: Numeric<T>, const ROWS: usize, const COLS: usize> {
    #[allow(unused)]
    rows: usize,
    #[allow(unused)]
    cols: usize,
    a: Box<[[T; COLS]; ROWS]>,
}

// Array access SMatrix
impl<'a, T: Numeric<T>, const ROWS: usize, const COLS: usize> SMatrix<T, ROWS, COLS> {
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
impl<'a, T: Numeric<T>, const ROWS: usize, const COLS: usize> HMatrix<T, ROWS, COLS> {
    #[inline]
    pub(crate) fn array(&'a self) -> &'a [[T; COLS]; ROWS] {
        self.a.as_ref()
    }

    #[inline]
    pub(crate) fn array_mut(&'a mut self) -> &'a mut [[T; COLS]; ROWS] {
        self.a.as_mut()
    }
}

/// `MF` is the `M`atrix `F`actory
pub struct MF<T: Numeric<T>, const ROWS: usize, const COLS: usize> {
    phantom: PhantomData<T>,
}

impl<T: Numeric<T>, const ROWS: usize, const COLS: usize> MF<T, ROWS, COLS> {
    /// Create a new matrix initialized with zeros on the stack.
    #[inline]
    pub fn new_stack() -> SMatrix<T, ROWS, COLS> {
        SMatrix {
            rows: ROWS,
            cols: COLS,
            a: [[T::default(); COLS]; ROWS],
        }
    }

    /// Create a new matrix initialized with zeros on the heap.
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

    /// Create an identity matrix on the stack.
    #[inline]
    pub fn unit_stack() -> SMatrix<T, ROWS, ROWS>
    where
        <T as FromStr>::Err: Debug,
    {
        let mut m = MF::<T, ROWS, ROWS>::new_stack();
        for i in 0..ROWS {
            m[i][i] = "1".parse().expect("unable to parse \"1\"");
        }
        m
    }

    /// Create an identity matrix on the heap.
    #[inline]
    pub fn unit_heap() -> HMatrix<T, ROWS, ROWS>
    where
        <T as FromStr>::Err: Debug,
    {
        let mut m = MF::<T, ROWS, ROWS>::new_heap();
        for i in 0..ROWS {
            m[i][i] = "1".parse().expect("unable to parse \"1\"");
        }
        m
    }

    /// Create a diagonal matrix with initial diagonal value `diag_val` on the stack.
    #[inline]
    pub fn diag_stack(diag_val: T) -> SMatrix<T, ROWS, ROWS>
    where
        <T as FromStr>::Err: Debug,
    {
        let mut m = MF::<T, ROWS, ROWS>::unit_stack();
        m *= diag_val;
        m
    }

    /// Create a diagonal matrix with initial diagonal value `diag_val` on the heap.
    #[inline]
    pub fn diag_heap(diag_val: T) -> HMatrix<T, ROWS, ROWS>
    where
        <T as FromStr>::Err: Debug,
    {
        let mut m = MF::<T, ROWS, ROWS>::unit_heap();
        m *= diag_val;
        m
    }
}

impl<T: Numeric<T>, const ROWS: usize, const COLS: usize> Default for SMatrix<T, ROWS, COLS> {
    fn default() -> Self {
        MF::<T, ROWS, COLS>::new_stack()
    }
}

impl<T: Numeric<T>, const ROWS: usize, const COLS: usize> Default for HMatrix<T, ROWS, COLS> {
    fn default() -> Self {
        MF::<T, ROWS, COLS>::new_heap()
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

    #[test]
    fn test_unit() {
        let a = MF::<f32, 2, 2>::unit_stack();
        let b = MF::<f64, 2, 2>::unit_heap();
        assert_eq!(a[0][0], 1.0);
        assert_eq!(a[1][1], 1.0);
        assert_eq!(a[0][1], 0.0);
        assert_eq!(a[1][0], 0.0);
        assert_eq!(b[0][0], 1.0);
        assert_eq!(b[1][1], 1.0);
        assert_eq!(b[0][1], 0.0);
        assert_eq!(b[1][0], 0.0);
        let a = MF::<i8, 2, 2>::unit_stack();
        let b = MF::<i8, 2, 2>::unit_heap();
        assert_eq!(a[0][0], 1);
        assert_eq!(a[1][1], 1);
        assert_eq!(a[0][1], 0);
        assert_eq!(a[1][0], 0);
        assert_eq!(b[0][0], 1);
        assert_eq!(b[1][1], 1);
        assert_eq!(b[0][1], 0);
        assert_eq!(b[1][0], 0);
        let a = MF::<i128, 2, 2>::unit_stack();
        let b = MF::<i128, 2, 2>::unit_heap();
        assert_eq!(a[0][0], 1);
        assert_eq!(a[1][1], 1);
        assert_eq!(a[0][1], 0);
        assert_eq!(a[1][0], 0);
        assert_eq!(b[0][0], 1);
        assert_eq!(b[1][1], 1);
        assert_eq!(b[0][1], 0);
        assert_eq!(b[1][0], 0);
    }

    #[test]
    fn test_diag() {
        let a = MF::<f32, 2, 2>::diag_stack(5.0);
        let b = MF::<f64, 2, 2>::diag_heap(3.0);
        assert_eq!(a[0][0], 5.0);
        assert_eq!(a[1][1], 5.0);
        assert_eq!(a[0][1], 0.0);
        assert_eq!(a[1][0], 0.0);
        assert_eq!(b[0][0], 3.0);
        assert_eq!(b[1][1], 3.0);
        assert_eq!(b[0][1], 0.0);
        assert_eq!(b[1][0], 0.0);
        let a = MF::<i8, 2, 2>::diag_stack(8);
        let b = MF::<i8, 2, 2>::diag_heap(2);
        assert_eq!(a[0][0], 8);
        assert_eq!(a[1][1], 8);
        assert_eq!(a[0][1], 0);
        assert_eq!(a[1][0], 0);
        assert_eq!(b[0][0], 2);
        assert_eq!(b[1][1], 2);
        assert_eq!(b[0][1], 0);
        assert_eq!(b[1][0], 0);
    }
}
