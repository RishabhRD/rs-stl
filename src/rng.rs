// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

pub mod find;
pub use find::*;

pub mod count;
pub use count::*;

pub mod of;
pub use of::*;

pub mod mismatch;
pub use mismatch::*;

pub mod infix {
    pub use super::count::infix::*;
    pub use super::find::infix::*;
    pub use super::of::infix::*;
}
