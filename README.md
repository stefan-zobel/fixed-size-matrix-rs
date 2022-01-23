[![License](https://img.shields.io/badge/license-Apache--2.0_OR_MIT-blue.svg)](https://github.com/stefan-zobel/fixed-size-matrix-rs)
[![Documentation](https://img.shields.io/badge/Docs-0.1.0-blue)](https://stefan-zobel.github.io/fixed-size-matrix-rs/)

# const_matrix-rs

Basic arithmetic for compile-time-sized matrices either allocated on the stack
([SMatrix](https://stefan-zobel.github.io/fixed-size-matrix-rs/const_matrix/matrix/struct.SMatrix.html))
or on the heap ([HMatrix](https://stefan-zobel.github.io/fixed-size-matrix-rs/const_matrix/matrix/struct.HMatrix.html))
using const generics. Both matrix types are fully interoperable with each other.
The elements of a `SMatrix` can be other `SMatrices` which theoretically could
contain `SMatrices` themselves up to arbitrarily deep finite nesting levels.
