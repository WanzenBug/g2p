// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use g2p::g2p;

g2p!(GF65536, 16);

#[test]
fn test_gf65536() {
    let z: GF65536 = 0.into();
    let e: GF65536 = 1.into();
    let a: GF65536 = 65535.into();
    let b: GF65536 = 30000.into();

    assert_eq!(z, a + a);
    assert_eq!(z, a - a);

    assert_eq!(e, a * (e / a));
    assert_eq!(a * b, b * a);
}
