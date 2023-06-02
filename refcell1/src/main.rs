use std::cell::RefCell;

fn f(v: &RefCell<Vec<i32>>) {
    v.borrow_mut().push(1);
}

fn main() {
    let v = RefCell::new(vec![1, 2, 3]);
    f(&v);
    println!("v: {:?}", v.into_inner());
}

