// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

#[cfg(test)]
pub mod tests {
    use stl::*;

    #[test]
    fn reverse_empty() {
        let mut arr: [i32; 0] = [];
        BidirectionalCollectionExt::reverse(&mut arr);
        assert_eq!(arr, []);
    }

    #[test]
    fn reverse_singleton() {
        let mut arr = [1];
        BidirectionalCollectionExt::reverse(&mut arr);
        assert_eq!(arr, [1]);
    }

    #[test]
    fn reverse_twos() {
        let mut arr = [1, 2];
        BidirectionalCollectionExt::reverse(&mut arr);
        assert_eq!(arr, [2, 1]);
    }

    #[test]
    fn reverse_odd() {
        let mut arr = [1, 2, 3];
        BidirectionalCollectionExt::reverse(&mut arr);
        assert_eq!(arr, [3, 2, 1]);
    }

    #[test]
    fn reverse_even() {
        let mut arr = [1, 2, 3, 4];
        BidirectionalCollectionExt::reverse(&mut arr);
        assert_eq!(arr, [4, 3, 2, 1]);
    }
}
