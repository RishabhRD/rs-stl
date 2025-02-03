// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

#[cfg(test)]
pub mod tests {
    use stl::*;

    fn bidirectional_access() {
        let arr = [1, 2, 3];
        let slice = ArraySlice::new(&arr, 1, 3);
        let prev = slice.before(slice.end());
        assert_eq!(prev, 2);
    }
}
