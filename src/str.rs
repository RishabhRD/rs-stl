// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

#![doc(hidden)]

use crate::{util::ValueRef, BoundedRange, ForwardRange, InputRange};

// This comes from UTF-8 standard.
fn char_len_at(s: &str, i: usize) -> i32 {
    match s.as_bytes()[i] {
        0x00..=0x7F => 1,
        0xC0..=0xDF => 2,
        0xE0..=0xEF => 3,
        0xF0..=0xF7 => 4,
        _ => 0,
    }
}

impl InputRange for str {
    type Element = char;

    type Position = usize;

    type ElementRef<'a>
        = ValueRef<char>
    where
        Self: 'a;

    fn start(&self) -> Self::Position {
        0
    }

    fn is_end(&self, i: &Self::Position) -> bool {
        *i == self.len()
    }

    fn after(&self, i: Self::Position) -> Self::Position {
        let len = char_len_at(self, i);
        i + len as usize
    }

    fn at<'a>(&'a self, i: &Self::Position) -> Self::ElementRef<'a> {
        let len = char_len_at(self, *i) as usize;
        let bytes = self.as_bytes();
        let val = unsafe { std::str::from_utf8_unchecked(&bytes[*i..i + len]) }
            .chars()
            .next()
            .unwrap();
        ValueRef { val }
    }
}

impl ForwardRange for str {}

impl BoundedRange for str {
    fn end(&self) -> Self::Position {
        self.len()
    }
}

impl InputRange for String {
    type Element = char;

    type Position = usize;

    type ElementRef<'a>
        = ValueRef<char>
    where
        Self: 'a;

    fn start(&self) -> Self::Position {
        0
    }

    fn is_end(&self, i: &Self::Position) -> bool {
        *i == self.len()
    }

    fn after(&self, i: Self::Position) -> Self::Position {
        let len = char_len_at(self, i);
        i + len as usize
    }

    fn at<'a>(&'a self, i: &Self::Position) -> Self::ElementRef<'a> {
        let len = char_len_at(self, *i) as usize;
        let bytes = self.as_bytes();
        let val = unsafe { std::str::from_utf8_unchecked(&bytes[*i..i + len]) }
            .chars()
            .next()
            .unwrap();
        ValueRef { val }
    }
}

impl ForwardRange for String {}

impl BoundedRange for String {
    fn end(&self) -> Self::Position {
        self.len()
    }
}
