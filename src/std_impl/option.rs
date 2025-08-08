// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{
    BidirectionalCollection, Collection, MutableCollection,
    RandomAccessCollection, ReorderableCollection, Slice, SliceMut,
};

impl<T> Collection for Option<T> {
    type Position = bool;

    type Element = T;

    type ElementRef<'a>
        = &'a T
    where
        Self: 'a;

    type Whole = Self;

    type Iter<'a>
        = std::slice::Iter<'a, T>
    where
        Self: 'a;

    fn start(&self) -> Self::Position {
        !self.is_some()
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

    fn form_next_n_limited_by(
        &self,
        position: &mut Self::Position,
        n: usize,
        limit: Self::Position,
    ) -> bool {
        match (n, *position) {
            (0, _) => true,
            (_, p) if p == limit => false,
            _ => {
                *position = limit;
                n == 1
            }
        }
    }

    fn distance(&self, from: Self::Position, to: Self::Position) -> usize {
        if from == to {
            0
        } else {
            1
        }
    }

    fn count(&self) -> usize {
        if self.is_some() {
            1
        } else {
            0
        }
    }

    fn underestimated_count(&self) -> usize {
        self.count()
    }

    fn at(&self, i: &Self::Position) -> Self::ElementRef<'_> {
        if *i {
            panic!("Out of bounds access");
        }

        match self {
            Some(e) => e,
            None => panic!("Out of bounds access"),
        }
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
        from: Self::Position,
        to: Self::Position,
    ) -> Self::Iter<'_> {
        match (self, from, to) {
            (Some(value), false, true) => std::slice::from_ref(value).iter(),
            _ => [].iter(),
        }
    }
}

impl<T> BidirectionalCollection for Option<T> {
    fn form_prior(&self, position: &mut Self::Position) {
        *position = false
    }

    fn form_prior_n(&self, position: &mut Self::Position, n: usize) {
        if n != 0 {
            *position = false
        }
    }

    fn form_prior_n_limited_by(
        &self,
        position: &mut Self::Position,
        n: usize,
        limit: Self::Position,
    ) -> bool {
        match (n, *position) {
            (0, _) => true,
            (_, p) if p == limit => false,
            _ => {
                *position = limit;
                n == 1
            }
        }
    }
}

impl<T> RandomAccessCollection for Option<T> {}

impl<T> ReorderableCollection for Option<T> {
    fn swap_at(&mut self, i: &Self::Position, j: &Self::Position) {
        if *i || *j {
            panic!("Out of bounds access")
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

impl<T> MutableCollection for Option<T> {
    type IterMut<'a>
        = std::slice::IterMut<'a, T>
    where
        Self: 'a;

    fn at_mut(&mut self, i: &Self::Position) -> &mut Self::Element {
        if *i {
            panic!("Out of bounds access");
        }

        match self {
            Some(e) => e,
            None => panic!("Out of bounds access"),
        }
    }

    fn iter_mut_within(
        &mut self,
        from: Self::Position,
        to: Self::Position,
    ) -> Self::IterMut<'_> {
        match (self, from, to) {
            (Some(value), false, true) => {
                std::slice::from_mut(value).iter_mut()
            }
            _ => [].iter_mut(),
        }
    }
}
