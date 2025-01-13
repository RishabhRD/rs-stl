// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

#[cfg(test)]
pub mod tests {
    use stl::*;
    use view::infix::*;

    #[test]
    fn test_cycle() {
        let arr = [1, 2, 3];

        let v = view::cycle(arr.view());
        let n = v.at(&v.after_n(v.start(), 3));
        assert_eq!(*n, 1);

        let v = arr.view().cycle();

        let n = v.at(&v.after_n(v.start(), 0));
        assert_eq!(*n, 1);

        let n = v.at(&v.after_n(v.start(), 1));
        assert_eq!(*n, 2);

        let n = v.at(&v.after_n(v.start(), 2));
        assert_eq!(*n, 3);

        let n = v.at(&v.after_n(v.start(), 3));
        assert_eq!(*n, 1);

        let n = v.at(&v.after_n(v.start(), 4));
        assert_eq!(*n, 2);

        let n = v.at(&v.after_n(v.start(), 5));
        assert_eq!(*n, 3);

        let n = v.at(&v.after_n(v.start(), 6));
        assert_eq!(*n, 1);

        let i = v.after_n(v.start(), 6);

        let n = v.at(&v.before_n(i.clone(), 0));
        assert_eq!(*n, 1);

        let n = v.at(&v.before_n(i.clone(), 1));
        assert_eq!(*n, 3);

        let n = v.at(&v.before_n(i.clone(), 2));
        assert_eq!(*n, 2);

        let n = v.at(&v.before_n(i.clone(), 3));
        assert_eq!(*n, 1);
    }
}
