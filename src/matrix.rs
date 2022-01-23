// Copyright 2022 Stefan Zobel
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! Basic arithmetic for compile-time-sized matrices either allocated
//! on the stack ([SMatrix](SMatrix)) or on the heap ([HMatrix](HMatrix))
//! using const generics. Both matrix types are fully interoperable with
//! each other.

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
{
}

impl<
        T: Add<Output = T>
            + AddAssign
            + Mul<Output = T>
            + MulAssign
            + Neg<Output = T>
            + Sub<Output = T>
            + SubAssign,
    > Arithmetic<T> for T
{
}

/// All types which are `Copy` and `Default` in addition
/// to being [Arithmetic](Arithmetic).
pub trait Numeric<T>: Copy + Default + Arithmetic<T> {}
impl<T: Copy + Default + Arithmetic<T>> Numeric<T> for T {}

/// A matrix which is allocated on the stack.
/// An `SMatrix` is itself [Numeric](Numeric), so that its
/// elements can also be other stack-allocated matrices which
/// theoretically could contain `SMatrices` themselves up to
/// arbitrarily deep finite nesting levels..
#[derive(Debug, Copy, Clone)]
pub struct SMatrix<T: Numeric<T>, const ROWS: usize, const COLS: usize> {
    a: [[T; COLS]; ROWS],
}

/// A matrix which is allocated on the heap.
#[derive(Debug, Clone)]
pub struct HMatrix<T: Numeric<T>, const ROWS: usize, const COLS: usize> {
    a: Box<[[T; COLS]; ROWS]>,
}

