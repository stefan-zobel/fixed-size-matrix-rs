//! Basic matrix arithmetic using const generics

pub mod matrix;
pub mod matrix_index;
pub mod matrix_mul;
pub mod matrix_mul_assign;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
