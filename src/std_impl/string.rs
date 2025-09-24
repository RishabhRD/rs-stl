// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{value_ref::ValueRef, Collection, LazyCollection, Slice};

// Returns the length of `i`th char in terms of bytes.
fn char_len_at(s: &str, i: usize) -> usize {
    match s.as_bytes()[i] {
        0x00..=0x7F => 1,
        0xC0..=0xDF => 2,
        0xE0..=0xEF => 3,
        0xF0..=0xF7 => 4,
        _ => 0,
    }
}

impl Collection for String {
    type Position = usize;

    type Element = char;

    type ElementRef<'a>
        = ValueRef<char>
    where
        Self: 'a;

    type Whole = Self;

    fn start(&self) -> Self::Position {
        0
    }

    fn end(&self) -> Self::Position {
        self.len()
    }

    fn form_next(&self, position: &mut Self::Position) {
        let len = char_len_at(self, *position);
        *position += len
    }

    fn at(&self, i: &Self::Position) -> Self::ElementRef<'_> {
        let len = char_len_at(self, *i);
        let bytes = self.as_bytes();
        let val = unsafe { std::str::from_utf8_unchecked(&bytes[*i..i + len]) }
            .chars()
            .next()
            .unwrap();
        ValueRef { val }
    }

    fn slice(
        &self,
        from: Self::Position,
        to: Self::Position,
    ) -> crate::Slice<'_, Self::Whole> {
        Slice::new(self, from, to)
    }
}

impl LazyCollection for String {
    fn compute_at(&self, i: &Self::Position) -> Self::Element {
        self.at(i).val
    }
}

impl Collection for &str {
    type Position = usize;

    type Element = char;

    type ElementRef<'a>
        = ValueRef<char>
    where
        Self: 'a;

    type Whole = Self;

    fn start(&self) -> Self::Position {
        0
    }

    fn end(&self) -> Self::Position {
        self.len()
    }

    fn form_next(&self, position: &mut Self::Position) {
        let len = char_len_at(self, *position);
        *position += len
    }

    fn at(&self, i: &Self::Position) -> Self::ElementRef<'_> {
        let len = char_len_at(self, *i);
        let bytes = self.as_bytes();
        let val = unsafe { std::str::from_utf8_unchecked(&bytes[*i..i + len]) }
            .chars()
            .next()
            .unwrap();
        ValueRef { val }
    }

    fn slice(
        &self,
        from: Self::Position,
        to: Self::Position,
    ) -> crate::Slice<'_, Self::Whole> {
        Slice::new(self, from, to)
    }
}

impl LazyCollection for &str {
    fn compute_at(&self, i: &Self::Position) -> Self::Element {
        self.at(i).val
    }
}
