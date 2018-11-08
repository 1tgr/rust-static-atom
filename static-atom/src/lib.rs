#![deny(warnings)]
#![deny(unused_extern_crates)]

extern crate itertools;

pub mod build;

use std::mem;

pub trait Expect<T>: Sized {
    fn expect(self, value: &T) -> Option<Self>;
}

impl<'a, T: PartialEq> Expect<T> for &'a [T] {
    #[inline]
    fn expect(self, value: &T) -> Option<Self> {
        if let Some(b) = self.first() {
            if b == value {
                return Some(&self[1..]);
            }
        }

        None
    }
}

impl<'a> Expect<[u8; 1]> for &'a [u8] {
    #[inline]
    fn expect(self, a: &[u8; 1]) -> Option<Self> {
        self.expect(&a[0])
    }
}

impl<'a> Expect<[u8; 2]> for &'a [u8] {
    #[inline]
    fn expect(self, a: &[u8; 2]) -> Option<Self> {
        if self.len() < 2 {
            return None;
        }

        let s2 = unsafe { mem::transmute::<&[u8], &[u16]>(self) };
        let a2 = unsafe { mem::transmute::<[u8; 2], u16>(*a) };
        s2.expect(&a2)?;
        Some(&self[2..])
    }
}

impl<'a> Expect<[u8; 3]> for &'a [u8] {
    #[inline]
    fn expect(self, a: &[u8; 3]) -> Option<Self> {
        self.expect(&[a[0], a[1]])?.expect(&[a[2]])
    }
}

impl<'a> Expect<[u8; 4]> for &'a [u8] {
    #[inline]
    fn expect(self, a: &[u8; 4]) -> Option<Self> {
        if self.len() < 4 {
            return None;
        }

        let s4 = unsafe { mem::transmute::<&[u8], &[u32]>(self) };
        let a4 = unsafe { mem::transmute::<[u8; 4], u32>(*a) };
        s4.expect(&a4)?;
        Some(&self[4..])
    }
}

impl<'a> Expect<[u8; 5]> for &'a [u8] {
    #[inline]
    fn expect(self, a: &[u8; 5]) -> Option<Self> {
        self.expect(&[a[0], a[1], a[2], a[3]])?.expect(&[a[4]])
    }
}

impl<'a> Expect<[u8; 6]> for &'a [u8] {
    #[inline]
    fn expect(self, a: &[u8; 6]) -> Option<Self> {
        self.expect(&[a[0], a[1], a[2], a[3]])?.expect(&[a[4], a[5]])
    }
}

impl<'a> Expect<[u8; 7]> for &'a [u8] {
    #[inline]
    fn expect(self, a: &[u8; 7]) -> Option<Self> {
        self.expect(&[a[0], a[1], a[2], a[3]])?
            .expect(&[a[4], a[5]])?
            .expect(&[a[6]])
    }
}

impl<'a> Expect<[u8; 8]> for &'a [u8] {
    #[inline]
    fn expect(self, a: &[u8; 8]) -> Option<Self> {
        if self.len() < 8 {
            return None;
        }

        let s8 = unsafe { mem::transmute::<&[u8], &[u64]>(self) };
        let a8 = unsafe { mem::transmute::<[u8; 8], u64>(*a) };
        s8.expect(&a8)?;
        Some(&self[8..])
    }
}
