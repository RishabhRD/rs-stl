// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

// As per Stepanov (not exact)
// Types can be:
//   - compared for equality
//   - moved
//   - destructed
pub trait SemiRegular: Eq {}
impl<T> SemiRegular for T where T: Eq {}

// As per Stepanov (not exact)
// Types is semiregular and can be copied.
// Copy should have equal value as of original object.
pub trait Regular: SemiRegular + Clone {}
impl<T> Regular for T where T: SemiRegular + Clone {}
