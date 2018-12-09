#![deny(warnings)]

use std::fmt;
use std::marker::PhantomData;

use static_atom::Mapping;
use try_from::TryFrom;

pub trait Convention {
    const QUOTE_SIZE: f64 = 0.01;
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
        const QUOTE_SIZE: f64 = 0.00001;
    }
}

pub fn price_digits<C: Convention>() -> i32 {
    -C::QUOTE_SIZE.log10().ceil() as i32
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Price<C>(i32, PhantomData<C>);

impl<C> fmt::Debug for Price<C>
where
    C: Convention,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Price({} * {})", self.0, C::QUOTE_SIZE)
    }
}

impl<C> TryFrom<f64> for Price<C>
where
    C: Convention,
{
    type Err = ();

    fn try_from(f: f64) -> Result<Self, ()> {
        let g = f / C::QUOTE_SIZE;
        if (g - g.round()).abs() < 1E-16 {
            Ok(Price(g as i32, PhantomData))
        } else {
            Err(())
        }
    }
}

impl<'a, C> From<&'a Price<C>> for f64
where
    C: Convention,
{
    fn from(p: &Price<C>) -> Self {
        p.0 as f64 * C::QUOTE_SIZE
    }
}

pub struct PriceMapping;

impl<C> Mapping<C> for PriceMapping
where
    C: Convention,
{
    type Value = Price<C>;
}
