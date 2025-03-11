// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Procedural macro to generate finite field types
//!
//! This is just the procedural macro, for more information look at [g2p](https://docs.rs/g2p).

#![recursion_limit = "128"]
extern crate proc_macro;

use proc_macro::TokenStream as P1TokenStream;
use proc_macro2::{Ident, Span, TokenStream as P2TokenStream};

use g2poly::{extended_gcd, G2Poly};
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, Token,
};

/// Generate a newtype of the given name and implement finite field arithmetic on it.
///
/// The generated type have implementations for [`Add`](::core::ops::Add),
/// [`Sub`](::core::ops::Sub), [`Mul`](::core::ops::Mul) and [`Div`](::core::ops::Div).
///
/// There are also implementations for equality, copy and debug. Conversion from and to the base
/// type are implemented via the From trait.
/// Depending on the size of `p` the underlying type is u8, u16 or u32.
///
/// # Example
/// ```ignore
/// g2gen::g2p!(
///     GF256,                  // Name of the newtype
///     8,                      // The power of 2 specifying the field size 2^8 = 256 in this
///                             // case.
///     modulus: 0b1_0001_1101, // The reduction polynomial to use, each bit is a coefficient.
///                             // Can be left out in case it is not needed.
/// );
///
/// # fn main() {
/// let a: GF256 = 255.into();  // Conversion from the base type
/// assert_eq!(a - a, a + a);   // Finite field arithmetic.
/// assert_eq!(format!("{}", a), "255_GF256");
/// # }
/// ```
#[proc_macro]
pub fn g2p(input: P1TokenStream) -> P1TokenStream {
    let args = parse_macro_input!(input as ParsedInput);
    let settings = Settings::from_input(args).unwrap();
    let ident = settings.ident;
    let ident_name = settings.ident_name;
    let modulus = settings.modulus;
    let generator = settings.generator;
    let p = settings.p_val;
    let field_size = 1_usize << p;
    let mask = (1_u64 << p).wrapping_sub(1);

    let ty = match p {
        0 => panic!("p must be > 0"),
        1..=8 => quote!(u8),
        9..=16 => quote!(u16),
        17..=32 => quote!(u32),
        _ => unimplemented!("p > 32 is not implemented right now"),
    };

    let mod_name = Ident::new(&format!("{}_mod", ident_name), Span::call_site());

    let struct_def = quote![
        #[derive(Clone, Copy, Eq, PartialEq, Hash)]
        pub struct #ident(pub #ty);
    ];

    let struct_impl = quote![
        impl #ident {
            pub const MASK: #ty = #mask as #ty;
        }
    ];

    let from = quote![
        impl ::core::convert::From<#ident> for #ty {
            fn from(v: #ident) -> #ty {
                v.0
            }
        }
    ];

    let into = quote![
        impl ::core::convert::From<#ty> for #ident {
            fn from(v: #ty) -> #ident {
                #ident(v & #ident::MASK)
            }
        }
    ];

    let tmpl = format!("{{}}_{}", ident_name);
    let debug = quote![
        impl ::core::fmt::Debug for #ident {
            fn fmt<'a>(&self, f: &mut ::core::fmt::Formatter<'a>) -> ::core::fmt::Result {
                write!(f, #tmpl, self.0)
            }
        }
    ];
    let display = quote![
        impl ::core::fmt::Display for #ident {
            fn fmt<'a>(&self, f: &mut ::core::fmt::Formatter<'a>) -> ::core::fmt::Result {
                write!(f, #tmpl, self.0)
            }
        }
    ];
    let add = quote![
        impl ::core::ops::Add for #ident {
            type Output = Self;

            #[allow(clippy::suspicious_arithmetic_impl)]
            fn add(self, rhs: Self) -> Self {
                Self(self.0 ^ rhs.0)
            }
        }
        impl ::core::ops::AddAssign for #ident {
            fn add_assign(&mut self, rhs: Self) {
                *self = *self + rhs;
            }
        }
    ];
    let sum = quote![
        impl ::core::iter::Sum for #ident {
            fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
                iter.fold(<Self as ::g2p::GaloisField>::ZERO, ::core::ops::Add::add)
            }
        }
    ];
    let sub = quote![
        impl ::core::ops::Sub for #ident {
            type Output = Self;


            #[allow(clippy::suspicious_arithmetic_impl)]
            fn sub(self, rhs: Self) -> Self {
                Self(self.0 ^ rhs.0)
            }
        }
        impl ::core::ops::SubAssign for #ident {
            fn sub_assign(&mut self, rhs: Self) {
                *self = *self - rhs;
            }
        }
        impl ::core::ops::Neg for #ident {
            type Output = Self;

            fn neg(self) -> Self::Output {
                self
            }
        }
    ];
    let gen = generator.0;
    let modulus_val = modulus.0;
    let galois_trait_impl = quote![
        impl ::g2p::GaloisField for #ident {
            const SIZE: usize = #field_size;
            const MODULUS: ::g2p::G2Poly = ::g2p::G2Poly(#modulus_val);
            const ZERO: Self = Self(0);
            const ONE: Self = Self(1);
            const GENERATOR: Self = Self(#gen as #ty);
        }
    ];

    let (tables, mul, div) =
        generate_mul_impl(ident.clone(), &ident_name, modulus, ty, field_size, mask);
    let product = quote![
        impl ::core::iter::Product for #ident {
            fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
                iter.fold(<Self as ::g2p::GaloisField>::ONE, ::core::ops::Mul::mul)
            }
        }
    ];

    P1TokenStream::from(quote![
        #struct_def

        mod #mod_name {
            use super::#ident;
            #struct_impl
            #tables
            #from
            #into
            #debug
            #display
            #add
            #sum
            #sub
            #mul
            #product
            #div
            #galois_trait_impl
        }
    ])
}

