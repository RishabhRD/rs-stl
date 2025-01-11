// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

#[cfg(test)]
pub mod tests {
    use stl::*;

    #[test]
    fn str_test() {
        let str = "hello";
        let n = str.distance(str.start(), str.end());
        assert_eq!(n, 5);

        let str = "你好";
        let n = str.distance(str.start(), str.end());
        assert_eq!(n, 2);
    }

    #[test]
    fn string_test() {
        let str = String::from("hello");
        let n = str.distance(str.start(), str.end());
        assert_eq!(n, 5);

        let str = String::from("你好");
        let n = str.distance(str.start(), str.end());
        assert_eq!(n, 2);

        let str = String::from("h你e好o");
        let n = str.distance(str.start(), str.end());
        assert_eq!(n, 5);
    }
}
