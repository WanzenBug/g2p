// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use g2p::g2p;

g2p!(GF16, 4, modulus: 0b10011);

g2p!(GF1024, 10);

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
    let a: GF1024 = 555.into();
    let b: GF1024 = 444.into();

    let c = a + b;
    let d = a * b;

    assert_eq!(c + a, b);
    assert_eq!(c + b, a);
    assert_eq!(d / b, a);
    assert_eq!(d / a, b);
    assert_eq!(u16::from(d / b), 555_u16);
}
