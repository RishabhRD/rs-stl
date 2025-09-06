// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

#[cfg(test)]
pub mod tests {
    use stl::*;

    #[test]
    fn test_the_equality() {
        let arr = [1, 2, 3];
        assert!(arr.reversed().equals(&[3, 2, 1]));
        let arr = [1, 2];
        assert!(arr.reversed().equals(&[2, 1]));
        let arr = [1];
        assert!(arr.reversed().equals(&[1]));
        let arr: [i32; 0] = [];
        assert!(arr.reversed().equals(&[]));
    }

    #[test]
    fn slicing() {
        let arr = [1, 2, 3, 4, 5].reversed();
        assert!(arr.prefix(3).equals(&[5, 4, 3]));
        assert!(arr.suffix(3).equals(&[3, 2, 1]));
        assert!(arr.prefix(7).equals(&[5, 4, 3, 2, 1]));
        assert!(arr.suffix(7).equals(&[5, 4, 3, 2, 1]));
        assert!(arr.prefix(0).equals(&[]));
        assert!(arr.suffix(0).equals(&[]));
    }

    #[test]
    fn position_ordering() {
        let arr = [1, 2, 3, 4, 5].reversed();
        assert!(arr.start() < arr.end());
        assert!(arr.start() == arr.start());
        assert!(arr.end() > arr.start());
    }

    #[test]
    fn reordering() {
        let mut arr = [1, 2, 3, 4, 5].reversed();
        arr.swap_at(&arr.start(), &arr.prior(arr.end()));
        assert!(arr.equals(&[1, 4, 3, 2, 5]));
    }

    #[test]
    fn element_mutation() {
        let mut arr = [1, 2, 3, 4, 5].reversed();
        let i = arr.start();
        *arr.at_mut(&i) = 3;
        assert!(arr.equals(&[3, 4, 3, 2, 1]));
    }
}
