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
pub struct EmptyCollectionIter<'a, T> {
    _phantom: std::marker::PhantomData<&'a T>,
}

impl<E> EmptyCollectionIter<'_, E> {
    pub fn new() -> Self {
        EmptyCollectionIter {
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<E> Default for EmptyCollectionIter<'_, E> {
    fn default() -> Self {
        EmptyCollectionIter {
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<'a, E> Iterator for EmptyCollectionIter<'a, E> {
    type Item = &'a E;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

/// A lazy iterator for empty collection.
pub struct EmptyCollectionIterMut<'a, T> {
    _phantom: std::marker::PhantomData<&'a T>,
}

impl<E> EmptyCollectionIterMut<'_, E> {
    pub fn new() -> Self {
        EmptyCollectionIterMut {
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<E> Default for EmptyCollectionIterMut<'_, E> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, E> Iterator for EmptyCollectionIterMut<'a, E> {
    type Item = &'a mut E;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

/// A lazy iterator for empty collection.
pub struct EmptyCollectionIterLazy<T> {
    _phantom: std::marker::PhantomData<T>,
}

impl<E> EmptyCollectionIterLazy<E> {
    pub fn new() -> Self {
        EmptyCollectionIterLazy {
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<E> Default for EmptyCollectionIterLazy<E> {
    fn default() -> Self {
        Self::new()
    }
}

impl<E> Iterator for EmptyCollectionIterLazy<E> {
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
        = EmptyCollectionIter<'a, E>
    where
        Self: 'a;

    fn start(&self) -> Self::Position {}

    fn end(&self) -> Self::Position {}

    fn form_next(&self, _: &mut Self::Position) {}

    fn form_next_n(&self, _: &mut Self::Position, _: usize) {}

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

    fn iter_pos(&self, _: Self::Position, _: Self::Position) -> Self::Iter<'_> {
        EmptyCollectionIter::new()
    }

    fn iter(&self) -> Self::Iter<'_> {
        EmptyCollectionIter::new()
    }
}

impl<E> BidirectionalCollection for EmptyCollection<E> {
    fn form_prior(&self, _: &mut Self::Position) {}

    fn form_prior_n(&self, _: &mut Self::Position, _: usize) {}
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
        = EmptyCollectionIterMut<'a, E>
    where
        Self: 'a;

    fn at_mut(&mut self, _: &Self::Position) -> &mut Self::Element {
        panic!("Out of bound access")
    }

    fn iter_mut_pos(
        &mut self,
        _: Self::Position,
        _: Self::Position,
    ) -> Self::IterMut<'_> {
        EmptyCollectionIterMut::new()
    }

    fn iter_mut(&mut self) -> Self::IterMut<'_> {
        EmptyCollectionIterMut::new()
    }
}

impl<E> LazyCollection for EmptyCollection<E> {
    type LazyIter<'a>
        = EmptyCollectionIterLazy<E>
    where
        Self: 'a;

    fn compute_at(&self, _: &Self::Position) -> Self::Element {
        panic!("Out of bound access")
    }

    fn lazy_iter_pos(
        &self,
        _: Self::Position,
        _: Self::Position,
    ) -> Self::LazyIter<'_> {
        EmptyCollectionIterLazy::new()
    }

    fn lazy_iter(&self) -> Self::LazyIter<'_> {
        EmptyCollectionIterLazy::new()
    }
}
