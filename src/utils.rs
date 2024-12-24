// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

/// Creates a Vec<T> of size n, with all elements uninitialized.
///
/// NOTE: This is an unsafe construct, and should be used carefully for
/// real performance reasons.
pub fn create_raw_buffer<T>(n: usize) -> Vec<T> {
    let mut vec = Vec::with_capacity(n);
    unsafe {
        vec.set_len(n);
    }
    vec
}
