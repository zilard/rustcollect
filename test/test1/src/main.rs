fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        println!("m:{} n:{}", m, n);
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        println!("m:{} n:{}", m, n);
        m = m % n;
        println!("m:{}", m);
        println!("--------------------");
    }
    n
}

fn main() {
    gcd(0,100);
}


#[test]
fn test_gcd() {
    assert_eq!(gcd(14,15), 1);

    assert_eq!(gcd(2 * 3 * 5 * 11 * 17,
                   3 * 7 * 11 * 13 * 19),
               3 * 11);
}

