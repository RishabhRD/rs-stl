// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Rishabh Dwivedi (rishabhdwivedi17@gmail.com)

/// Returns the length of character for character of type `char_type`.
pub fn char_len_for(char_type: u8) -> usize {
    match char_type {
        0x00..=0x7F => 1,
        0xC0..=0xDF => 2,
        0xE0..=0xEF => 3,
        0xF0..=0xF7 => 4,
        _ => 0,
    }
}
