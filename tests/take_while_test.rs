// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

#[cfg(test)]
pub mod tests {
    use rng::infix::*;
    use stl::*;
    use view::infix::*;

    #[test]
    fn take_while() {
        let arr = [1, 3, 5, 2, 4];
        let v = view::take_while(arr.view(), |x| *x % 2 == 1);
        assert!(v.equals(&[1, 3, 5]));

        let arr = [1, 3, 5, 2, 4];
        let v = arr.view().take_while(|x| *x % 2 == 1);
        assert!(v.equals(&[1, 3, 5]));
    }
}
