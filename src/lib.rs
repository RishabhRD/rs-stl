// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

pub mod ranges;
pub mod regular;

pub use ranges::*;

pub fn add(a: u64, b: u64) -> u64 {
    a + b
}
