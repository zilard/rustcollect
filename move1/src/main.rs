fn main() {

    let data = vec![1, 2, 3];

    let closure = move || println!("captured {data:?} by value");

    //println!("data {:?}", data);

    closure();
}
