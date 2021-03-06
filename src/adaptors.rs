//! Licensed under the Apache License, Version 2.0
//! http://www.apache.org/licenses/LICENSE-2.0 or the MIT license
//! http://opensource.org/licenses/MIT, at your
//! option. This file may not be copied, modified, or distributed
//! except according to those terms.

use std::mem;
#[cfg(feature = "unstable")]
use std::num::One;
#[cfg(feature = "unstable")]
use std::ops::Add;
use std::usize;
use std::iter::{Fuse, Peekable};
use super::Itertools;

macro_rules! clone_fields {
    ($name:ident, $base:expr, $($field:ident),+) => (
        $name {
            $(
                $field : $base . $field .clone()
            ),*
        }
    );
}


/// Alternate elements from two iterators until both
/// are run out
///
/// Iterator element type is `A` if `I: Iterator<A>`
#[derive(Clone)]
pub struct Interleave<I, J> {
    a: I,
    b: J,
    flag: bool,
}

impl<I, J> Interleave<I, J> {
    ///
    pub fn new(a: I, b: J) -> Interleave<I, J> {
        Interleave{a: a, b: b, flag: false}
    }
}

impl<I, J> Iterator for Interleave<I, J> where
    I: Iterator,
    J: Iterator<Item=I::Item>,
{
    type Item = I::Item;
    #[inline]
    fn next(&mut self) -> Option<I::Item> {
        self.flag = !self.flag;
        if self.flag {
            match self.a.next() {
                None => self.b.next(),
                r => r,
            }
        } else {
            match self.b.next() {
                None => self.a.next(),
                r => r,
            }
        }
    }
}

/// Clonable iterator adaptor to map elementwise
/// from `Iterator<A>` to `Iterator<B>`
///
/// Created with `.fn_map(..)` on an iterator
///
/// Iterator element type is `B`
pub struct FnMap<B, I> where
    I: Iterator,
{
    map: fn(I::Item) -> B,
    iter: I,
}

impl<B, I> FnMap<B, I> where
    I: Iterator
{
    pub fn new(iter: I, map: fn(I::Item) -> B) -> Self
    {
        FnMap{iter: iter, map: map}
    }
}

