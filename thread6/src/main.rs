use std::rc::Rc;

fn main() {

    let a = Rc::new([1, 2, 3]);

    let b = a.clone();

    println!("a {:?}", a.as_ptr());
    println!("b {:?}", b.as_ptr());

}
