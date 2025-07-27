// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{
    BidirectionalCollection, Collection, MutableCollection,
    RandomAccessCollection, ReorderableCollection, Slice, SliceMut,
};

/// A collection of one element.
pub struct SingletonCollection<E> {
    element: E,
}

impl<E> SingletonCollection<E> {
    pub fn new(element: E) -> Self {
        SingletonCollection { element }
    }
}

/// Iterator for `CollectionOfOne`.
pub enum Iter<'a, E> {
    First(&'a E),
    Last,
}

impl<'a, E> Iterator for Iter<'a, E> {
    type Item = &'a E;

    fn next(&mut self) -> Option<Self::Item> {
        match std::mem::replace(self, Iter::Last) {
            Iter::First(e) => Some(e),
            Iter::Last => None,
        }
    }
}

/// Mutable Iterator for `CollectionOfOne`.
pub enum IterMut<'a, E> {
    First(&'a mut E),
    Last,
}

impl<'a, E> Iterator for IterMut<'a, E> {
    type Item = &'a mut E;

    fn next(&mut self) -> Option<Self::Item> {
        match std::mem::replace(self, IterMut::Last) {
            IterMut::First(e) => Some(e),
            IterMut::Last => None,
        }
    }
}

impl<E> Collection for SingletonCollection<E> {
    type Position = bool;

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
            Iter::Last
        } else {
            Iter::First(&self.element)
        }
    }

    fn iter(&self) -> Self::Iter<'_> {
        Iter::First(&self.element)
    }
}

impl<E> BidirectionalCollection for SingletonCollection<E> {
    fn form_prior(&self, position: &mut Self::Position) {
        *position = false
    }

    fn form_prior_n(&self, position: &mut Self::Position, n: usize) {
        if n != 0 {
            *position = false
        }
    }
}

impl<E> RandomAccessCollection for SingletonCollection<E> {}

impl<E> ReorderableCollection for SingletonCollection<E> {
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

impl<E> MutableCollection for SingletonCollection<E> {
    type IterMut<'a>
        = IterMut<'a, E>
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
            IterMut::Last
        } else {
            IterMut::First(&mut self.element)
        }
    }

    fn iter_mut(&mut self) -> Self::IterMut<'_> {
        IterMut::First(&mut self.element)
    }
}
