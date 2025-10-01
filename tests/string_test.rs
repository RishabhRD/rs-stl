// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

#[cfg(test)]
pub mod tests {
    use stl::*;

    #[test]
    fn str_test() {
        let str = "hello";
        assert_eq!(str.count(), 5);

        let str = "你好";
        assert_eq!(str.count(), 2);

        let str = String::from("h你e好o");
        assert_eq!(str.count(), 5);
    }

    #[test]
    fn string_test() {
        let str = String::from("hello");
        assert_eq!(str.count(), 5);

        let str = String::from("你好");
        assert_eq!(str.count(), 2);

        let str = String::from("h你e好o");
        assert_eq!(str.count(), 5);
    }
}
