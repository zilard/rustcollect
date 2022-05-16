


fn print_refs<'a, 'b>(x: &'a i32, y: &'b i32) {
    println!("x is {} and y is {}", x, y);
}


// this function has a lifetime parameter `'a`
fn failed_borrow<'a>() {
    let _x = 12;

    //let y: &'a i32 = &_x;
    // ERROR: borrowed value does not live long enough

    // attempting to use the lifetime of `'a` as an explicit type annotation
    // inside the function will fail bcs the lifetime of `&_x` is shorter
    // than that of `y`
    // A short lifetime cannot be coerced into a longer one
}


fn main() {

    // create variables to be borrowed below
    let (four, nine) = (4, 9);

    // Borrows ('&') of both variable are passed into the function
    print_refs(&four, &nine);
    // lifetime of 'four' and 'nine' must be longer that that of 'print_refs'


    failed_borrow();
    // 'failed_borrow' contains no references to force `a` to be
    // longer than the lifetime of the function `'a` is longer
    // bcs the lifetime is never constrained, it defaults to `'static` 

}


