// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

#[cfg(test)]
pub mod tests {
    use rng::infix::*;
    use stl::*;
    use view::infix::*;

    #[test]
    fn take_for_less_elements() {
        let mut arr = [3, 1, 2, 7, 4];
        view::take(arr.view_mut(), 3).sort_range();
        assert_eq!(arr, [1, 2, 3, 7, 4]);

        let mut arr = [3, 1, 2, 7, 4];
        arr.view_mut().take(3).sort_range();
        assert_eq!(arr, [1, 2, 3, 7, 4]);

        let mut arr = [3, 1, 2, 7, 4];
        view::take(arr.view_mut(), 3).reverse();
        assert_eq!(arr, [2, 1, 3, 7, 4]);

        let mut arr = [3, 1, 2, 7, 4];
        arr.view_mut().take(3).reverse();
        assert_eq!(arr, [2, 1, 3, 7, 4]);
    }

    #[test]
    fn take_for_all_elements() {
        let mut arr = [3, 1, 2, 7, 4];
        view::take(arr.view_mut(), 5).sort_range();
        assert_eq!(arr, [1, 2, 3, 4, 7]);

        let mut arr = [3, 1, 2, 7, 4];
        arr.view_mut().take(5).sort_range();
        assert_eq!(arr, [1, 2, 3, 4, 7]);

        let mut arr = [3, 1, 2, 7, 4];
        view::take(arr.view_mut(), 5).reverse();
        assert_eq!(arr, [4, 7, 2, 1, 3]);

        let mut arr = [3, 1, 2, 7, 4];
        arr.view_mut().take(5).reverse();
        assert_eq!(arr, [4, 7, 2, 1, 3]);
    }

    #[test]
    fn take_for_more_elements() {
        let mut arr = [3, 1, 2, 7, 4];
        view::take(arr.view_mut(), 10).sort_range();
        assert_eq!(arr, [1, 2, 3, 4, 7]);

        let mut arr = [3, 1, 2, 7, 4];
        arr.view_mut().take(10).sort_range();
        assert_eq!(arr, [1, 2, 3, 4, 7]);

        let mut arr = [3, 1, 2, 7, 4];
        view::take(arr.view_mut(), 10).reverse();
        assert_eq!(arr, [4, 7, 2, 1, 3]);

        let mut arr = [3, 1, 2, 7, 4];
        arr.view_mut().take(10).reverse();
        assert_eq!(arr, [4, 7, 2, 1, 3]);
    }
}
