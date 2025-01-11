// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

#[cfg(test)]
pub mod tests {
    use rng::infix::*;
    use stl::*;

    #[test]
    fn iter() {
        let mut sum = 0;
        for e in view::single(2).iter() {
            sum += e;
        }

        assert_eq!(sum, 2);
    }
}
