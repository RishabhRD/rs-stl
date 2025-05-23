// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

#[cfg(test)]
pub mod tests {
    use stl::*;

    #[test]
    fn for_each() {
        let mut sum = 0;
        let arr = [1, 2, 3];
        arr.for_each(|e| sum += e);
        assert_eq!(sum, 6);
    }

    #[test]
    fn for_each_mut() {
        let mut arr = [1, 2, 3];
        arr.for_each_mut(|e| *e += 1);
        assert_eq!(arr, [2, 3, 4]);
    }

    #[test]
    fn lazy_for_each() {
        let mut sum = 0;
        let arr = 1..=3;
        arr.lazy_for_each(|e| sum += e);
        assert_eq!(sum, 6);
    }
}
