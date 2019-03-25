# g2gen



[Documentation](https://docs.rs/g2gen)

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
Licensed under the Apache License, Version 2.0 [LICENSE-APACHE](LICENSE-APACHE)
or the MIT license [LICENSE-MIT](LICENSE-MIT)>, at your
option.
