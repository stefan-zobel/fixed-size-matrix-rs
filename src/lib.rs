// Copyright 2022 Stefan Zobel
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! Basic matrix arithmetic for heap and stack allocated matrices using const generics

#![crate_name = "const_matrix"]
#![warn(missing_docs)]
#![warn(rust_2018_idioms)]

pub mod matrix;
mod matrix_add;
mod matrix_add_assign;
mod matrix_index;
mod matrix_mul;
mod matrix_mul_assign;
mod matrix_neg;
mod matrix_sub;
mod matrix_sub_assign;
mod mul_to_heap;
mod mul_to_stack;
mod scalar_mul_f32;
mod scalar_mul_f64;
mod scalar_mul_i128;
mod scalar_mul_i16;
mod scalar_mul_i32;
mod scalar_mul_i64;
mod scalar_mul_i8;
mod scalar_mul_inplace;

#[cfg(test)]
mod lib_tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut a = matrix::MF::<f32, 2, 2>::new_stack();
        let mut b = matrix::MF::<f32, 2, 2>::new_heap();
        a[0][0] = 1.0f32;
        b[0][0] = 1.0;
        a[1][1] = 1.0f32;
        b[1][1] = 1.0;
        let c = a.clone() * b.clone();
        let d = b * a;
        println!("c[0][0]: {}", c[0][0]);
        println!("c[0][1]: {}", c[0][1]);
        println!("c[1][0]: {}", c[1][0]);
        println!("c[1][1]: {}", c[1][1]);
        println!("d[0][0]: {}", d[0][0]);
        println!("d[0][1]: {}", d[0][1]);
        println!("d[1][0]: {}", d[1][0]);
        println!("d[1][1]: {}", d[1][1]);
    }

    #[test]
    fn test_2() {
        let mut a = matrix::MF::<f32, 2, 2>::new_stack();
        let mut b = matrix::MF::<f32, 2, 2>::new_heap();
        a[0][0] = 1.0;
        a[0][1] = 2.0;
        a[1][0] = 3.0;
        a[1][1] = 4.0;
        b[0][0] = 5.0;
        b[0][1] = 6.0;
        b[1][0] = 7.0;
        b[1][1] = 8.0;
        let c = a.clone() * b.clone();
        let d = b * a;
        println!("c[0][0]: {}", c[0][0]);
        println!("c[0][1]: {}", c[0][1]);
        println!("c[1][0]: {}", c[1][0]);
        println!("c[1][1]: {}", c[1][1]);
        println!("d[0][0]: {}", d[0][0]);
        println!("d[0][1]: {}", d[0][1]);
        println!("d[1][0]: {}", d[1][0]);
        println!("d[1][1]: {}", d[1][1]);
    }
}
