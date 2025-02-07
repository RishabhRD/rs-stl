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

impl<T> CollectionBase for ArraySlice<'_, T> {
    type Position = usize;
}

impl<'this, T> CollectionLifetime<'_> for ArraySlice<'this, T> {
    type Element = &'this T;

    type MutableElement = &'this T;

    type Slice = ArraySlice<'this, T>;

    type MutableSlice = ArraySlice<'this, T>;
}

impl<T> Collection for ArraySlice<'_, T> {
    fn start(&self) -> Self::Position {
        self.m_start
    }

    fn end(&self) -> Self::Position {
        self.m_end
    }

    fn after(&self, i: Self::Position) -> Self::Position {
        i + 1
    }

    fn after_n(&self, i: Self::Position, n: usize) -> Self::Position {
        i + n
    }

    fn distance(&self, from: Self::Position, to: Self::Position) -> usize {
        to - from
    }

    fn slice<Callback, ReturnType>(
        &self,
        from: Self::Position,
        to: Self::Position,
        mut callback: Callback,
    ) -> ReturnType
    where
        Callback: FnMut(&<Self as CollectionLifetime<'_>>::Slice) -> ReturnType,
    {
        let slice = ArraySlice::new(self.m_slice, from, to);
        callback(&slice)
    }

    fn at<Callback, ReturnType>(
        &self,
        i: &Self::Position,
        mut callback: Callback,
    ) -> ReturnType
    where
        Callback:
            FnMut(<Self as CollectionLifetime<'_>>::Element) -> ReturnType,
    {
        callback(&self.m_slice[*i])
    }
}

impl<T> CollectionBase for MutableArraySlice<'_, T> {
    type Position = usize;
}

impl<'this, T> CollectionLifetime<'_> for MutableArraySlice<'this, T> {
    type Element = &'this T;

    type MutableElement = &'this mut T;

    type Slice = ArraySlice<'this, T>;

    type MutableSlice = MutableArraySlice<'this, T>;
}

impl<T> Collection for MutableArraySlice<'_, T> {
    fn start(&self) -> Self::Position {
        self.m_start
    }

    fn end(&self) -> Self::Position {
        self.m_end
    }

    fn after(&self, i: Self::Position) -> Self::Position {
        i + 1
    }

    fn after_n(&self, i: Self::Position, n: usize) -> Self::Position {
        i + n
    }

    fn distance(&self, from: Self::Position, to: Self::Position) -> usize {
        to - from
    }

    fn slice<Callback, ReturnType>(
        &self,
        from: Self::Position,
        to: Self::Position,
        mut callback: Callback,
    ) -> ReturnType
    where
        Callback: FnMut(&<Self as CollectionLifetime<'_>>::Slice) -> ReturnType,
    {
        let slice = ArraySlice::new(self.m_slice, from, to);
        callback(&slice)
    }

    fn at<Callback, ReturnType>(
        &self,
        i: &Self::Position,
        mut callback: Callback,
    ) -> ReturnType
    where
        Callback:
            FnMut(<Self as CollectionLifetime<'_>>::Element) -> ReturnType,
    {
        todo!()
    }
}
