use g2p::{GaloisField, G2Poly, g2p};


g2p!(GF256, 8, modulus: 0b_1_0001_1011);

#[test]
fn test_build() {
    static_assertions::assert_impl!(GF256, GaloisField);

    assert_eq!(GF256::SIZE, 256);
    assert_eq!(GF256::MODULUS, G2Poly(0b_1_0001_1011));
    assert_eq!(GF256(0), GF256::ZERO);
    assert_eq!(GF256(1), GF256::ONE);
}
