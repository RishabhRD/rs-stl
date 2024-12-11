// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

#[cfg(test)]
pub mod tests {
    use rng::infix::*;
    use stl::*;

    #[test]
    fn reverse() {
        let mut arr = [0, 1, 2];
        algo::reverse(&mut arr, 0, 2);
        assert!(arr.equals(&[1, 0, 2]));

        let mut arr = [0, 1, 2];
        rng::reverse(&mut arr);
        assert!(arr.equals(&[2, 1, 0]));

        let mut arr = [0, 1, 2];
        arr.reverse();
        assert!(arr.equals(&[2, 1, 0]));

        let mut arr = [0, 1, 2];
        algo::reverse(&mut arr, 0, 0);
        assert!(arr.equals(&[0, 1, 2]));
    }
}
