// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

#[cfg(test)]
pub mod tests {
    use rng::infix::*;
    use stl::*;
    use view::infix::*;

    #[test]
    fn drop_for_less_elements() {
        let mut arr = [3, 1, 2, 7, 4];
        view::drop(arr.view_mut(), 2).sort_range();
        assert_eq!(arr, [3, 1, 2, 4, 7]);

        let mut arr = [3, 1, 2, 7, 4];
        arr.view_mut().drop(2).sort_range();
        assert_eq!(arr, [3, 1, 2, 4, 7]);

        let mut arr = [3, 1, 2, 7, 4];
        view::drop(arr.view_mut(), 2).reverse();
        assert_eq!(arr, [3, 1, 4, 7, 2]);

        let mut arr = [3, 1, 2, 7, 4];
        arr.view_mut().drop(2).reverse();
        assert_eq!(arr, [3, 1, 4, 7, 2]);
    }

    #[test]
    fn drop_for_all_elements() {
        let mut arr = [3, 1, 2, 7, 4];
        let mut v = view::drop(arr.view_mut(), 5);
        v.sort_range();
        assert!(v.equals(&[]));
        assert_eq!(arr, [3, 1, 2, 7, 4]);

        let mut arr = [3, 1, 2, 7, 4];
        let mut v = arr.view_mut().drop(5);
        v.sort_range();
        assert!(v.equals(&[]));
        assert_eq!(arr, [3, 1, 2, 7, 4]);

        let mut arr = [3, 1, 2, 7, 4];
        let mut v = view::drop(arr.view_mut(), 5);
        v.reverse();
        assert!(v.equals(&[]));
        assert_eq!(arr, [3, 1, 2, 7, 4]);

        let mut arr = [3, 1, 2, 7, 4];
        let mut v = arr.view_mut().drop(5);
        v.reverse();
        assert!(v.equals(&[]));
        assert_eq!(arr, [3, 1, 2, 7, 4]);
    }

    #[test]
    fn drop_for_more_elements() {
        let mut arr = [3, 1, 2, 7, 4];
        let mut v = view::drop(arr.view_mut(), 6);
        v.sort_range();
        assert!(v.equals(&[]));
        assert_eq!(arr, [3, 1, 2, 7, 4]);

        let mut arr = [3, 1, 2, 7, 4];
        let mut v = arr.view_mut().drop(6);
        v.sort_range();
        assert!(v.equals(&[]));
        assert_eq!(arr, [3, 1, 2, 7, 4]);

        let mut arr = [3, 1, 2, 7, 4];
        let mut v = view::drop(arr.view_mut(), 6);
        v.reverse();
        assert!(v.equals(&[]));
        assert_eq!(arr, [3, 1, 2, 7, 4]);

        let mut arr = [3, 1, 2, 7, 4];
        let mut v = arr.view_mut().drop(6);
        v.reverse();
        assert!(v.equals(&[]));
        assert_eq!(arr, [3, 1, 2, 7, 4]);
    }
}
