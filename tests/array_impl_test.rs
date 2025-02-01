// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

#[cfg(test)]
pub mod tests {
    use stl::*;

    #[test]
    fn start() {
        let arr = [10, 20, 30];
        assert_eq!(arr.start(), 0);
    }

    #[test]
    fn end() {
        let arr = [10, 20, 30];
        assert_eq!(arr.end(), 3);
    }

    #[test]
    fn after() {
        let arr = [10, 20, 30];
        assert_eq!(arr.after(0), 1);
    }

    #[test]
    fn after_n() {
        let arr = [10, 20, 30];
        assert_eq!(arr.after_n(0, 2), 2);
    }

    #[test]
    fn before() {
        let arr = [10, 20, 30];
        assert_eq!(arr.before(1), 0);
    }

    #[test]
    fn before_n() {
        let arr = [10, 20, 30];
        assert_eq!(arr.before_n(2, 2), 0);
    }

    #[test]
    fn distance() {
        let arr = [10, 20, 30];
        assert_eq!(arr.distance(0, 2), 2);
    }

    #[test]
    fn at() {
        let arr = [10, 20, 30];
        assert_eq!(*arr.at(&0), 10);
    }

    #[test]
    fn lifetime() {
        let start = [1, 2, 3].start(); // Positions can outlive ranges.
        assert_eq!(start, 0);
    }

    #[test]
    fn swap_at() {
        let mut arr = [1, 2, 3];
        arr.swap_at(&0, &1);
        assert_eq!(arr, [2, 1, 3])
    }

    #[test]
    fn at_mut() {
        let mut arr = [1, 2, 3];
        *arr.at_mut(&0) = 2;
        assert_eq!(arr, [2, 2, 3])
    }
}
