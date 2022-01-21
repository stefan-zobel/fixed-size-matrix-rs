//! Basic matrix arithmetic using const generics

pub mod matrix;
pub mod matrix_index;
pub mod matrix_mul;
pub mod matrix_mul_assign;
pub mod mul_to_heap;
pub mod mul_to_stack;
pub mod scalar_mul_f32;
pub mod scalar_mul_f64;
pub mod scalar_mul_i128;
pub mod scalar_mul_i16;
pub mod scalar_mul_i32;
pub mod scalar_mul_i64;
pub mod scalar_mul_i8;
pub mod scalar_mul_inplace;

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
