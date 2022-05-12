

use std::cmp::Ordering;


fn compare(n: i32, m: i32) -> Ordering {
    if n < m {
        Ordering::Less
    } else if n > m {
        Ordering::Greater
    } else {
        Ordering::Equal
    }
}


#[cfg(test)]

mod test {
    use super::*;

    #[test]
    fn t1() {
        assert_eq!(Ordering::Less, compare(2, 3));
    }

    #[test]
    fn t2() {
        assert_eq!(Ordering::Greater, compare(6, 7));
    }


}


fn main() {
    println!("compare => {:?}", compare(2, 3));
}


