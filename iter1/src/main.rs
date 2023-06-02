fn main() {

    let v = [1, 2, 3];

    let mut iter = v.into_iter();

    println!("i1: {}", iter.next().unwrap());
    println!("i2: {}", iter.next().unwrap());
    println!("i3: {}", iter.next().unwrap());
    println!("i4: {:?}", iter.next());

}
