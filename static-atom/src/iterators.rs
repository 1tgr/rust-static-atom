use std::marker::PhantomData;
use std::slice;

use try_from::TryFrom;

pub struct Iter<'a, Key, Value: 'a> {
    entries: slice::Iter<'a, Option<Value>>,
    index: usize,
    _pd: PhantomData<Key>,
}

impl<'a, Key, Value: 'a> Iter<'a, Key, Value> {
    pub fn new(slice: &'a [Option<Value>]) -> Self {
        Iter {
            entries: slice.iter(),
            index: 0,
            _pd: PhantomData,
        }
    }
}

impl<'a, Key, Value> Iterator for Iter<'a, Key, Value>
where
    Key: TryFrom<usize>,
{
    type Item = (Key, &'a Value);

    fn next(&mut self) -> Option<(Key, &'a Value)> {
        while let Some(opt) = self.entries.next() {
            let key = Key::try_from(self.index).ok().unwrap();
            self.index += 1;

            if let Some(value) = opt.as_ref() {
                return Some((key, value));
            }
        }

        None
    }
}

pub struct IterMut<'a, Key, Value: 'a> {
    entries: slice::IterMut<'a, Option<Value>>,
    index: usize,
    _pd: PhantomData<Key>,
}

impl<'a, Key, Value: 'a> IterMut<'a, Key, Value> {
    pub fn new(slice: &'a mut [Option<Value>]) -> Self {
        IterMut {
            entries: slice.iter_mut(),
            index: 0,
            _pd: PhantomData,
        }
    }
}

impl<'a, Key, Value> Iterator for IterMut<'a, Key, Value>
where
    Key: TryFrom<usize>,
{
    type Item = (Key, &'a mut Value);

    fn next(&mut self) -> Option<(Key, &'a mut Value)> {
        while let Some(opt) = self.entries.next() {
            let key = Key::try_from(self.index).ok().unwrap();
            self.index += 1;

            if let Some(value) = opt.as_mut() {
                return Some((key, value));
            }
        }

        None
    }
}

pub struct Keys<'a, Key, Value: 'a> {
    entries: slice::Iter<'a, Option<Value>>,
    index: usize,
    _pd: PhantomData<Key>,
}

impl<'a, Key, Value: 'a> Keys<'a, Key, Value> {
    pub fn new(slice: &'a [Option<Value>]) -> Self {
        Keys {
            entries: slice.iter(),
            index: 0,
            _pd: PhantomData,
        }
    }
}

impl<'a, Key, Value> Iterator for Keys<'a, Key, Value>
where
    Key: TryFrom<usize>,
{
    type Item = Key;

    fn next(&mut self) -> Option<Key> {
        while let Some(opt) = self.entries.next() {
            let key = Key::try_from(self.index).ok().unwrap();
            self.index += 1;

            if opt.is_some() {
                return Some(key);
            }
        }

        None
    }
}

pub struct Values<'a, Value: 'a> {
    entries: slice::Iter<'a, Option<Value>>,
}

impl<'a, Value: 'a> Values<'a, Value> {
    pub fn new(slice: &'a [Option<Value>]) -> Self {
        Values { entries: slice.iter() }
    }
}

impl<'a, Value> Iterator for Values<'a, Value> {
    type Item = &'a Value;

    fn next(&mut self) -> Option<&'a Value> {
        while let Some(opt) = self.entries.next() {
            if let Some(value) = opt.as_ref() {
                return Some(value);
            }
        }

        None
    }
}
