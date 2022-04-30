// Given two strings, write a method to decide if one 
// is a permutation of the other.

use std::collections::HashMap;

fn count_chars(s: &str) -> HashMap<char, i32> {
    let mut characters: HashMap<char, i32> = HashMap::new();

    for c in s.chars() {

        // insert a key only if it doesn't already exist
        let x = characters.entry(c).or_insert(0);
        *x += 1;
    }
    characters
}




fn main() {
   
 
    //is_permutation(&String::from("cat"), &String::from("dog"));

    let string_hash = count_chars(&String::from("ccat"));

    for (key, value) in &string_hash {
        println!("{}: {}", key, value);
    }

}

