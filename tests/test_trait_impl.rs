use g2p::{GaloisField, g2p};


g2p!(GF256, 8);

#[test]
fn test_build() {
    static_assertions::assert_impl!(GF256, GaloisField)
}
