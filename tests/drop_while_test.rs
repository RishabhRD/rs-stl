// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

#[cfg(test)]
pub mod tests {
    use rng::infix::*;
    use stl::*;
    use view::infix::*;

    #[test]
    fn take_while() {
        let mut arr = [1, 3, 5, 4, 2, 6];
        let mut v = view::drop_while(arr.view_mut(), |x| *x % 2 == 1);
        v.sort_range();
        assert!(v.equals(&[2, 4, 6]));
        assert_eq!(arr, [1, 3, 5, 2, 4, 6]);

        let mut arr = [1, 3, 5, 4, 2, 6];
        let mut v = arr.view_mut().drop_while(|x| *x % 2 == 1);
        v.sort_range();
        assert!(v.equals(&[2, 4, 6]));
        assert_eq!(arr, [1, 3, 5, 2, 4, 6]);

        let mut arr = [1, 3, 5];
        let mut v = arr.view_mut().drop_while(|x| *x % 2 == 1);
        v.sort_range();
        assert!(v.equals(&[]));
        assert_eq!(arr, [1, 3, 5]);
    }
}
