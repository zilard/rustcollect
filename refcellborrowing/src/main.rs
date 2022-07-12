// RefCell::new(value)

// ref_cell.borrow() returns a Ref<T> which is a shared 
// reference to the value stored in ref_cell

// ref_cell.borrow_mut() - returns RefMut<T> a mutable reference to the value in ref_cell

// ref_cell.try_borrow()
// ref_cell.try_borrow_mut()
// works like borrow() and borrow_mut() but returns a Result

use std::cell::RefCell;

fn main() {

    let ref_cell: RefCell<String> = RefCell::new("hello".to_string());
 
    {
        let r = ref_cell.borrow();
        let count = r.len();
 
        println!("count: {}", count);
    }

    {

        let mut w = ref_cell.borrow_mut();
        w.push_str("world");

        println!("w: {}", w);
    }

}


