// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{
    BidirectionalCollection, Collection, MutableCollection,
    RandomAccessCollection, ReorderableCollection, Slice, SliceMut,
};

/// A collection of one element.
pub struct CollectionOfOne<E> {
    element: E,
}

impl<E> CollectionOfOne<E> {
    pub fn new(element: E) -> Self {
        CollectionOfOne { element }
    }
}

/// Iterator for `CollectionOfOne`.
pub enum CollectionOfOneIter<'a, E> {
    First(&'a E),
    Last,
}

impl<'a, E> Iterator for CollectionOfOneIter<'a, E> {
    type Item = &'a E;

    fn next(&mut self) -> Option<Self::Item> {
        match std::mem::replace(self, CollectionOfOneIter::Last) {
            CollectionOfOneIter::First(e) => Some(e),
            CollectionOfOneIter::Last => None,
        }
    }
}

/// Mutable Iterator for `CollectionOfOne`.
pub enum CollectionOfOneIterMut<'a, E> {
    First(&'a mut E),
    Last,
}

impl<'a, E> Iterator for CollectionOfOneIterMut<'a, E> {
    type Item = &'a mut E;

    fn next(&mut self) -> Option<Self::Item> {
        match std::mem::replace(self, CollectionOfOneIterMut::Last) {
            CollectionOfOneIterMut::First(e) => Some(e),
            CollectionOfOneIterMut::Last => None,
        }
    }
}

impl<E> Collection for CollectionOfOne<E> {
    type Position = bool;

    type Element = E;

    type ElementRef<'a>
        = &'a E
    where
        Self: 'a;

    type Whole = Self;

    type Iter<'a>
        = CollectionOfOneIter<'a, E>
    where
        Self: 'a;

    fn start(&self) -> Self::Position {
        false
    }

    fn end(&self) -> Self::Position {
        true
    }

    fn form_next(&self, position: &mut Self::Position) {
        *position = true
    }

    fn form_next_n(&self, position: &mut Self::Position, n: usize) {
        if n != 0 {
            *position = true
        }
    }

    fn distance(&self, from: Self::Position, to: Self::Position) -> usize {
        if from == to {
            return 0;
        }
        1
    }

    fn count(&self) -> usize {
        1
    }

    fn underestimated_count(&self) -> usize {
        1
    }

    fn at(&self, i: &Self::Position) -> Self::ElementRef<'_> {
        if *i {
            panic!("Out of bound access");
        }
        &self.element
    }

    fn slice(
        &self,
        from: Self::Position,
        to: Self::Position,
    ) -> crate::Slice<Self::Whole> {
        Slice::new(self, from, to)
    }

    fn iter_pos(
        &self,
        from: Self::Position,
        to: Self::Position,
    ) -> Self::Iter<'_> {
        if from == to {
            CollectionOfOneIter::Last
        } else {
            CollectionOfOneIter::First(&self.element)
        }
    }

    fn iter(&self) -> Self::Iter<'_> {
        CollectionOfOneIter::First(&self.element)
    }
}

impl<E> BidirectionalCollection for CollectionOfOne<E> {
    fn form_prior(&self, position: &mut Self::Position) {
        *position = false
    }

    fn form_prior_n(&self, position: &mut Self::Position, n: usize) {
        if n != 0 {
            *position = false
        }
    }
}

impl<E> RandomAccessCollection for CollectionOfOne<E> {}

impl<E> ReorderableCollection for CollectionOfOne<E> {
    fn swap_at(&mut self, i: &Self::Position, j: &Self::Position) {
        if *i || *j {
            panic!("Out of bound access");
        }
    }

    fn slice_mut(
        &mut self,
        from: Self::Position,
        to: Self::Position,
    ) -> crate::SliceMut<Self::Whole> {
        SliceMut::new(self, from, to)
    }
}

impl<E> MutableCollection for CollectionOfOne<E> {
    type IterMut<'a>
        = CollectionOfOneIterMut<'a, E>
    where
        Self: 'a;

    fn at_mut(&mut self, i: &Self::Position) -> &mut Self::Element {
        if *i {
            panic!("Out of bound access");
        }
        &mut self.element
    }

    fn iter_mut_pos(
        &mut self,
        from: Self::Position,
        to: Self::Position,
    ) -> Self::IterMut<'_> {
        if from == to {
            CollectionOfOneIterMut::Last
        } else {
            CollectionOfOneIterMut::First(&mut self.element)
        }
    }

    fn iter_mut(&mut self) -> Self::IterMut<'_> {
        CollectionOfOneIterMut::First(&mut self.element)
    }
}
