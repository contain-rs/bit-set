//! `BitSet` of types which can be mapped to `BitIdx`
//!
//! Module is a wrapper around `BitSet` that allows the set elements to be some
//! type other than a simple `usize`. To use this `BitSet`, the type must implement
//! `Into<BitIdx>` and `From<BitIdx>`, where `BitIdx` is a simple wrapper around
//! a `usize`.
//!
//! Aside from this, this `BitSet`'s API is identical to `bit_set::BitSet`.
//!
//! # Example
//!
//! ```
//! use bit_set::bitidx::{BitSet, BitIdx};
//!
//! #[derive(Debug)]
//! enum Foo { A, B, C };
//!
//! impl From<BitIdx> for Foo {
//!     fn from(BitIdx(idx): BitIdx) -> Foo {
//!         match idx {
//!         0 => Foo::A,
//!         1 => Foo::B,
//!         2 => Foo::C,
//!         _ => panic!("Bad idx {}", idx),
//!         }
//!     }
//! }
//!
//! impl Into<BitIdx> for Foo {
//!     fn into(self) -> BitIdx { BitIdx(self as usize) }
//! }
//!
//! let mut s = BitSet::new();
//! s.insert(Foo::A);
//! s.insert(Foo::B);
//!
//! assert!(s.contains(Foo::A));
//! assert!(!s.contains(Foo::C));
//! println!("set: {:?}", s);
//! ```
use std::iter::FromIterator;
use std::marker::PhantomData;
use std::fmt::{self, Debug};
use bit_vec::{BitVec, BitBlock};

/// Wrapper for a bit index
///
/// This is a simple wrapper for usize so that types can implement `Into`/`From`
/// for BitIdx. Default implementation for `usize` so it works as normal.
#[derive(Debug, Eq, PartialEq)]
pub struct BitIdx(pub usize);

impl Into<BitIdx> for usize {
    #[inline] fn into(self) -> BitIdx { BitIdx(self) }
}

impl<'a> Into<BitIdx> for &'a usize {
    #[inline] fn into(self) -> BitIdx { BitIdx(*self) }
}

impl From<BitIdx> for usize {
    #[inline] fn from(bit: BitIdx) -> usize { bit.0 }
}

impl AsRef<usize> for BitIdx {
    #[inline] fn as_ref(&self) -> &usize { &self.0 }
}

/// A set of elements represented as a bit vector.
///
/// Elements of the set are any type that implements `Into<BitIdx>` and
/// `From<BitIdx>`. It is a wrapper around the `bit_set::BitSet` where the
/// elements are always represented by `usize`.
///
/// `usize` implements `Into<BitIdx>`/`From<BitIdx>`, and so can be used
/// as a set element here.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct BitSet<T, B=u32>(super::BitSet<B>, PhantomData<T>)
    where B: BitBlock;

impl<T> BitSet<T, u32> {
    #[inline]
    pub fn new() -> Self { Self::default() }

    #[inline]
    pub fn with_capacity(nbits: usize) -> Self {
        BitSet(super::BitSet::with_capacity(nbits), PhantomData)
    }

    #[inline]
    pub fn from_bit_vec(bit_vec: BitVec) -> Self {
        BitSet(super::BitSet::from_bit_vec(bit_vec), PhantomData)
    }

    #[inline]
    pub fn from_bytes(bytes: &[u8]) -> Self {
        BitSet(super::BitSet::from_bytes(bytes), PhantomData)
    }
}

impl<T, B: BitBlock> BitSet<T, B> {
    #[inline]
    pub fn capacity(&self) -> usize { self.0.capacity() }

    #[inline]
    pub fn reserve_len(&mut self, len: usize) { self.0.reserve_len(len) }

    #[inline]
    pub fn reserve_len_exact(&mut self, len: usize) { self.0.reserve_len_exact(len) }

    #[inline]
    pub fn into_bit_vec(self) -> BitVec<B> { self.0.into_bit_vec() }

    #[inline]
    pub fn get_ref(&self) -> &BitVec<B> { self.0.get_ref() }

    #[inline]
    pub fn shrink_to_fit(&mut self) { self.0.shrink_to_fit() }

    #[inline]
    pub fn union_with(&mut self, other: &Self) {
        self.0.union_with(&other.0)
    }

    #[inline]
    pub fn intersect_with(&mut self, other: &Self) {
        self.0.intersect_with(&other.0)
    }

    #[inline]
    pub fn difference_with(&mut self, other: &Self) {
        self.0.difference_with(&other.0)
    }

    #[inline]
    pub fn symmetric_difference_with(&mut self, other: &Self) {
        self.0.symmetric_difference_with(&other.0)
    }

    #[inline]
    pub fn len(&self) -> usize { self.0.len() }

    #[inline]
    pub fn is_empty(&self) -> bool { self.0.is_empty() }

