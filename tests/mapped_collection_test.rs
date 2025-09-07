// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

#[cfg(test)]
pub mod tests {
    use stl::*;

    #[test]
    fn mapping_equality() {
        let arr = CollectionExt::map([1, 2, 3, 4, 5], |x| x * 2);
        assert!(arr.equals(&[2, 4, 6, 8, 10]));
    }

    #[test]
    fn reordering() {
        let mut arr = CollectionExt::map([1, 2, 3, 4, 5], |x| x * 2);
        arr.reverse();
        assert!(arr.equals(&[10, 8, 6, 4, 2]));
    }
}
