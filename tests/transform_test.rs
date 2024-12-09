// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

#[cfg(test)]
pub mod tests {
    use rng::infix::*;
    use stl::*;
    #[test]
    pub fn transform() {
        let input = vec![1, 2, 3];
        let mut out = vec![0, 0, 0, 0];

        let j = algo::transform(&input, 1, 2, &mut out, 1, |x| x + 1);
        assert!(out.equals(&vec![0, 3, 0, 0]));
        assert_eq!(j, 2);

        let j = rng::transform(&input, &mut out, |x| x + 1);
        assert!(out.equals(&vec![2, 3, 4, 0]));
        assert_eq!(j, 3);
    }

    #[test]
    pub fn zip_transform() {
        let input = vec![1, 2, 3];
        let input1 = vec![1, 2];
        let mut out = vec![0, 0, 0, 0];

        let j = algo::zip_transform(
            &input,
            1,
            2,
            &input,
            1,
            &mut out,
            1,
            |x, y| x * y,
        );
        assert!(out.equals(&vec![0, 4, 0, 0]));
        assert_eq!(j, 2);

        let j = rng::zip_transform(&input, &input1, &mut out, |x, y| x * y);
        assert!(out.equals(&vec![1, 4, 0, 0]));
        assert_eq!(j, 2);
    }
}
