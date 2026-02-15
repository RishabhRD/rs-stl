// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

#[doc(hidden)]
pub mod empty;
#[doc(inline)]
pub use empty::EmptyCollection;

pub mod reversed;
#[doc(inline)]
pub use reversed::ReversedCollection;
