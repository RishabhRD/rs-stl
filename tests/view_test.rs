// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

#[cfg(test)]
pub mod tests {
    use rng::infix::*;
    use stl::*;

    #[test]
    fn view_acts_as_range() {
        let mut sum = 0;
        let arr = [1, 2, 3];
        arr.view().for_each(|x| sum += x);
        assert_eq!(sum, 6);
    }

    #[test]
    fn mutable_view_acts_as_range() {
        let mut arr = [3, 2, 1];
        arr.view_mut().sort_range();
        assert_eq!(arr, [1, 2, 3])
    }
}
