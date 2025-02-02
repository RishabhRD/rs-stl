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
        let mut sum = 0;
        for e in arr.map(|x| x + 1).iter() {
            sum += e;
        }
        assert_eq!(sum, 9);
    }

    #[test]
    fn reordering() {
        let mut arr = Array([1, 2, 3]);
        arr.slice_mut().map(|x| x + 1).swap_at(&0, &2);
        assert_eq!(arr, Array([3, 2, 1]));
    }

    // TODO: tests for bidirectional range and random access range.
}
