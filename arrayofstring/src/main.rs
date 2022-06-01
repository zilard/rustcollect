use std::{cell::RefCell, rc::Rc};

fn main() {

    let my_vec = Some(vec![String::from("Foo"), String::from("Bar")]);

    let mut rc_vec = Rc::from(RefCell::from(my_vec));


    // Rc - A single-threaded reference-counting pointer. 'Rc' stands for 'Reference Counted'

    // RefCell - mutable memory location with dynamically checked borrow rules
    // borrow_mut() -> RefMut<'_, T>   Mutably borrows the wrapped value

    // RefMut - A wrapper type for a mutably borrowed value from a RefCell<T>

    // as_ref(&self) -> &T   - Converts this type into a shared reference of the (usually inferred) input type.

    // as_mut(&mut self) -> &mut T   - Converts this type into a mutable reference of the (usually inferred) input type.



    // Ref<T> - shared reference to the value stored in ref_cell

    // borrow - Immutably borrows from an owned value
    // borrow_mut - Mutably borrows from an owned value
    // as_ref - Converts this type into a shared reference of the (usually inferred) input type
    // unwrap - Returns the contained Some value, consuming the self value.


    // rc_vec.borrow_mut().as_ref().unwrap().push(String::from("Baz"));
    // ERROR as_ref() - cannot borrow as mutable
    // !!! you need as_mut()
    rc_vec
        .borrow_mut()
        .as_mut()
        .map(|v| v.push(String::from("Baz")));
     
    // map() - returns an array of the same size as self
    // with function f applied to each element in order
 

    //println!("my_vec: {:?}" , my_vec);  // ERROR: value borrowed here after move


    // println!("rec_vec: {:?}", rc_vec);
    // rec_vec: RefCell { value: Some(["Foo", "Bar", "Baz"]) }


    // RefCell borrow - Immutably borrows the wrapped value.

    println!("rc_vec: {:?}", rc_vec.borrow().as_ref());
    // rec_vec: Some(["Foo", "Bar", "Baz"])

    // unwrap() - consumes the Option (moves out of it)
    // as_ref converts T to &T


    // ------------------------------------------------------


    let c = RefCell::new(5);


    // let borrowed_five = c.borrow_mut();  <-- this will cause PANIC at the next borrow()
    let borrowed_five = c.borrow();
    println!("borrowed_five: {}", borrowed_five);



    let borrowed_five2 = c.borrow();  //PANIC if previously borrow_mut was used on this reference
                                     // 'already mutably borrowed: BorrowError'

    println!("borrowed_five2: {}", borrowed_five2);
 

    // ------------------------------------------------------


    let mut str_vec = Rc::from(RefCell::from(vec![String::from("Foo"), String::from("Bar")]));

    /*
    str_vec
        .borrow_mut()
        .as_mut()
        .push(String::from("Baz"));

    println!("str_vec {:?}", str_vec);
    */


    // borrow_mut(&mut self) -> &mut Borrowed    Mutably borrows from an owned value.
    // println!("str_vec.borrow_mut() {:?}", str_vec.borrow_mut());
    // will print out =>["Foo", "Bar"]

    str_vec
        .borrow_mut()
        .push(String::from("Baz"));

    println!("str_vec {:?}", str_vec);

}


