// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

#[cfg(test)]
pub mod tests {
    use rng::infix::*;
    use stl::*;
    use view::infix::*;

    #[test]
    fn filter_test() {
        let arr = [1, 2, 3, 4, 5, 6];

        let v = view::filter(arr.view(), |x| x % 2 == 1);
        assert!(v.all_of(|x| x % 2 == 1));

        assert_eq!(v.at(&v.before(v.after(v.start()))), &1);

        let v = arr.view().filter(|x| x % 2 == 1);
        assert!(v.all_of(|x| x % 2 == 1));
    }
}
