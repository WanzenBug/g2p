use g2p::{g2p, GaloisField};
use static_assertions;

use core::ops::{
    Add,
    AddAssign,
    Sub,
    SubAssign,
    Mul,
    MulAssign,
    Div,
    DivAssign,
};
use core::fmt::{
    Debug,
    Display,
};
use core::marker::{
    Sync,
    Send,
    Sized,
    Copy,
};
use core::clone::Clone;
use core::convert::{
    From,
    Into,
};

g2p!(GF4, 2);

#[test]
fn test_impls() {
    static_assertions::assert_impl_all!(GF4:
        Clone,
        Copy,
        Send,
        Sync,
        Sized,
        Debug,
        Display,
        Add,
        AddAssign,
        Sub,
        SubAssign,
        Mul,
        MulAssign,
        Div,
        DivAssign,
        Eq,
        PartialEq,
        Into<u8>,
        From<u8>,
    );
}

#[test]
fn test_div_impl() {
    let z = GF4::ZERO;
    let a = GF4::from(3);

    assert_eq!(z, z / a);
}


#[test]
#[should_panic]
fn test_div_panic() {
    let z = GF4::ZERO;
    let a = GF4::from(3);

    let _ = a / z;
}
