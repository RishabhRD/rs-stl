// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

#[cfg(test)]
pub mod tests {
    use stl::*;

    #[test]
    fn when_first_half_is_empty() {
        let mut arr = [1, 2, 3];
        let i = arr.rotate(0);
        assert_eq!(i, 3);
        assert!(arr.equals(&[1, 2, 3]));
    }

    #[test]
    fn when_second_half_is_empty() {
        let mut arr = [1, 2, 3];
        let i = arr.rotate(3);
        assert_eq!(i, 0);
        assert!(arr.equals(&[1, 2, 3]));
    }

    #[test]
    fn when_first_half_is_bigger() {
        let mut arr = [1, 2, 3, 4, 5];
        let i = arr.rotate(3);
        assert_eq!(i, 2);
        assert!(arr.equals(&[4, 5, 1, 2, 3]));
    }

    #[test]
    fn when_first_half_is_smaller() {
        let mut arr = [1, 2, 3, 4, 5];
        let i = arr.rotate(2);
        assert_eq!(i, 3);
        assert!(arr.equals(&[3, 4, 5, 1, 2]));
    }

    #[test]
    fn when_both_halves_are_equal_length() {
        let mut arr = [1, 2, 3, 4, 5, 6];
        let i = arr.rotate(3);
        assert_eq!(i, 3);
        assert!(arr.equals(&[4, 5, 6, 1, 2, 3]));
    }
}
