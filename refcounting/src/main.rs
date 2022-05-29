// reference counting, Rc<T> type keeps track of the number of references
// to a value to determine whether or not the value is still in use
// If there are zero references to a value, the value can be cleanup up
// without any references becoming valid

// We use the Rc<T> type when we want to allocated some data on the heap
// for multiple parts of our program to read and we can't determine at 
// compile time which part will finish using the data last.
// If we knew which part would finish last, we could just make that part the 
// data's owner, and the normal ownership rules enforced at compile time would
// take effect

// Note that Rc<T> is only for use in single-threaded scenarios

//  Lets create 2 lists that both share ownership of a third list


// Demonstrating that we're not allowed to have 2 lists using Box<T>
// that try to tshar ownership of a 3rd list

// https://doc.rust-lang.org/rust-by-example/custom_types/enum/testcase_linked_list.html

// Cons: Tuple struct that wraps an element and a pointer to the next node


/*
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {

    let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));

    let b = Cons(3, Box::new(a));

    let c = Cons(4, Box::new(a));

}
*/


/*

33 |     let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
   |         - move occurs because `a` has type `List`, which does not implement the `Copy` trait
34 | 
35 |     let b = Cons(3, Box::new(a));
   |                              - value moved here
36 | 
37 |     let c = Cons(4, Box::new(a));
   |                              ^ value used here after move

*/


// The Cons variants own the data they hold, so when we create the b list
// a is moved into b and b owns a
// Then when we try to use a again when creating c, we are not allowed because
// a has been moved

// We could change the definition of Cons to hold references instead
// but then we would have to specify lifetime parameters. 
// By specifying lifetime parameters, we would be specifying that
// every element in the list will live at least as long as the entire list





// Instead we'll change our definition of List to use Rc<T> in place of Box<T>
// Each Cons variant will now hold a value and and Rc<T> pointing to a List
// When we create b instead of taking ownership of a, we'll clone the Rc<List>
// that a is holding, thereby increasing the number of references from 1 to 2
// and leting a and b share ownership of the data in that Rc<List>
// We'll also clone a when creating c, increasing the number of references 
// from 2 o 3
// Every time we call Rc::clone, the reference count to the data within the 
// Rc<List> will increase and the data wont be cleaned up unless there are
// zero references to it


enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;


fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a));
}





