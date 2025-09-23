// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

/// Unwraps given `Vec<Option<T>>` as `Vec<T>` without any allocation.
///
/// # Precondition
///   - All elements of v is `Some`.
///
/// # Complexity
///   - O(1).
pub fn unwrap_option_vec<T>(mut v: Vec<Option<T>>) -> Vec<T> {
    let len = v.len();
    let capacity = v.capacity();
    let ptr = v.as_mut_ptr();
    std::mem::forget(v);
    unsafe { Vec::from_raw_parts(ptr as *mut T, len, capacity) }
}
