#![deny(warnings)]

use std::fmt;
use std::mem;
use std::str::FromStr;

#[inline]
fn expect<'a, T: Copy + PartialEq>(s: &mut &[T], value: T) -> Option<()> {
    if let Some(b) = s.first() {
        if *b == value {
            *s = &s[1..];
            return Some(());
        }
    }

    None
}

trait ArrayExpect<T> {
    fn expect(s: &mut &[T], a: &Self) -> Option<()>;
}

impl ArrayExpect<u8> for [u8; 1] {
    #[inline]
    fn expect(s: &mut &[u8], a: &[u8; 1]) -> Option<()> {
        expect(s, a[0])
    }
}

impl ArrayExpect<u8> for [u8; 2] {
    #[inline]
    fn expect(s: &mut &[u8], a: &[u8; 2]) -> Option<()> {
        let mut s2: &[u16] = unsafe { mem::transmute(*s) };
        let a2: u16 = unsafe { mem::transmute(*a) };
        expect(&mut s2, a2)?;
        *s = &s[2..];
        Some(())
    }
}

impl ArrayExpect<u8> for [u8; 3] {
    #[inline]
    fn expect(s: &mut &[u8], a: &[u8; 3]) -> Option<()> {
        ArrayExpect::expect(s, &[a[0], a[1]])?;
        ArrayExpect::expect(s, &[a[2]])?;
        Some(())
    }
}

impl ArrayExpect<u8> for [u8; 4] {
    #[inline]
    fn expect(s: &mut &[u8], a: &[u8; 4]) -> Option<()> {
        let mut s4: &[u32] = unsafe { mem::transmute(*s) };
        let a4: u32 = unsafe { mem::transmute(*a) };
        expect(&mut s4, a4)?;
        *s = &s[4..];
        Some(())
    }
}

impl ArrayExpect<u8> for [u8; 5] {
    #[inline]
    fn expect(s: &mut &[u8], a: &[u8; 5]) -> Option<()> {
        ArrayExpect::expect(s, &[a[0], a[1], a[2], a[3]])?;
        ArrayExpect::expect(s, &[a[4]])?;
        Some(())
    }
}

impl ArrayExpect<u8> for [u8; 6] {
    #[inline]
    fn expect(s: &mut &[u8], a: &[u8; 6]) -> Option<()> {
        ArrayExpect::expect(s, &[a[0], a[1], a[2], a[3]])?;
        ArrayExpect::expect(s, &[a[4], a[5]])?;
        Some(())
    }
}

impl ArrayExpect<u8> for [u8; 7] {
    #[inline]
    fn expect(s: &mut &[u8], a: &[u8; 7]) -> Option<()> {
        ArrayExpect::expect(s, &[a[0], a[1], a[2], a[3]])?;
        ArrayExpect::expect(s, &[a[4], a[5]])?;
        ArrayExpect::expect(s, &[a[6]])?;
        Some(())
    }
}

impl ArrayExpect<u8> for [u8; 8] {
    #[inline]
    fn expect(s: &mut &[u8], a: &[u8; 8]) -> Option<()> {
        let mut s8: &[u64] = unsafe { mem::transmute(*s) };
        let a8: u64 = unsafe { mem::transmute(a) };
        expect(&mut s8, a8)?;
        *s = &s[8..];
        Some(())
    }
}

include!(concat!(env!("OUT_DIR"), "/atoms.rs"));
