// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

pub struct ValueRef<T> {
    pub val: T,
}

impl<T> std::ops::Deref for ValueRef<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.val
    }
}
