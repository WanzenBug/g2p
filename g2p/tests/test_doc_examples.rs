// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use g2p::{g2p, GaloisField};

g2p!(GF4, 2);
g2p!(GF16, 4, modulus: 0b10011);
g2p!(GF1024, 10, modulus: 0b100_0000_1001);

#[test]
fn test_g16() {
    let one: GF16 = 1.into();
    let a: GF16 = 5.into();
    let b: GF16 = 4.into();
    let c: GF16 = 7.into();
    assert_eq!(a + c, 2.into());
    assert_eq!(a - c, 2.into());
    assert_eq!(a * b, c);
    assert_eq!(a / c, one / b);
    assert_eq!(b / b, one);
}

#[test]
fn test_gf1024() {
    eprintln!("1");
    let a: GF1024 = 555.into();
    eprintln!("2");
    let b: GF1024 = 444.into();
    eprintln!("3");
    let c = a + b;
    eprintln!("4");
    let d = a * b;
    assert_eq!(765, u16::from(d));
    eprintln!("5");

    assert_eq!(c + a, b);
    eprintln!("6");
    assert_eq!(c + b, a);
    eprintln!("7");
    assert_eq!(d / b, a);
    eprintln!("8");
    assert_eq!(d / a, b);
    eprintln!("9");
    assert_eq!(u16::from(d / b), 555_u16);
    eprintln!("10");
}

#[test]
fn test_g4() {
    let g = GF4::GENERATOR;
    assert_ne!(g * g, GF4::ONE);
    assert_eq!(g * g * g, GF4::ONE);
}

#[test]
fn test_pow() {
    let g: GF16 = 2.into();
    assert_eq!(g.pow(0), GF16::ONE);
    assert_eq!(g.pow(1), g);
    assert_eq!(g.pow(2), 4.into());
    assert_eq!(g.pow(3), 8.into());
    assert_eq!(g.pow(4), 3.into());
}
