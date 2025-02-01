// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

#[cfg(test)]
pub mod tests {
    use stl::*;

    struct Array<T, const N: usize>([T; N]);

    impl<T, const N: usize> Range for Array<T, N> {
        type Position = usize;

        type Element = T;

        fn start(&self) -> Self::Position {
            0
        }

        fn end(&self) -> Self::Position {
            N
        }

        fn after(&self, i: Self::Position) -> Self::Position {
            assert!(i != N);
            i + 1
        }

        fn at_as_deref(
            &self,
            i: &Self::Position,
        ) -> impl std::ops::Deref<Target = Self::Element> {
            self.at(i)
        }
    }

    impl<T, const N: usize> Collection for Array<T, N> {
        fn at(&self, i: &Self::Position) -> &T {
            assert!(*i != N);
            &self.0[*i]
        }
    }

    impl<T, const N: usize> BidirectionalRange for Array<T, N> {
        fn before(&self, i: Self::Position) -> Self::Position {
            assert!(i > 0);
            i - 1
        }
    }

    #[test]
    fn start() {
        let arr = Array([10, 20, 30]);
        assert_eq!(arr.start(), 0);
    }

    #[test]
    fn end() {
        let arr = Array([10, 20, 30]);
        assert_eq!(arr.end(), 3);
    }

    #[test]
    fn after() {
        let arr = Array([10, 20, 30]);
        assert_eq!(arr.after(0), 1);
    }

    #[test]
    fn after_n() {
        let arr = Array([10, 20, 30]);
        assert_eq!(arr.after_n(0, 2), 2);
    }

    #[test]
    fn before() {
        let arr = Array([10, 20, 30]);
        assert_eq!(arr.before(1), 0);
    }

    #[test]
    fn before_n() {
        let arr = Array([10, 20, 30]);
        assert_eq!(arr.before_n(2, 2), 0);
    }

    #[test]
    fn distance() {
        let arr = Array([10, 20, 30]);
        assert_eq!(arr.distance(0, 2), 2);
    }

    #[test]
    fn at() {
        let arr = Array([10, 20, 30]);
        assert_eq!(*arr.at(&0), 10);
    }

    #[test]
    fn lifetime() {
        let start = Array([1, 2, 3]).start(); // Positions can outlive ranges.
        assert_eq!(start, 0);
    }

    #[test]
    fn iter() {
        let arr = [1, 2, 3];
        let mut sum = 0;
        for n in arr.iter() {
            sum += n;
        }
        assert_eq!(sum, 6);
    }

    // TODO: add test for iter for lazy collection.
}
