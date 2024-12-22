// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

#[cfg(test)]
pub mod tests {
    use rng::infix::*;
    use stl::*;

    #[test]
    fn sort_range() {
        let mut arr = [2, 4, 4, 1, 2, 5];
        let start = arr.start();
        let end = arr.end();
        algo::sort_range_by(&mut arr, start, end, |x, y| y < x);
        assert!(arr.equals(&[5, 4, 4, 2, 2, 1]));

        let mut arr = [2, 4, 4, 1, 2, 5];
        rng::sort_range_by(&mut arr, |x, y| y < x);
        assert!(arr.equals(&[5, 4, 4, 2, 2, 1]));

        let mut arr = [2, 4, 4, 1, 2, 5];
        arr.sort_range_by(|x, y| y < x);
        assert!(arr.equals(&[5, 4, 4, 2, 2, 1]));

        let mut arr = [2, 4, 4, 1, 2, 5];
        let start = arr.start();
        let end = arr.end();
        algo::sort_range(&mut arr, start, end);
        assert!(arr.equals(&[1, 2, 2, 4, 4, 5]));

        let mut arr = [2, 4, 4, 1, 2, 5];
        rng::sort_range(&mut arr);
        assert!(arr.equals(&[1, 2, 2, 4, 4, 5]));

        let mut arr = [2, 4, 4, 1, 2, 5];
        arr.sort_range();
        assert!(arr.equals(&[1, 2, 2, 4, 4, 5]));

        let mut arr: [i32; 0] = [];
        arr.sort_range();
        assert!(arr.equals(&[]));
    }
}
