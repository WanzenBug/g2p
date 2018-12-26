use g2p;
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

g2p::g2p!(GF4, 2);

#[test]
fn test_impls() {
    static_assertions::assert_impl!(test; GF4,
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
