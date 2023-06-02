

fn main() {

    let s: &str = "a";

    let _ss: String = s.to_owned();


    let mut _sss = String::new();
    _ss.clone_into(&mut _sss);

    //let a: &str = s;

    println!("_ss {}", _ss);


    let mut s: String = String::new();

    let b = String::from("hello");
    b.clone_into(&mut s);

    println!("s {}", s);
    println!("b {}", b);


}

