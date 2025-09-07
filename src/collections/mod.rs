// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

#[doc(hidden)]
pub mod empty;
#[doc(inline)]
pub use empty::EmptyCollection;

#[doc(hidden)]
pub mod singleton;
#[doc(inline)]
pub use singleton::SingletonCollection;

pub mod reversed;
#[doc(inline)]
pub use reversed::ReversedCollection;

#[doc(hidden)]
pub mod mapped;
#[doc(inline)]
pub use mapped::*;