struct ParsedInput {
    ident: syn::Ident,
    p: syn::LitInt,
    modulus: Option<syn::LitInt>,
}

impl Parse for ParsedInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ident = input.parse()?;
        let _sep: Token![,] = input.parse()?;
        let p = input.parse()?;

        let mut modulus = None;

        loop {
            let sep: Option<Token![,]> = input.parse()?;
            if sep.is_none() || input.is_empty() {
                break;
            }
            let ident: syn::Ident = input.parse()?;
            let ident_name = ident.to_string();
            let _sep: Token![:] = input.parse()?;
            match ident_name.as_str() {
                "modulus" => {
                    if modulus.is_some() {
                        Err(syn::parse::Error::new(
                            ident.span(),
                            "Double declaration of 'modulus'",
                        ))?
                    }
                    modulus = Some(input.parse()?);
                }
                _ => Err(syn::parse::Error::new(ident.span(), "Expected 'modulus'"))?,
            }
        }

        Ok(ParsedInput { ident, p, modulus })
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Settings {
    ident: syn::Ident,
    ident_name: String,
    p_val: u64,
    modulus: G2Poly,
    generator: G2Poly,
}

fn find_modulus_poly(p: u64) -> G2Poly {
    assert!(p < 64);

    let start = (1 << p) + 1;
    let end = (1_u64 << (p + 1)).wrapping_sub(1);

    for m in start..=end {
        let p = G2Poly(m);
        if p.is_irreducible() {
            return p;
        }
    }

    unreachable!("There are irreducible polynomial for any degree!")
}

fn find_generator(m: G2Poly) -> G2Poly {
    let max = m.degree().expect("Modulus must have positive degree");

    for g in 1..(2 << max) {
        let g = G2Poly(g);
        if g.is_generator(m) {
            return g;
        }
    }

    unreachable!("There must be a generator element")
}

