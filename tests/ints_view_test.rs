// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

#[cfg(test)]
pub mod tests {
    use rng::infix::*;
    use stl::*;

    #[test]
    fn ints() {
        let nums = view::ints(0);

        let mut i = nums.find(&10);
        assert_eq!(i, 10);

        i = nums.after_n(i, 3);
        assert_eq!(i, 13);
        assert_eq!(*nums.at(&i), 13);

        i = nums.before(i);
        assert_eq!(i, 12);
        assert_eq!(*nums.at(&i), 12);
    }
}
