// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

#[cfg(test)]
pub mod tests {
    use rng::infix::*;
    use stl::*;

    #[test]
    fn maybe() {
        let mut arr = view::maybe(Some(2));
        assert!(arr.equals(&[2]));

        let i = arr.start();
        *arr.at_mut(&i) = 3;
        assert!(arr.equals(&[3]));

        let arr = view::maybe(Option::<i32>::None);
        assert!(arr.equals(&[]));
    }
}
