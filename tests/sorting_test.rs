// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

#[cfg(test)]
pub mod tests {
    use stl::*;

    #[test]
    fn sort_unstable() {
        let mut arr = [4, 2, 1, 3];
        arr.sort_unstable();
        assert_eq!(arr, [1, 2, 3, 4]);

        let mut arr = [4];
        arr.sort_unstable();
        assert_eq!(arr, [4]);

        let mut arr: [i32; 0] = [];
        arr.sort_unstable();
        assert_eq!(arr, []);
    }
}
