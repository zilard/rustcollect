
use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {


// The type Target = T; syntax defines an associated type 
// for the Deref trait to use.

// Without the Deref trait, the compiler can dereference only & references.

    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }


}


// Treating a Type Like a Reference by Implmeenting the Deref Trait


fn main() {
    let x = 5;
    let y = MyBox::new(x);

    println!("x: {:?}", x);

    // this throws error: type `MyBox<{integer}>` cannot be dereferenced
    println!("*y: {:?}", *y);


// When we entered *y
// behind the scenes Rust actually ran this code:
// *(y.deref())

// Note that the * operator is replaced with a call to the deref method 
// and then a call to the * operator just once, 
// each time we use a * in our code.


// The reason the deref method returns a reference to a value, and that the
// plain dereference outside the parentheses in *(y.deref()) is still necessary,
// is, the ownership system. 
// If the deref method returned the value directly instead
// of a reference to the value, the value would be moved out of self . We don’t
// want to take ownership of the inner value inside MyBox<T> in this case or in
// most cases where we use the dereference operator.

    // MyBox<T> type can’t be dereferenced because we haven’t implemented
    // that ability on our type. To enable dereferencing with the * operator,  
    // we implement the Deref trait.

}



