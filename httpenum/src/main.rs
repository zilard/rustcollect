use std::mem::size_of;

use std::cmp::Ordering;




enum HttpStatus {
    Ok = 200,
    NotModified = 304,
    NotFound = 404,
}


fn main() {

    println!("");
   
}


#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test1() {
        assert_eq!(size_of::<Ordering>(), 1);
    }

    #[test]
    fn test2() {
        assert_eq!(size_of::<HttpStatus>(), 2);
    }

    #[test]
    fn test3() {
        assert_eq!(HttpStatus::Ok as i32, 200);
    }
   
}