/// Calculate the log base 256, rounded up
///
/// Given a number n, calculate the log base 256, rounded up. This can be though of as the number
/// of bytes needed to represent this number.
fn ceil_log256(mut n: usize) -> usize {
    if n == 0 {
        return 0;
    }

    let mut c = 1;
    while n > 256 {
        c += 1;
        // NB: This is the rounding up part. If n is a proper power of 256, adding 255 will not
        // change the result. In the other cases, this ensures that we round up in the division.
        n = (n + 255) >> 8;
    }
    c
}

/// Generate multiplication array
///
/// Generate a string representing a 5d multiplication array. This array uses the associativity
/// of multiplication `(a + b) * (c + d) == a*c + a*d + b*c + b*d` to reduce table size.
///
/// The input is split into bit chunks e.g. for a GF_1024 number we take the lower 8 bit and the
/// remaining 2 and calculate the multiplications for each separately. Then we can cheaply add them
/// together to get the the result with requiring a full 1024 * 1024 input.
fn generate_mul_table_string(modulus: G2Poly) -> String {
    assert!(modulus.is_irreducible());

    let field_size = 1
        << modulus
            .degree()
            .expect("Irreducible polynomial has positive degree");
    let nparts = ceil_log256(field_size as usize);

    let mut mul_table = Vec::with_capacity(nparts);
    for left in 0..nparts {
        let mut left_parts = Vec::with_capacity(nparts);
        for right in 0..nparts {
            let mut right_parts = Vec::with_capacity(256);
            for i in 0..256 {
                let i = i << (8 * left);
                let mut row = Vec::with_capacity(256);
                for j in 0..256 {
                    let j = j << (8 * right);
                    let v = if i < field_size && j < field_size {
                        G2Poly(i as u64) * G2Poly(j as u64) % modulus
                    } else {
                        G2Poly(0)
                    };

                    row.push(format!("{}", v.0));
                }
                right_parts.push(format!("[{}]", row.join(",")));
            }
            left_parts.push(format!("[{}]", right_parts.join(",")));
        }
        mul_table.push(format!("[{}]", left_parts.join(",")));
    }

    format!("[{}]", mul_table.join(","))
}

fn generate_inv_table_string(modulus: G2Poly) -> String {
    assert!(modulus.is_irreducible());

    let field_size = 1
        << modulus
            .degree()
            .expect("Irreducible polynomial has positive degree");
    let mut inv_table = vec![0; field_size as usize];
    // Inverse table is small enough to compute directly
    for i in 1..field_size {
        if inv_table[i as usize] != 0 {
            // Already computed inverse
            continue;
        }

        let a = G2Poly(i);

        // Returns (gcd, x, y) such that gcd(a, m) == a * x + y * m
        // Since we know that gcd(a, m) == 1 and that we operate modulo m, y * m === 0 mod m
        // So we have 1 === a * x mod m

        let (_gcd, x, _y) = extended_gcd(a, modulus);
        inv_table[i as usize] = x.0;
        inv_table[x.0 as usize] = i;
    }

    use std::fmt::Write;
    let mut res = String::with_capacity(3 * field_size as usize);
    write!(&mut res, "[").unwrap();
    for v in inv_table {
        write!(&mut res, "{},", v).unwrap();
    }
    write!(&mut res, "]").unwrap();
    res
}

