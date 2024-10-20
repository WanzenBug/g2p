// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! This crate can generate types that implement fast finite field arithmetic.
//!
//! Many error correcting codes rely on some form of finite field of the form GF(2^p), where
//! p is relatively small. Similarly some cryptographic algorithms such as AES use finite field
//! arithmetic.
//!
//! While addition and subtraction can be done quickly using just a simple XOR, multiplication is
//! more involved. To speed things up, you can use a precomputed table. Typically this table is just
//! copied into the source code directly.
//!
//! Using this crate, you can have the benefits in speed of precomputed table, without the need
//! to create your own type with custom multiplication and division implementation.
//!
//! # WARNING
//! The types generated by this library are probably not suitable for cryptographic purposes, as
//! multiplication is not guaranteed to be constant time.
//!
//! # Note
//! The implementation was tested for finite fields up to 2^17 in size, which compiles reasonably
//! fast. The space requirements are linear to the field size for the inversion table and log^2(N)
//! for the multiplication table. This means it is not feasible to use this to generate fields of
//! size 2^32, which would 4*4GB memory.
//!
//! # Examples
//!
//! ```rust
//! g2p::g2p!(GF16, 4, modulus: 0b10011);
//! # fn main() {
//! let one: GF16 = 1.into();
//! let a: GF16 = 5.into();
//! let b: GF16 = 4.into();
//! let c: GF16 = 7.into();
//! assert_eq!(a + c, 2.into());
//! assert_eq!(a - c, 2.into());
//! assert_eq!(a * b, c);
//! assert_eq!(a / c, one / b);
//! assert_eq!(b / b, one);
//! # }
//! ```
//!
//! # Implementation details
//! `g2p` generates a new type that implements all the common arithmetic operations. The
//! calculations are performed on either u8, u16 or u32, depending on the field size.
//!
//! Addition and subtraction are implemented using regular `Xor`. For division, the divisor inverted
//! using a precomputed inversion table, which is then multiplied using the multiplication outlined
//! below
//!
//! ## Multiplication
//! Multiplication uses a number of precomputed tables to determine the result. Because a full table
//! would grow with the square of the field size, this approach was not deemed feasible. For
//! example, using a full table for GF65536 = GF(2^16), one would need 2^32 entries, which would
//! mean the program reserves 2*4GB just for these tables alone.
//!
//! Instead a number `n` is split into 8bit components `n = a + 256 * b + 65536 * c ...`. Using this
//! representation we can multiply two numbers by cross-multiplying all the components
//! and then adding them up again. So assuming 16bit numbers `n = n0 + 256 * n1` and
//! `m = m0 + 256 * m1` we get `n*m = n0*m0 + 256*n0*m1 + 256*n1*m0 + 65536*n1*m1`.
//!
//! We can now create precomputed tables for multiplying the different components together. There is
//! a table for first component times first component, first times second etc. The results then just
//! have to be added together using the normal finite field addition. For our GF65536 example this
//! means the multiplication tables use 4 * 256 * 256 entries á 2 byte which is ~0.5MB

use core::{
    iter::{Product, Sum},
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

/// Procedural macro to generate binary galois fields
pub use g2gen::g2p;

/// Polynomial representation of values
pub use g2poly::G2Poly;

/// Common trait for finite fields
///
/// All types generated by `g2p!` implement this trait.
/// The trait ensures that all the expected operations of a finite field are implemented.
///
/// In addition, some often used constants like `ONE` and `ZERO` are exported, as well as the more
/// esoteric `GENERATOR`.
pub trait GaloisField:
    Add<Output = Self>
    + AddAssign
    + Sum
    + Sub<Output = Self>
    + SubAssign
    + Neg<Output = Self>
    + Mul<Output = Self>
    + MulAssign
    + Product
    + Div<Output = Self>
    + DivAssign
    + Copy
    + PartialEq
    + Eq
{
    /// Number of elements in the field
    const SIZE: usize;

    /// The value 0 as a finite field constant
    const ZERO: Self;
    /// The value 1 as a finite field constant
    const ONE: Self;
    /// A generator of the multiplicative group of a finite field
    ///
    /// The powers of this element will generate all non-zero elements of the finite field
    ///
    /// ```rust
    /// use g2p::{GaloisField, g2p};
    ///
    /// g2p!(GF4, 2);
    /// # fn main() {
    /// let g = GF4::GENERATOR;
    /// assert_ne!(g, GF4::ONE);
    /// assert_ne!(g * g, GF4::ONE);
    /// assert_eq!(g * g * g, GF4::ONE);
    /// # }
    /// ```
    const GENERATOR: Self;

    /// Polynomial representation of the modulus used to generate the field
    const MODULUS: G2Poly;

    /// Calculate the p-th power of a value
    ///
    /// Calculate the value of x to the power p in finite field arithmethic
    ///
    /// # Example
    /// ```rust
    /// use g2p::{GaloisField, g2p};
    ///
    /// g2p!(GF16, 4);
    /// # fn main() {
    /// let g: GF16 = 2.into();
    /// assert_eq!(g.pow(0), GF16::ONE);
    /// assert_eq!(g.pow(1), g);
    /// assert_eq!(g.pow(2), 4.into());
    /// assert_eq!(g.pow(3), 8.into());
    /// assert_eq!(g.pow(4), 3.into());
    /// # }
    /// ```
    fn pow(self, p: usize) -> Self {
        let mut val = Self::ONE;
        let mut pow_pos = 1 << (::std::mem::size_of::<usize>() * 8 - 1);
        assert_eq!(pow_pos << 1, 0);
        while pow_pos > 0 {
            val *= val;
            if (pow_pos & p) > 0 {
                val *= self;
            }
            pow_pos >>= 1;
        }
        val
    }
}
