// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

#[cfg(test)]
pub mod tests {
    use algo::*;
    use array::Array;
    use stl::*;

    #[test]
    fn single_pass() {
        let arr = Array([1, 2, 3]);
        let v = arr.map(|x| x + 1).map(|x| x - 1);
        let mut sum = 0;
        for e in v.iter() {
            sum += e;
        }
        assert_eq!(sum, 6);
    }

    #[test]
    fn reordering() {
        let mut arr = Array([1, 2, 3]);
        let mut v = arr.slice_mut().map(|x| x + 1).map(|x| x - 1);
        v.swap_at(&0, &2);
        assert_eq!(arr, Array([3, 2, 1]));
    }

    // TODO: tests for bidirectional range and random access range.
}
