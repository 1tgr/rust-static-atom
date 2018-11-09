#![deny(warnings)]
#![deny(unused_extern_crates)]

extern crate serde;
extern crate static_atom;

pub trait Convention {
    const PRICE_DIGITS: u32 = 2;
}

pub trait ConventionVisitor {
    type Value;

    fn visit<C: Convention>(self) -> Self::Value;
}

#[macro_use]
pub mod atoms {
    use super::{Convention, ConventionVisitor};

    include!(concat!(env!("OUT_DIR"), "/atoms.rs"));

    impl Convention for small_type!("BTC-EUR") {}

    impl Convention for small_type!("BTC-USDC") {}

    impl Convention for small_type!("ETH-EUR") {}

    impl Convention for small_type!("ETH-BTC") {
        const PRICE_DIGITS: u32 = 5;
    }
}

pub fn quote_increment<C: Convention>() -> f64 {
    f64::powi(10.0, -(C::PRICE_DIGITS as i32))
}