fn generate_mul_impl(
    ident: syn::Ident,
    ident_name: &str,
    modulus: G2Poly,
    ty: P2TokenStream,
    field_size: usize,
    mask: u64,
) -> (P2TokenStream, P2TokenStream, P2TokenStream) {
    let mul_table = generate_mul_table_string(modulus);
    let inv_table = generate_inv_table_string(modulus);

    // Faster generation than using quote
    let mul_table_string: proc_macro2::TokenStream = mul_table.parse().unwrap();
    let inv_table_string: proc_macro2::TokenStream = inv_table.parse().unwrap();

    let nparts = ceil_log256(field_size);

    // NB: We generate static arrays, as they are guaranteed to have a fixed location in memory.
    //     Using const would mean the compiler is free to create copies on the stack etc. Since
    //     The arrays are quite large, this could lead to stack overflows.
    let tables = quote! {
        pub static MUL_TABLE: [[[[#ty; 256]; 256]; #nparts]; #nparts] = #mul_table_string;
        pub static INV_TABLE: [#ty; #field_size] = #inv_table_string;
    };

    let mut mul_ops = Vec::with_capacity(nparts * nparts);
    for left in 0..nparts {
        for right in 0..nparts {
            mul_ops.push(quote![
                #ident(MUL_TABLE[#left][#right][(((self.0 & #mask as #ty) >> (8*#left)) & 255) as usize][(((rhs.0 & #mask as #ty) >> (8*#right)) & 255) as usize])
            ]);
        }
    }

    let mul = quote![
        impl ::core::ops::Mul for #ident {
            type Output = Self;
            fn mul(self, rhs: Self) -> Self {
                #(#mul_ops)+*
            }
        }
        impl ::core::ops::MulAssign for #ident {
            fn mul_assign(&mut self, rhs: Self) {
                *self = *self * rhs;
            }
        }
    ];

    let err_msg = format!("Division by 0 in {}", ident_name);

    let div = quote![
        impl ::core::ops::Div for #ident {
            type Output = Self;

            fn div(self, rhs: Self) -> Self {
                if (rhs.0 & #mask as #ty) == 0 {
                    panic!(#err_msg);
                }
                self * Self(INV_TABLE[(rhs.0 & #mask as #ty) as usize])
            }
        }
        impl ::core::ops::DivAssign for #ident {
            fn div_assign(&mut self, rhs: Self) {
                *self = *self / rhs;
            }
        }
    ];

    (tables, mul, div)
}

impl Settings {
    pub fn from_input(input: ParsedInput) -> syn::Result<Self> {
        let ident = input.ident;
        let ident_name = ident.to_string();
        let p_val = input.p.base10_parse()?;
        let modulus = match input.modulus {
            Some(lit) => G2Poly(lit.base10_parse()?),
            None => find_modulus_poly(p_val),
        };

        if !modulus.is_irreducible() {
            Err(syn::Error::new(
                Span::call_site(),
                format!("Modulus {} is not irreducible", modulus),
            ))?;
        }

        let generator = find_generator(modulus);

        if !generator.is_generator(modulus) {
            Err(syn::Error::new(
                Span::call_site(),
                format!("{} is not a generator", generator),
            ))?;
        }

        Ok(Settings {
            ident,
            ident_name,
            p_val,
            modulus,
            generator,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_settings_parser() {
        let span = Span::call_site();

        let input = ParsedInput {
            ident: Ident::new("foo", span),
            p: syn::LitInt::new("3", span),
            modulus: None,
        };

        let r = Settings::from_input(input);
        assert!(r.is_ok());
        assert_eq!(
            r.unwrap(),
            Settings {
                ident: syn::Ident::new("foo", span),
                ident_name: "foo".to_string(),
                p_val: 3,
                modulus: G2Poly(0b1011),
                generator: G2Poly(0b10),
            }
        );
    }

    #[test]
    fn test_generate_mul_table() {
        let m = G2Poly(0b111);

        assert_eq!(
            include_str!("../tests/mul_table.txt").trim(),
            generate_mul_table_string(m)
        );
    }

    #[test]
    fn test_generate_inv_table_string() {
        let m = G2Poly(0b1_0001_1011);

        assert_eq!(
            include_str!("../tests/inv_table.txt").trim(),
            generate_inv_table_string(m)
        );
    }

    #[test]
    fn test_ceil_log256() {
        assert_eq!(0, ceil_log256(0));
        assert_eq!(1, ceil_log256(1));
        assert_eq!(1, ceil_log256(256));
        assert_eq!(2, ceil_log256(257));
        assert_eq!(2, ceil_log256(65536));
        assert_eq!(3, ceil_log256(65537));
        assert_eq!(3, ceil_log256(131072));
        assert_eq!(3, ceil_log256(16777216));
        assert_eq!(4, ceil_log256(16777217));
    }
}
