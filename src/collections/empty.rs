// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{
    BidirectionalCollection, Collection, LazyCollection, MutableCollection,
    RandomAccessCollection, ReorderableCollection, Slice, SliceMut,
};

/// An empty collection.
pub struct EmptyCollection<E> {
    phantom: std::marker::PhantomData<E>,
}

impl<E> EmptyCollection<E> {
    pub fn new() -> Self {
        EmptyCollection {
            phantom: std::marker::PhantomData,
        }
    }
}

impl<E> Default for EmptyCollection<E> {
    fn default() -> Self {
        Self::new()
    }
}

/// An iterator for empty collection.
pub struct Iter<'a, T> {
    _phantom: std::marker::PhantomData<&'a T>,
}

impl<E> Iter<'_, E> {
    pub fn new() -> Self {
        Iter {
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<E> Default for Iter<'_, E> {
    fn default() -> Self {
        Iter {
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<'a, E> Iterator for Iter<'a, E> {
    type Item = &'a E;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

/// A mutable iterator for empty collection.
pub struct IterMut<'a, T> {
    _phantom: std::marker::PhantomData<&'a T>,
}

impl<E> IterMut<'_, E> {
    pub fn new() -> Self {
        IterMut {
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<E> Default for IterMut<'_, E> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, E> Iterator for IterMut<'a, E> {
    type Item = &'a mut E;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

/// A lazy iterator for empty collection.
pub struct LazyIter<T> {
    _phantom: std::marker::PhantomData<T>,
}

impl<E> LazyIter<E> {
    pub fn new() -> Self {
        LazyIter {
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<E> Default for LazyIter<E> {
    fn default() -> Self {
        Self::new()
    }
}

impl<E> Iterator for LazyIter<E> {
    type Item = E;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

impl<E> Collection for EmptyCollection<E> {
    type Position = ();

    type Element = E;

    type ElementRef<'a>
        = &'a E
    where
        Self: 'a;

    type Whole = Self;

    type Iter<'a>
        = Iter<'a, E>
    where
        Self: 'a;

    fn start(&self) -> Self::Position {}

    fn end(&self) -> Self::Position {}

    fn form_next(&self, _: &mut Self::Position) {}

    fn form_next_n(&self, _: &mut Self::Position, _: usize) {}

    fn form_next_n_limited_by(
        &self,
        _: &mut Self::Position,
        n: usize,
        _: Self::Position,
    ) -> bool {
        n == 0
    }

    fn distance(&self, _: Self::Position, _: Self::Position) -> usize {
        0
    }

    fn count(&self) -> usize {
        0
    }

    fn underestimated_count(&self) -> usize {
        0
    }

    fn at(&self, _: &Self::Position) -> Self::ElementRef<'_> {
        panic!("Out of bound access")
    }

    fn slice(
        &self,
        from: Self::Position,
        to: Self::Position,
    ) -> crate::Slice<Self::Whole> {
        Slice::new(self, from, to)
    }

    fn iter_within(
        &self,
        _: Self::Position,
        _: Self::Position,
    ) -> Self::Iter<'_> {
        Iter::new()
    }

    fn iter(&self) -> Self::Iter<'_> {
        Iter::new()
    }
}

impl<E> BidirectionalCollection for EmptyCollection<E> {
    fn form_prior(&self, _: &mut Self::Position) {}

    fn form_prior_n(&self, _: &mut Self::Position, _: usize) {}

    fn form_prior_n_limited_by(
        &self,
        _: &mut Self::Position,
        n: usize,
        _: Self::Position,
    ) -> bool {
        n == 0
    }
}

impl<E> RandomAccessCollection for EmptyCollection<E> {}

impl<E> ReorderableCollection for EmptyCollection<E> {
    fn swap_at(&mut self, _: &Self::Position, _: &Self::Position) {
        panic!("Out of bound access")
    }

    fn slice_mut(
        &mut self,
        from: Self::Position,
        to: Self::Position,
    ) -> crate::SliceMut<Self::Whole> {
        SliceMut::new(self, from, to)
    }
}

impl<E> MutableCollection for EmptyCollection<E> {
    type IterMut<'a>
        = IterMut<'a, E>
    where
        Self: 'a;

    fn at_mut(&mut self, _: &Self::Position) -> &mut Self::Element {
        panic!("Out of bound access")
    }

    fn iter_mut_within(
        &mut self,
        _: Self::Position,
        _: Self::Position,
    ) -> Self::IterMut<'_> {
        IterMut::new()
    }

    fn iter_mut(&mut self) -> Self::IterMut<'_> {
        IterMut::new()
    }
}

impl<E> LazyCollection for EmptyCollection<E> {
    type LazyIter<'a>
        = LazyIter<E>
    where
        Self: 'a;

    fn compute_at(&self, _: &Self::Position) -> Self::Element {
        panic!("Out of bound access")
    }

    fn lazy_iter_within(
        &self,
        _: Self::Position,
        _: Self::Position,
    ) -> Self::LazyIter<'_> {
        LazyIter::new()
    }

    fn lazy_iter(&self) -> Self::LazyIter<'_> {
        LazyIter::new()
    }
}
