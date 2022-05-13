use std::mem::size_of;

use std::cmp::Ordering;



#[derive(Debug)]
enum HttpStatus {
    Ok = 200,
    NotModified = 304,
    NotFound = 404,
}


fn http_status_from_u32(n: u32) -> Option<HttpStatus> {
    match n {
        200 => Some(HttpStatus::Ok),
        304 => Some(HttpStatus::NotModified),
        404 => Some(HttpStatus::NotFound),
        _ => None,
    }
}


fn main() {

    println!("200 is {:?}", http_status_from_u32(200));

    println!("566 is {:?}", http_status_from_u32(566));


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
