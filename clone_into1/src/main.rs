
fn main() {

    let v = String::from("hello");

    let mut s = String::new(); 

    v.clone_into(&mut s);

    let _a = s;

    println!("v: {}", v);

    println!("s: {}", s);

}

