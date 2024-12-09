// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

#[cfg(test)]
pub mod tests {
    use rng::infix::*;
    use stl::*;

    #[test]
    fn fill_value() {
        let mut arr = [0, 0, 0];
        algo::fill_value(&mut arr[..], 0, 2, &1);
        assert!(arr.equals(&[1, 1, 0]));

        let mut arr = [0, 0, 0];
        rng::fill_value(&mut arr[1..], &1);
        assert!(arr.equals(&[0, 1, 1]));

        let mut arr = [0, 0, 0];
        arr[1..].fill_value(&1);
        assert!(arr.equals(&[0, 1, 1]));
    }

    #[test]
    fn fill_by() {
        let mut arr = [0, 0, 0];
        algo::fill_by(&mut arr[..], 0, 2, || 1);
        assert!(arr.equals(&[1, 1, 0]));

        let mut arr = [0, 0, 0];
        rng::fill_by(&mut arr[1..], || 1);
        assert!(arr.equals(&[0, 1, 1]));

        let mut arr = [0, 0, 0];
        arr[1..].fill_by(|| 1);
        assert!(arr.equals(&[0, 1, 1]));
    }
}
