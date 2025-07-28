// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

use std::ops::Deref;

/// Proxy Reference to temporary value.
#[derive(Clone, Copy, Debug)]
pub struct ValueRef<T> {
    pub val: T,
}

impl<T> ValueRef<T> {
    pub fn new(val: T) -> Self {
        ValueRef { val }
    }
}

impl<T> Deref for ValueRef<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.val
    }
}

impl<T> AsRef<T> for ValueRef<T> {
    fn as_ref(&self) -> &T {
        &self.val
    }
}