// new(), to_heap() and crate-internal array access for SMatrix
impl<'a, T: Numeric<T>, const ROWS: usize, const COLS: usize> SMatrix<T, ROWS, COLS> {
    /// Creates a new stack-allocated matrix from the given initial values.
    #[inline]
    pub fn new(array: [[T; COLS]; ROWS]) -> Self {
        SMatrix { a: array }
    }

    /// Creates a stack-allocated transpose of this matrix.
    #[inline]
    pub fn trans(&self) -> SMatrix<T, COLS, ROWS> {
        let mut transposed = MF::<T, COLS, ROWS>::new_stack();
        copy_trans(self.array(), transposed.array_mut());
        transposed
    }

    /// Creates a heap-allocated copy of this stack-allocated matrix.
    #[inline]
    pub fn to_heap(&self) -> HMatrix<T, ROWS, COLS> {
        let mut heap_copy = MF::<T, ROWS, COLS>::new_heap();
        heap_copy.array_mut().copy_from_slice(self.array());
        heap_copy
    }

    #[inline]
    pub(crate) fn array(&'a self) -> &'a [[T; COLS]; ROWS] {
        &self.a
    }

    #[inline]
    pub(crate) fn array_mut(&'a mut self) -> &'a mut [[T; COLS]; ROWS] {
        &mut self.a
    }
}

// new(), to_stack() and crate-internal array access for HMatrix
impl<'a, T: Numeric<T>, const ROWS: usize, const COLS: usize> HMatrix<T, ROWS, COLS> {
    /// Creates a new stack-allocated matrix from the given initial values.
    #[inline]
    pub fn new(array: [[T; COLS]; ROWS]) -> Self {
        let mut heap = MF::<T, ROWS, COLS>::new_heap();
        heap.array_mut().copy_from_slice(&array);
        heap
    }

    /// Creates a heap-allocated transpose of this matrix.
    #[inline]
    pub fn trans(&self) -> HMatrix<T, COLS, ROWS> {
        let mut transposed = MF::<T, COLS, ROWS>::new_heap();
        copy_trans(self.array(), transposed.array_mut());
        transposed
    }

    /// Creates a stack-allocated copy of this heap-allocated matrix.
    #[inline]
    pub fn to_stack(&self) -> SMatrix<T, ROWS, COLS> {
        let mut stack_copy = MF::<T, ROWS, COLS>::new_stack();
        stack_copy.array_mut().copy_from_slice(self.array());
        stack_copy
    }

    #[inline]
    pub(crate) fn array(&'a self) -> &'a [[T; COLS]; ROWS] {
        self.a.as_ref()
    }

    #[inline]
    pub(crate) fn array_mut(&'a mut self) -> &'a mut [[T; COLS]; ROWS] {
        self.a.as_mut()
    }
}

#[inline]
fn copy_trans<T: Numeric<T>, const ROWS: usize, const COLS: usize>(
    source: &[[T; COLS]; ROWS],
    target: &mut [[T; ROWS]; COLS],
) {
    for (i, source_row) in source.iter().enumerate().take(ROWS) {
        for (j, source_cell) in source_row.iter().enumerate().take(COLS) {
            target[j][i] = *source_cell;
        }
    }
}

/// `MF` is the `M`atrix `F`actory used for the creation of new matrices.
pub struct MF<T: Numeric<T>, const ROWS: usize, const COLS: usize> {
    phantom: PhantomData<T>,
}

impl<T: Numeric<T>, const ROWS: usize, const COLS: usize> MF<T, ROWS, COLS> {
    /// Create a new matrix initialized with zeros on the stack.
    #[inline]
    pub fn new_stack() -> SMatrix<T, ROWS, COLS> {
        SMatrix {
            a: [[T::default(); COLS]; ROWS],
        }
    }

    /// Create a new matrix initialized with zeros on the heap.
    #[inline]
    pub fn new_heap() -> HMatrix<T, ROWS, COLS> {
        let slice = vec![[T::default(); COLS]; ROWS].into_boxed_slice();
        let ptr = Box::into_raw(slice) as *mut [[T; COLS]; ROWS];
        let box_ = unsafe { Box::from_raw(ptr) };
        HMatrix { a: box_ }
    }

    /// Create an identity matrix on the stack.
    #[inline]
    pub fn unit_stack() -> SMatrix<T, ROWS, ROWS>
    where
        T: FromStr,
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
        T: FromStr,
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
        T: FromStr,
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
        T: FromStr,
        <T as FromStr>::Err: Debug,
    {
        let mut m = MF::<T, ROWS, ROWS>::unit_heap();
        m *= diag_val;
        m
    }
}

impl<T: Numeric<T>, const ROWS: usize, const COLS: usize> Default for SMatrix<T, ROWS, COLS> {
    /// Create a new matrix initialized with zeros on the stack.
    #[inline]
    fn default() -> Self {
        MF::<T, ROWS, COLS>::new_stack()
    }
}

impl<T: Numeric<T>, const ROWS: usize, const COLS: usize> Default for HMatrix<T, ROWS, COLS> {
    /// Create a new matrix initialized with zeros on the stack.
    #[inline]
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
    fn test_create_stack_from_literal() {
        let a = SMatrix::new([[1, 2, 3], [4, 5, 6]]);
        let b = SMatrix::new([[7, 8], [9, 10], [11, 12]]);
        let c = a * b;
        println!("C (stack matrix): {:?}", c);
    }

    #[test]
    fn test_create_heap_from_literal() {
        let a = HMatrix::new([[1, 2, 3], [4, 5, 6]]);
        let b = HMatrix::new([[7, 8], [9, 10], [11, 12]]);
        let c = a * b;
        println!("C (heap matrix): {:?}", c);
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

    #[test]
    fn test_transpose_stack() {
        let mut a = MF::<f64, 2, 3>::new_stack();
        a[0][0] = 1.0;
        a[0][1] = 2.0;
        a[0][2] = 3.0;
        a[1][0] = 4.0;
        a[1][1] = 5.0;
        a[1][2] = 6.0;
        let b = a.trans();
        assert_eq!(b[0][0], 1.0);
        assert_eq!(b[1][0], 2.0);
        assert_eq!(b[2][0], 3.0);
        assert_eq!(b[0][1], 4.0);
        assert_eq!(b[1][1], 5.0);
        assert_eq!(b[2][1], 6.0);
        println!("TRANS stack: {:?}", b);
    }

    #[test]
    fn test_transpose_heap() {
        let mut a = MF::<f64, 2, 3>::new_heap();
        a[0][0] = 1.0;
        a[0][1] = 2.0;
        a[0][2] = 3.0;
        a[1][0] = 4.0;
        a[1][1] = 5.0;
        a[1][2] = 6.0;
        let b = a.trans();
        assert_eq!(b[0][0], 1.0);
        assert_eq!(b[1][0], 2.0);
        assert_eq!(b[2][0], 3.0);
        assert_eq!(b[0][1], 4.0);
        assert_eq!(b[1][1], 5.0);
        assert_eq!(b[2][1], 6.0);
        println!("TRANS heap: {:?}", b);
    }

    #[test]
    fn test_matrix_of_matrices() {
        let a = MF::<f32, 2, 2>::unit_stack();
        let b = a.clone();
        let c = a.clone();
        let d = a.clone();
        let mut matrix_of_matrices = MF::<SMatrix<f32, 2, 2>, 2, 2>::new_stack();
        matrix_of_matrices[0][0] = a;
        matrix_of_matrices[0][1] = b;
        matrix_of_matrices[1][0] = c;
        matrix_of_matrices[1][1] = d;
        let mut another_m_of_m = matrix_of_matrices.trans();
        let scalar = MF::<f32, 2, 2>::diag_stack(2.0);
        another_m_of_m *= scalar;
        let res = matrix_of_matrices * another_m_of_m;
        println!("THIS IS THE CLIMAX!: {:?}", res);
    }
}
