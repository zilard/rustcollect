use std::borrow::Borrow;

fn check<T: Borrow<str>>(s: T) {
    assert_eq!("Hello", s.borrow());
}



#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test1() {
        let s = "Hello".to_string();
        check(s);
        let s = "Hello";
        check(s);
    }

}




//#![allow(unused)]
fn main() {

    // move occurs because `s` has type `String`, 
    // which does not implement the `Copy` trait
    let s = "Hello".to_string();

    // value moved here
    check(s);

    // ERROR: value borrowed here after move   
    // println!("1st s: {}", s);

    let s = "Hello";
    check(s);
    println!("2nd s: {}", s);

}

