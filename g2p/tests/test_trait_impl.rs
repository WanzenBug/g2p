use g2p::{GaloisField, g2p};


g2p!(GF256, 8);

#[test]
fn test_build() {
    static_assertions::assert_impl!(GF256, GaloisField);

    assert_eq!(GF256(0), GF256::ZERO);
    assert_eq!(GF256(1), GF256::ONE);
}
