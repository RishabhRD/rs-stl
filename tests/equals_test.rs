// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

#[cfg(test)]
pub mod tests {
    use stl::*;

    #[test]
    fn should_not_equal_on_different_length() {
        assert!(![1, 2, 3].equals(&[1, 2]));
        assert!(![1, 2].equals(&[1, 2, 3]));
        assert!(![].equals(&[1, 2]));
        assert!(![1, 2].equals(&[]));
    }

    #[test]
    fn should_not_equal_on_different_content() {
        assert!(![1, 2].equals(&[2, 3]));
    }

    #[test]
    fn should_equal_if_both_are_empty() {
        let arr1 = [];
        let arr2: [i32; 0] = [];
        assert!(arr1.equals(&arr2));
    }

    #[test]
    fn should_equal_if_both_have_same_content() {
        assert!([1, 2, 3].equals(&[1, 2, 3]));
        assert!(vec![1, 2, 3].equals(&[1, 2, 3])); // different types
    }

    #[test]
    fn equals_by() {
        assert!([1, 2, 3].equals_by(&[2, 3, 4], |x, y| *y == *x + 1));
    }
}
