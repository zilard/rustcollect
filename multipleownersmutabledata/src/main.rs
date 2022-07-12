// Rc<T> lets you have multiple owners of some data
// but it only gives immutable access to that data

// If you have an Rc<T> that holds a RefCell<T> 
// you can get a value that can have multiple owners
// and that you can mutate


// we use Rc<T> to allow multiple list to share ownership 
// of another list
// Because Rc<T> holds only immutable values
// we can't change any of the values in the list once we've created them
// Let's add in RefCell<T> to gain the ability to change the values in the lists

// https://doc.rust-lang.org/std/keyword.enum.html
// Enum - a type that can be any one of several variants
// Enums in Rust are similar to those of other compiler languages like C
// each enum variant can have data to go along with it

// enum SimpleEnum {
//     FirstVariant,
//     SecondVariant,
//     ThirdVariant,
// }

// https://doc.rust-lang.org/std/primitive.never.html
// The ! type, also called "never"
// ! represents the type of computations which never resolve 
// to any value at all



// by using a RefCell<T> in the Cons definition
// we can modify the value stored in all the lists

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);

}

