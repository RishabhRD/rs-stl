// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

#[cfg(test)]
pub mod tests {
    use rng::infix::*;
    use stl::algo::sort::*;
    use stl::*;

    #[test]
    fn insertion_sort_non_empty_range() {
        let mut arr = [(2, 4), (2, 3), (1, 2), (1, 3)];
        let start = arr.start();
        let end = arr.end();
        details::insertion_sort(&mut arr, start, end, |x, y| x.0 < y.0);
        assert!(arr.equals(&[(1, 2), (1, 3), (2, 4), (2, 3)]));
    }

    #[test]
    fn insertion_sort_empty_range() {
        let mut arr: [i32; 0] = [];
        let start = arr.start();
        let end = arr.end();
        details::insertion_sort(&mut arr, start, end, |x, y| x < y);
        assert!(arr.equals(&[]));
    }
}