    #[inline]
    pub fn clear(&mut self) { self.0.clear() }

    #[inline]
    pub fn is_disjoint(&self, other: &Self) -> bool {
        self.0.is_disjoint(&other.0)
    }

    #[inline]
    pub fn is_subset(&self, other: &Self) -> bool {
        self.0.is_subset(&other.0)
    }

    #[inline]
    pub fn is_superset(&self, other: &Self) -> bool {
        self.0.is_superset(&other.0)
    }
}

impl<T, B> BitSet<T, B>
    where T: From<BitIdx>, B: BitBlock
{
    #[inline]
    pub fn iter<'a>(&'a self) -> Iter<'a, T, B> {
        MapBitIdx(self.0.iter(), PhantomData)
    }

    #[inline]
    pub fn union<'a>(&'a self, other: &'a Self) -> Union<'a, T, B> {
        MapBitIdx(self.0.union(&other.0), PhantomData)
    }

    #[inline]
    pub fn intersection<'a>(&'a self, other: &'a Self) -> Intersection<'a, T, B> {
        MapBitIdx(self.0.intersection(&other.0), PhantomData)
    }

    #[inline]
    pub fn difference<'a>(&'a self, other: &'a Self) -> Difference<'a, T, B> {
        MapBitIdx(self.0.difference(&other.0), PhantomData)
    }

    #[inline]
    pub fn symmetric_difference<'a>(&'a self, other: &'a Self) -> SymmetricDifference<'a, T, B> {
        MapBitIdx(self.0.symmetric_difference(&other.0), PhantomData)
    }
}

impl<T, B> BitSet<T, B>
    where T: Into<BitIdx>, B: BitBlock
{
    #[inline]
    pub fn contains(&self, value: T) -> bool {
        let BitIdx(idx) = value.into();
        self.0.contains(idx)
    }

    #[inline]
    pub fn insert(&mut self, value: T) -> bool {
        let BitIdx(idx) = value.into();
        self.0.insert(idx)
    }

    #[inline]
    pub fn remove(&mut self, value: T) -> bool {
        let BitIdx(idx) = value.into();
        self.0.remove(idx)
    }
}

pub type Iter<'a, T, B> = MapBitIdx<super::Iter<'a, B>, T>;
pub type Union<'a, T, B> = MapBitIdx<super::Union<'a, B>, T>;
pub type Intersection<'a, T, B> = MapBitIdx<super::Intersection<'a, B>, T>;
pub type Difference<'a, T, B> = MapBitIdx<super::Difference<'a, B>, T>;
pub type SymmetricDifference<'a, T, B> = MapBitIdx<super::SymmetricDifference<'a, B>, T>;

#[derive(Clone)]
#[doc(hidden)]
pub struct MapBitIdx<I, T>(I, PhantomData<T>);

impl<I, T> Iterator for MapBitIdx<I, T>
    where I: Iterator<Item=usize>, T: From<BitIdx>
{
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<T> {
        self.0.next().map(|v| T::from(BitIdx(v)))
    }
}

impl<T, B: BitBlock> Default for BitSet<T, B> {
    fn default() -> Self { BitSet(Default::default(), PhantomData) }
}

impl<'a, T, B> IntoIterator for &'a BitSet<T, B>
    where T: From<BitIdx> + 'a, B: BitBlock
{
    type Item = T;
    type IntoIter = Iter<'a, T, B>;
    fn into_iter(self) -> Self::IntoIter { self.iter() }
}

impl<T, B> FromIterator<T> for BitSet<T, B>
    where T: Into<BitIdx>, B: BitBlock
{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut ret = Self::default();
        ret.extend(iter);
        ret
    }
}

impl<T, B> Extend<T> for BitSet<T, B>
    where T: Into<BitIdx>, B: BitBlock
{
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for i in iter {
            self.insert(i);
        }
    }
}

impl<T: From<BitIdx> + Debug, B: BitBlock> fmt::Debug for BitSet<T, B> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_set().entries(self).finish()
    }
}

#[cfg(test)]
mod test {
    use super::{BitSet, BitIdx};

    #[derive(Debug, PartialEq)]
    enum Foo { A, B, C, D }

    impl Into<BitIdx> for Foo {
        fn into(self) -> BitIdx { BitIdx(self as usize) }
    }

    impl From<BitIdx> for Foo {
        fn from(BitIdx(v): BitIdx) -> Self {
            match v {
                0 => Foo::A,
                1 => Foo::B,
                2 => Foo::C,
                3 => Foo::D,
                _ => panic!("bad v {}", v),
            }
        }
    }

    #[test]
    fn iter() {
        let mut s = BitSet::new();

        s.insert(Foo::A);
        s.insert(Foo::C);

        let v: Vec<_> = s.iter().collect();
        assert_eq!(v, vec![Foo::A, Foo::C]);
    }
}
