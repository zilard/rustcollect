// algorithm that determines if a string has all unique characters

use std::collections::HashSet;

fn all_chars_uniq_v1(s: &str) -> bool {
    let mut charset: HashSet<char> = HashSet::new();

    for c in s.chars() {
        if charset.contains(&c) {
            return false;
        }
        charset.insert(c);
    }

    true
}

/// what if cannot use additional data structures
fn all_chars_uniq_v2(s: &str) -> bool {
    let mut bitfield: i64 = 0;

    let charinta: i16 = 'a' as i16;

    for c in s.chars() {
        let mut int_char: i16 = c as i16;
        int_char -= charinta;

        if (1 << int_char) & bitfield != 0 {
            return false;
        }

        bitfield |= 1 << int_char;
    }

    true
}

fn main() {
    println!(
        "UNIQ_v1 ? {}",
        all_chars_uniq_v1(&String::from("helloworld"))
    );
    println!(
        "UNIQ_v2 ? {}",
        all_chars_uniq_v2(&String::from("helloworld"))
    );
}
