// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use crate::{
    value_ref::ValueRef, Collection, LazyCollection, Slice, UnsafeSubSequence,
};

// Returns the length of character for character of type `char_type`.
fn char_len_for(char_type: u8) -> usize {
    match char_type {
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

    type SubSequence = StringSlice;

    type MutableSubSequence = StringSlice;

    fn start(&self) -> Self::Position {
        0
    }

    fn end(&self) -> Self::Position {
        self.len()
    }

    fn form_next(&self, i: &mut Self::Position) {
        let len = char_len_for(self.as_bytes()[*i]);
        *i += len
    }

    fn at(&self, i: &Self::Position) -> Self::ElementRef<'_> {
        let len = char_len_for(self.as_bytes()[*i]);
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
    ) -> crate::Slice<'_, Self::SubSequence> {
        let s = StringSlice {
            start_address: self.as_ptr(),
            start_position: from,
            end_position: to,
        };
        unsafe { Slice::new(s.unsafe_slice(from, to)) }
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

    type SubSequence = StringSlice;

    type MutableSubSequence = StringSlice;

    fn start(&self) -> Self::Position {
        0
    }

    fn end(&self) -> Self::Position {
        self.len()
    }

    fn form_next(&self, i: &mut Self::Position) {
        let len = char_len_for(self.as_bytes()[*i]);
        *i += len
    }

    fn at(&self, i: &Self::Position) -> Self::ElementRef<'_> {
        let len = char_len_for(self.as_bytes()[*i]);
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
    ) -> crate::Slice<'_, Self::SubSequence> {
        let s = StringSlice {
            start_address: self.as_ptr(),
            start_position: from,
            end_position: to,
        };
        unsafe { Slice::new(s.unsafe_slice(from, to)) }
    }
}

impl LazyCollection for &str {
    fn compute_at(&self, i: &Self::Position) -> Self::Element {
        self.at(i).val
    }
}

/// Unsafe slice for string.
struct StringSlice {
    start_address: *const u8,

    start_position: usize,

    end_position: usize,
}

impl Collection for StringSlice {
    type Position = usize;

    type Element = char;

    type ElementRef<'a>
        = ValueRef<char>
    where
        Self: 'a;

    type SubSequence = StringSlice;

    type MutableSubSequence = StringSlice;

    fn start(&self) -> Self::Position {
        self.start_position
    }

    fn end(&self) -> Self::Position {
        self.end_position
    }

    fn form_next(&self, i: &mut Self::Position) {
        let len = unsafe {
            char_len_for(*self.start_address.add(*i).as_ref_unchecked())
        };
        *i += len
    }

    fn at(&self, i: &Self::Position) -> Self::ElementRef<'_> {
        unsafe { self.unsafe_at(i) }
    }

    fn slice(
        &self,
        from: Self::Position,
        to: Self::Position,
    ) -> crate::Slice<'_, Self::SubSequence> {
        unsafe { Slice::new(self.unsafe_slice(from, to)) }
    }
}

impl LazyCollection for StringSlice {
    fn compute_at(&self, i: &Self::Position) -> Self::Element {
        self.at(i).val
    }
}

impl UnsafeSubSequence for StringSlice {
    unsafe fn unsafe_at<'a>(&self, i: &Self::Position) -> Self::ElementRef<'a> {
        let p = unsafe { self.start_address.add(*i) };
        let l = unsafe { char_len_for(*p.as_ref_unchecked()) };
        let r = unsafe {
            str::from_utf8_unchecked(std::slice::from_raw_parts(p, l))
        }
        .chars()
        .next()
        .unwrap();
        ValueRef { val: r }
    }

    unsafe fn unsafe_slice(
        &self,
        from: Self::Position,
        to: Self::Position,
    ) -> Self::SubSequence {
        StringSlice {
            start_address: self.start_address,
            start_position: from,
            end_position: to,
        }
    }

    unsafe fn set_start(&mut self, p: Self::Position) {
        self.start_position = p
    }

    unsafe fn set_end(&mut self, p: Self::Position) {
        self.end_position = p
    }
}
