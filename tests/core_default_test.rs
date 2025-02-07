// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

#[cfg(test)]
pub mod tests {
    use stl::*;

    struct Array<T, const N: usize>([T; N]);

    impl<'this, T: 'this, const N: usize> Collection<'this> for Array<T, N> {
        type Position = usize;

        type Element = &'this T;

        type MutableElement = &'this mut T;

        type Slice = ArraySlice<'this, T>;

        type MutableSlice = MutableArraySlice<'this, T>;

        fn start(&self) -> Self::Position {
            0
        }

        fn end(&self) -> Self::Position {
            N
        }

        fn after(&self, i: Self::Position) -> Self::Position {
            i + 1
        }

        fn distance(&self, from: Self::Position, to: Self::Position) -> usize {
            to - from
        }

        fn slice<'a, Callback, ReturnType>(
            &'a self,
            from: Self::Position,
            to: Self::Position,
            mut callback: Callback,
        ) -> ReturnType
        where
            Callback: FnMut(&Self::Slice) -> ReturnType,
            'a: 'this,
        {
            let slice = ArraySlice::new(&self.0, from, to);
            callback(&slice)
        }

        fn at<'a, Callback, ReturnType>(
            &'a self,
            i: &Self::Position,
            mut callback: Callback,
        ) -> ReturnType
        where
            Callback: FnMut(Self::Element) -> ReturnType,
            'a: 'this,
        {
            callback(&self.0[*i])
        }
    }

    impl<'this, T: 'this, const N: usize> BidirectionalCollection<'this>
        for Array<T, N>
    {
        fn before(&self, i: Self::Position) -> Self::Position {
            i - 1
        }

        fn before_n(&self, i: Self::Position, n: usize) -> Self::Position {
            i - n
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
        arr.at(&0, |e| assert_eq!(*e, 10));
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
