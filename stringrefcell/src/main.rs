
use std::cell::RefCell;


fn main() {

    let ref_cell: RefCell<String> = RefCell::new("hello".to_string());


    // ref_cell.borrow() - returns a Ref<T> which is essentially just a 
    // shared reference to the value stored in ref_cell
    let r = ref_cell.borrow();    // returns a Ref<String> a shared reference
    let count = r.len();
    assert_eq!(count, 5);

    //let mut w = ref_cell.borrow_mut();   // panic: already borrowed
    //w.push_str(" world");
}


