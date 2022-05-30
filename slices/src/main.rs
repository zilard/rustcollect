
fn first_word(s: &String) -> &str {

    // as_bytes() - Returns a byte slice of this String’s contents.
    // the inverse of this method is from_utf8()

    let bytes = s.as_bytes();

    println!("string converted in bytes: {:?}", bytes);    

    for &item in bytes.iter() {
        println!("byte items of byte converted string: {}", item);
    }
 

    "test"

}



fn main() {

    let mut s = String::from("hello world");

    let word = first_word(&s);   // <-- immutable borrow occurs here

     println!("word: {}", word);


    // clear() - truncates this String, removing all contents, the String will have a length
    // of zero but it does not touch its capacity
    // s.clear();  <--- mutable borrow occurs here, AND this would trigger an error because:
    // recall from the borrowing rules that if we have an immutable reference to something
    // we cannot also take a mutable reference
    // clear needs to truncate the String, it tries to take a mutable reference which fails


}


