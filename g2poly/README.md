# g2poly

A small library to handle polynomials of degree < 64 over the finite field GF(2).

The main motivation for this library is generating finite fields of the form GF(2^p).
Elements of GF(2^p) can be expressed as polynomials over GF(2) with degree < p. These
finite fields are used in cryptographic algorithms as well as error detecting / correcting
codes.

[Documentation](https://docs.rs/g2poly)

# Example

```rust
use g2poly;

let a = g2poly::G2Poly(0b10011);
assert_eq!(format!("{}", a), "G2Poly { x^4 + x + 1 }");
let b = g2poly::G2Poly(0b1);
assert_eq!(a + b, g2poly::G2Poly(0b10010));

// Since products could overflow in u64, the product is defined as a u128
assert_eq!(a * a, g2poly::G2PolyProd(0b100000101));

// This can be reduced using another polynomial
let s = a * a % g2poly::G2Poly(0b1000000);
assert_eq!(s, g2poly::G2Poly(0b101));
```

## License
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