impl<B, I> Iterator for FnMap<B, I> where
    I: Iterator,
{
    type Item = B;
    #[inline]
    fn next(&mut self) -> Option<B>
    {
        self.iter.next().map(|a| (self.map)(a))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<B, I> DoubleEndedIterator for FnMap<B, I> where
    I: DoubleEndedIterator
{
    #[inline]
    fn next_back(&mut self) -> Option<B> {
        self.iter.next_back().map(|a| (self.map)(a))
    }
}

impl<B, I> Clone for FnMap<B, I> where
    I: Clone + Iterator,
{
    fn clone(&self) -> Self
    {
        FnMap::new(self.iter.clone(), self.map)
    }
}

#[derive(Clone)]
/// An iterator adaptor that allows putting back a single
/// item to the front of the iterator.
///
/// Iterator element type is **I::Item**.
pub struct PutBack<I> where
    I: Iterator,
{
    top: Option<I::Item>,
    iter: I
}

impl<I> PutBack<I>
    where I: Iterator,
{
    /// Iterator element type is `A`
    #[inline]
    pub fn new(it: I) -> Self
    {
        PutBack{top: None, iter: it}
    }

    /// Put back a single value to the front of the iterator.
    ///
    /// If a value is already in the put back slot, it is overwritten.
    #[inline]
    pub fn put_back(&mut self, x: I::Item)
    {
        self.top = Some(x)
    }
}

impl<I> Iterator for PutBack<I> where
    I: Iterator,
{
    type Item = I::Item;
    #[inline]
    fn next(&mut self) -> Option<I::Item> {
        match self.top {
            None => self.iter.next(),
            ref mut some => some.take(),
        }
    }
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let (lo, hi) = self.iter.size_hint();
        match self.top {
            Some(_) => (lo.saturating_add(1), hi.and_then(|x| x.checked_add(1))),
            None => (lo, hi)
        }
    }
}

#[derive(Clone)]
/// An iterator adaptor that iterates over the cartesian product of
/// the element sets of two iterators **I** and **J**.
///
/// Iterator element type is **(I::Item, J::Item)**.
pub struct Product<I, J> where
    I: Iterator,
{
    a: I,
    a_cur: Option<I::Item>,
    b: J,
    b_orig: J,
}

impl<I, J> Product<I, J> where
    I: Iterator,
    J: Clone + Iterator,
    I::Item: Clone,
{
    /// Create a new cartesian product iterator
    ///
    /// Iterator element type is **(I::Item, J::Item)**.
    pub fn new(i: I, j: J) -> Self
    {
        let mut i = i;
        Product{a_cur: i.next(), a: i, b: j.clone(), b_orig: j}
    }
}


impl<I, J> Iterator for Product<I, J> where
    I: Iterator,
    J: Clone + Iterator,
    I::Item: Clone,
{
    type Item = (I::Item, J::Item);
    fn next(&mut self) -> Option<(I::Item, J::Item)>
    {
        let elt_b = match self.b.next() {
            None => {
                self.b = self.b_orig.clone();
                match self.b.next() {
                    None => return None,
                    Some(x) => {
                        self.a_cur = self.a.next();
                        x
                    }
                }
            }
            Some(x) => x
        };
        match self.a_cur {
            None => None,
            Some(ref a) => {
                Some((a.clone(), elt_b))
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>)
    {
        let (a, ah) = self.a.size_hint();
        let (b, bh) = self.b.size_hint();
        let (bo, boh) = self.b_orig.size_hint();

        // Compute a * bo + b for both lower and upper bound
        let low = a.checked_mul(bo)
                    .and_then(|x| x.checked_add(b))
                    .unwrap_or(::std::usize::MAX);
        let high = ah.and_then(|x| boh.and_then(|y| x.checked_mul(y)))
                     .and_then(|x| bh.and_then(|y| x.checked_add(y)));
        (low, high)
    }
}

#[derive(Clone)]
/// Remove duplicates from sections of consecutive identical elements.
/// If the iterator is sorted, all elements will be unique.
///
/// Iterator element type is **I::Item**.
pub struct Dedup<I> where
    I: Iterator,
{
    last: Option<I::Item>,
    iter: I,
}

impl<I> Dedup<I> where I: Iterator
{
    /// Create a new Dedup Iterator.
    pub fn new(iter: I) -> Dedup<I>
    {
        Dedup{last: None, iter: iter}
    }
}

impl<I> Iterator for Dedup<I> where
    I: Iterator,
    I::Item: PartialEq
{
    type Item = I::Item;
    #[inline]
    fn next(&mut self) -> Option<I::Item>
    {
        for elt in self.iter.by_ref() {
            match self.last {
                Some(ref x) if x == &elt => continue,
                None => {
                    self.last = Some(elt);
                    continue;
                }

                ref mut lst => {
                    let ret = mem::replace(lst, Some(elt));
                    return ret
                }
            }
        }
        self.last.take()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>)
    {
        let (lower, upper) = self.iter.size_hint();
        if self.last.is_some() || lower > 0 {
            (1, upper.and_then(|x| x.checked_add(1)))
        } else {
            // they might all be duplicates
            (0, upper)
        }
    }
}


/// A “meta iterator adaptor”. Its closure recives a reference to the iterator
/// and may pick off as many elements as it likes, to produce the next iterator element.
///
/// Iterator element type is *X*, if the return type of **F** is *Option\<X\>*.
#[derive(Clone)]
pub struct Batching<I, F> {
    f: F,
    iter: I,
}

impl<F, I> Batching<I, F> {
    /// Create a new Batching iterator.
    pub fn new(iter: I, f: F) -> Batching<I, F>
    {
        Batching{f: f, iter: iter}
    }
}

impl<B, F, I> Iterator for Batching<I, F> where
    I: Iterator,
    F: FnMut(&mut I) -> Option<B>,
{
    type Item = B;
    #[inline]
    fn next(&mut self) -> Option<B>
    {
        (self.f)(&mut self.iter)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>)
    {
        // No information about closue behavior
        (0, None)
    }
}

#[derive(Clone)]
/// Group iterator elements. Consecutive elements that map to the same key ("runs"),
/// are returned as the iterator elements of `GroupBy`.
///
/// Iterator element type is **(K, Vec\<A\>)**
pub struct GroupBy<K, I, F> where
    I: Iterator,
{
    key: F,
    iter: I,
    current_key: Option<K>,
    elts: Vec<I::Item>,
}

impl<K, F, I> GroupBy<K, I, F> where
    I: Iterator,
{
    /// Create a new GroupBy iterator.
    pub fn new(iter: I, key: F) -> Self
    {
        GroupBy{key: key, iter: iter, current_key: None, elts: Vec::new()}
    }
}

impl<K, I, F> Iterator for GroupBy<K, I, F> where
    K: PartialEq,
    I: Iterator,
    F: FnMut(&I::Item) -> K,
{
    type Item = (K, Vec<I::Item>);
    fn next(&mut self) -> Option<(K, Vec<I::Item>)>
    {
        for elt in self.iter.by_ref() {
            let key = (self.key)(&elt);
            match self.current_key.take() {
                None => {}
                Some(old_key) => if old_key != key {
                    self.current_key = Some(key);
                    let v = mem::replace(&mut self.elts, vec![elt]);
                    return Some((old_key, v))
                },
            }
            self.current_key = Some(key);
            self.elts.push(elt);
        }
        match self.current_key.take() {
            None => None,
            Some(key) => {
                let v = mem::replace(&mut self.elts, Vec::new());
                Some((key, v))
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>)
    {
        let (lower, upper) = self.iter.size_hint();
        let stored_count = self.current_key.is_some() as usize;
        let my_upper = upper.and_then(|x| x.checked_add(stored_count));
        if lower > 0 || stored_count > 0 {
            (1, my_upper)
        } else {
            (0, my_upper)
        }
    }
}

/// An iterator adaptor that steps a number elements in the base iterator
/// for each iteration.
///
/// The iterator steps by yielding the next element from the base iterator,
/// then skipping forward *n-1* elements.
#[derive(Clone)]
pub struct Step<I> {
    iter: Fuse<I>,
    skip: usize,
}

impl<I> Step<I>
    where I: Iterator
{
    /// Create a **Step** iterator.
    ///
    /// **Panics** if the step is 0.
    pub fn new(iter: I, step: usize) -> Self
    {
        assert!(step != 0);
        Step{iter: iter.fuse(), skip: step - 1}
    }
}

impl<I> Iterator for Step<I>
    where I: Iterator
{
    type Item = I::Item;
    #[inline]
    fn next(&mut self) -> Option<I::Item>
    {
        let elt = self.iter.next();
        self.iter.dropn(self.skip);
        elt
    }

    fn size_hint(&self) -> (usize, Option<usize>)
    {
        let (low, high) = self.iter.size_hint();
        let div = |x: usize| {
            if x == 0 {
                0
            } else {
                1 + (x - 1) / (self.skip + 1)
            }
        };
        (div(low), high.map(div))
    }
}

/// An iterator adaptor that merges the two base iterators in ascending order.
/// If both base iterators are sorted (ascending), the result is sorted.
///
/// Iterator element type is **I::Item**.
pub struct Merge<I, J> where
    I: Iterator,
    J: Iterator<Item=I::Item>,
{
    a: Peekable<I>,
    b: Peekable<J>,
}

impl<I, J> Merge<I, J> where
    I: Iterator,
    J: Iterator<Item=I::Item>,
{
    /// Create a **Merge** iterator.
    pub fn new(a: I, b: J) -> Self
    {
        Merge {
            a: a.peekable(),
            b: b.peekable(),
        }
    }
}

impl<I, J> Iterator for Merge<I, J> where
    I: Iterator,
    I::Item: PartialOrd,
    J: Iterator<Item=I::Item>,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<I::Item> {
        if match (self.a.peek(), self.b.peek()) {
            (Some(a), Some(b)) => a <= b,
            (Some(_), None) => true,
            (None, Some(_)) => false,
            (None, None) => return None,
        } {
            self.a.next()
        } else {
            self.b.next()
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let (a_min, a_max) = self.a.size_hint();
        let (b_min, b_max) = self.b.size_hint();

        let min = a_min.checked_add(b_min).unwrap_or(usize::MAX);
        let max = match (a_max, b_max) {
            (Some(a_min), Some(b_min)) => a_min.checked_add(b_min),
            _ => None,
        };

        (min, max)
    }
}

#[cfg(feature = "unstable")]
/// An iterator adaptor that enumerates the iterator elements,
/// with a custom starting value and integer type.
pub struct EnumerateFrom<I, K>
{
    index: K,
    iter: I,
}

#[cfg(feature = "unstable")]
impl<K, I> EnumerateFrom<I, K> where
    I: Iterator,
{
    pub fn new(iter: I, start: K) -> Self
    {
        EnumerateFrom{index: start, iter: iter}
    }
}

#[cfg(feature = "unstable")]
impl<K, I> Iterator for EnumerateFrom<I, K> where
    K: Copy + One + Add<Output=K>,
    I: Iterator,
{
    type Item = (K, I::Item);
    fn next(&mut self) -> Option<(K, I::Item)>
    {
        match self.iter.next() {
            None => None,
            Some(elt) => {
                let index = self.index.clone();
                // FIXME: Arithmetic needs to be wrapping here to be sane,
                // imagine i8 counter to enumerate a sequence 0 to 127 inclusive.
                self.index = self.index + K::one();
                Some((index, elt))
            }
        }
    }
}

#[derive(Clone)]
/// An Iterator adaptor that allows the user to peek at multiple *.next()* values without advancing itself.
pub struct MultiPeek<I> where
    I: Iterator,
{
    iter: Fuse<I>,
    buf: Vec<I::Item>,
    index: usize,
}

impl<I: Iterator> MultiPeek<I> {
    /// Create a **MultiPeek** iterator.
    pub fn new(iter: I) -> MultiPeek<I> {
        MultiPeek{ iter: iter.fuse(), buf: Vec::new(), index: 0 }
    }

    /// Works exactly like *.next()* with the only difference that it doesn't advance itself.
    /// *.peek()* kann be called multiple times, behaving exactly like *.next()*.
    pub fn peek(&mut self) -> Option<&I::Item> {
        let ret = if self.index < self.buf.len() {
            Some(&self.buf[self.index])
        } else {
            match self.iter.next() {
                Some(x) => {
                    self.buf.push(x);
                    Some(&self.buf[self.index])
                }
                None => return None
            }
        };

        self.index += 1;
        ret
    }
}

impl<I> Iterator for MultiPeek<I> where
    I: Iterator,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<I::Item> {
        self.index = 0;
        if self.buf.is_empty() {
            self.iter.next()
        } else {
            Some(self.buf.remove(0))
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) { self.iter.size_hint() }
}
