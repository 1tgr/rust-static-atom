#![deny(warnings)]

pub mod iterators;

use std::iter::FromIterator;
use std::mem;

use crate::iterators::{Iter, IterMut, Keys, Values};

pub use try_from::TryFrom;

pub trait AtomMap: FromIterator<(<Self as AtomMap>::Key, <Self as AtomMap>::Value)> {
    type Key: TryFrom<usize>;
    type Value;

    fn entry(&self, key: Self::Key) -> &Option<Self::Value>;
    fn entry_mut(&mut self, key: Self::Key) -> &mut Option<Self::Value>;
    fn entries(&self) -> &[Option<Self::Value>];
    fn entries_mut(&mut self) -> &mut [Option<Self::Value>];

    fn get(&self, key: Self::Key) -> Option<&Self::Value> {
        self.entry(key).as_ref()
    }

    fn get_mut(&mut self, key: Self::Key) -> Option<&mut Self::Value> {
        self.entry_mut(key).as_mut()
    }

    fn insert(&mut self, key: Self::Key, value: Self::Value) -> Option<Self::Value> {
        mem::replace(self.entry_mut(key), Some(value))
    }

    fn remove(&mut self, key: Self::Key) -> Option<Self::Value> {
        mem::replace(self.entry_mut(key), None)
    }

    fn get_or_insert(&mut self, key: Self::Key, value: Self::Value) -> &mut Self::Value {
        let entry = self.entry_mut(key);
        if entry.is_none() {
            *entry = Some(value);
        }

        entry.as_mut().unwrap()
    }

    fn get_or_insert_with<F: FnOnce() -> Self::Value>(&mut self, key: Self::Key, f: F) -> &mut Self::Value {
        let entry = self.entry_mut(key);
        if entry.is_none() {
            *entry = Some(f());
        }

        entry.as_mut().unwrap()
    }

    fn iter(&self) -> Iter<Self::Key, Self::Value> {
        Iter::new(self.entries())
    }

    fn iter_mut(&mut self) -> IterMut<Self::Key, Self::Value> {
        IterMut::new(self.entries_mut())
    }

    fn keys(&self) -> Keys<Self::Key, Self::Value> {
        Keys::new(self.entries())
    }

    fn values(&self) -> iterators::Values<Self::Value> {
        Values::new(self.entries())
    }
}

pub trait Mapping<Atom> {
    type Value;
}

pub trait TypedAtomMap<M> {
    fn entry<A>(&self) -> &Option<<M as Mapping<A>>::Value>
    where
        M: Mapping<A>,
        A: 'static;

    fn entry_mut<A>(&mut self) -> &mut Option<<M as Mapping<A>>::Value>
    where
        M: Mapping<A>,
        A: 'static;

    fn get<A>(&self) -> Option<&<M as Mapping<A>>::Value>
    where
        M: Mapping<A>,
        A: 'static,
    {
        self.entry::<A>().as_ref()
    }

    fn get_mut<A>(&mut self) -> Option<&mut <M as Mapping<A>>::Value>
    where
        M: Mapping<A>,
        A: 'static,
    {
        self.entry_mut::<A>().as_mut()
    }

    fn insert<A>(&mut self, value: <M as Mapping<A>>::Value) -> Option<<M as Mapping<A>>::Value>
    where
        M: Mapping<A>,
        A: 'static,
    {
        mem::replace(self.entry_mut::<A>(), Some(value))
    }

    fn remove<A>(&mut self) -> Option<<M as Mapping<A>>::Value>
    where
        M: Mapping<A>,
        A: 'static,
    {
        mem::replace(self.entry_mut::<A>(), None)
    }

    fn get_or_insert<A>(&mut self, value: <M as Mapping<A>>::Value) -> &mut <M as Mapping<A>>::Value
    where
        M: Mapping<A>,
        A: 'static,
    {
        let entry = self.entry_mut::<A>();
        if entry.is_none() {
            *entry = Some(value);
        }

        entry.as_mut().unwrap()
    }

    fn get_or_insert_with<A, F>(&mut self, f: F) -> &mut <M as Mapping<A>>::Value
    where
        M: Mapping<A>,
        A: 'static,
        F: FnOnce() -> <M as Mapping<A>>::Value,
    {
        let entry = self.entry_mut::<A>();
        if entry.is_none() {
            *entry = Some(f());
        }

        entry.as_mut().unwrap()
    }
}

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
