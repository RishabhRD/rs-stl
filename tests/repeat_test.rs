// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

#[cfg(test)]
pub mod tests {
    use rng::infix::*;
    use stl::*;
    use view::infix::*;

    #[test]
    fn repeat() {
        let arr = view::repeat(2).take(3);
        assert!(arr.equals(&[2, 2, 2]));
    }
}
