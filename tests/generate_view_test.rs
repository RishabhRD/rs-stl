// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

#[cfg(test)]
pub mod tests {
    use rng::infix::*;
    use stl::*;
    use view::infix::*;

    #[test]
    fn generate() {
        let mut counter = 0;
        let generator = || {
            if counter <= 3 {
                let result = Some(counter);
                counter += 1;
                result
            } else {
                None
            }
        };
        let arr = view::generate(generator).map(|x| x + 1);
        assert!(arr.equals(&[1, 2, 3, 4]));
    }
}
