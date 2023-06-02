use std::cell::Cell;

fn f(v: &Cell<Vec<i32>>) {
    let mut v2 = v.take();
    v2.push(1);
    v.set(v2);
}

fn main() {
    let v = Cell::new(vec![1, 2, 3]);
    //println!("BEFORE: v {:?}", v.into_inner());
    f(&v);
    println!("AFTER: v {:?}", v.into_inner());
}

