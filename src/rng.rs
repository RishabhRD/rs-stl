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

pub mod adjacent_find;
pub use adjacent_find::*;

pub mod equals;
pub use equals::*;

pub mod copy;
pub use copy::*;

pub mod transform;
pub use transform::*;

pub mod replace;
pub use replace::*;

pub mod fill;
pub use fill::*;

pub mod remove;
pub use remove::*;

pub mod unique;
pub use unique::*;

pub mod reverse;
pub use reverse::*;

pub mod infix {
    pub use super::adjacent_find::infix::*;
    pub use super::count::infix::*;
    pub use super::equals::infix::*;
    pub use super::fill::infix::*;
    pub use super::find::infix::*;
    pub use super::of::infix::*;
    pub use super::remove::infix::*;
    pub use super::replace::infix::*;
    pub use super::reverse::infix::*;
    pub use super::unique::infix::*;
}
