// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use g2p::g2p;

g2p!(
    GF256,
    8,
    modulus: 0b_1_0001_1011,
);

#[test]
fn test_rijndael() {
    let a = GF256::from(0);
    let b = GF256::from(1);
    let c = GF256::from(0x53);
    let d = GF256::from(0xca);

    assert_eq!(GF256::MASK, 0b1111_1111);

    assert_eq!(b + a, b);
    assert_eq!(a + b, b);

    assert_eq!(a * b, a);
    assert_eq!(b * a, a);
    assert_eq!(c * d, b);
    assert_eq!(d * c, b);

    assert_eq!(b / c, d);
    assert_eq!(b / d, c);
    assert_eq!(c / b, c);
}
