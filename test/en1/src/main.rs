//#[derive(Debug)]

/*
enum Ordering {
    Less,
    Equal,
    Greater
}
*/

//use std::cmp::Ordering;

mod ordering;

use ordering::Ordering;
use ordering::Ordering::*;

fn compare(n: i32, m: i32) -> Ordering {
    if n < m {
        Less
    } else if n > m {
        Greater
    } else {
        Equal
    }
}


fn main() {

    let r: Ordering = compare(1, 2);

    println!("compare: {:?}", r);

}


