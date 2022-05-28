// if we want to traverse from Owner to Gadget we will run into problems
// an Rc pointer from Owner to Gadget introduces a cycle
// This means that their reference counts can never reach 0, and the
// allocation will never be destroyed: a memory leak
// In order to get around this, we can use Weak pointers

// In order to end up with 2 values that point at each other, 
// one of them needs to be mutable
// This is difficult because Rc enforces memory safety by only giving out
// shared references to the value it wraps and these don't allow 
// direct mutation.
// We need to wrap the part of the value we wish to mutate in a RefCell
// which provided interior mutability, a method to achieve mutability 
// through a shared reference
// RefCell enforces Rust's borrowing rules at runtime


// https://doc.rust-lang.org/std/rc/

use std::rc::Rc;
use std::rc::Weak;
use std::cell::RefCell;


struct Owner {
    name: String,
    gadgets: RefCell<Vec<Weak<Gadget>>>,
}


struct Gadget {
    id: i32,
    owner: Rc<Owner>,
}


fn main() {

    // Create a refernce-counted Owner
    // Note that we've put the Owner's vector of Gadgets inside a RefCell
    // so that we can mutate it through a shared reference

    let gadget_owner: Rc<Owner> = Rc::new(
        Owner {   
            name: "Gadget Man".to_string(),
            gadgets: RefCell::new(vec![]),
        }
    );

    // Create Gadget's belonging to gadget_owner, as before
    let gadget1 = Rc::new(
        Gadget {
            id: 1,
            owner: Rc::clone(&gadget_owner),
        }
    );

    let gadget2 = Rc::new(
        Gadget {
            id: 1,
            owner: Rc::clone(&gadget_owner),
        }

    );

    // Add the Gadgets to their Owner
    {
        let mut gadgets = gadget_owner.gadgets.borrow_mut();
        gadgets.push(Rc::downgrade(&gadget1));
        gadgets.push(Rc::downgrade(&gadget2));

        // RefCell dynamic borrow ends here
    }


    // Iterate over our Gadgets pinting their details out 
    for gadget_weak in gadget_owner.gadgets.borrow().iter() {

        // gadget_weak is a Weak<Gadget>
        // Since Weak pointers cant guarantter the allocation still exists
        // we need to call upgrade which returns an Option<Rc<Gadget>>

        // in this case we know the allocation still exists, so we simply
        // unwrap the Option
        // in a more complicated program you might need graceful error 
        // handling for a None result

        let gadget = gadget_weak.upgrade().unwrap();
        println!("Gadget {} owned by {}", gadget.id, gadget.owner.name);

    }

    // At the end of the function, `gadget_owner`, `gadget1`, and `gadget2`
    // are destroyed. There are now no strong (`Rc`) pointers to the
    // gadgets, so they are destroyed. This zeroes the reference count on
    // Gadget Man, so he gets destroyed as well.

}

