// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{Collection, CollectionBase, CollectionLifetime};

pub struct ArraySlice<'a, T> {
    m_slice: &'a [T],
    m_start: usize,
    m_end: usize,
}

pub struct MutableArraySlice<'a, T> {
    m_slice: &'a mut [T],
    m_start: usize,
    m_end: usize,
}

impl<'a, T> ArraySlice<'a, T> {
    pub fn new(slice: &'a [T], start: usize, end: usize) -> Self {
        ArraySlice {
            m_slice: slice,
            m_start: start,
            m_end: end,
        }
    }
}

impl<'a, T> MutableArraySlice<'a, T> {
    pub fn new(slice: &'a mut [T], start: usize, end: usize) -> Self {
        MutableArraySlice {
            m_slice: slice,
            m_start: start,
            m_end: end,
        }
    }
}

impl<T> CollectionBase for MutableArraySlice<'_, T> {
    type Position = usize;
}

impl<'a, T> CollectionLifetime<'a> for MutableArraySlice<'_, T> {
    type Element = &'a T;
    type Slice = MutableArraySlice<'a, T>;
}

impl<T> Collection for MutableArraySlice<'_, T> {
    fn at<'a>(
        &'a self,
        i: &Self::Position,
    ) -> <Self as CollectionLifetime<'a>>::Element
    where
        Self: 'a,
    {
        &self.m_slice[*i]
    }

    fn slice<'a>(
        &'a mut self,
        from: Self::Position,
        to: Self::Position,
    ) -> <Self as CollectionLifetime<'a>>::Slice
    where
        Self: 'a,
    {
        MutableArraySlice::new(self.m_slice, from, to)
    }
}
