use std::fmt::Debug;


// function signatures with lifetimes have a few constraints:
// - any reference must have an annotated lifetime
// - any reference being returned must have the same lifetime as an input or be static

// 'static  is a reserved lifetime name,  which indicated that the data pointed
// to by the reference lives for the entire lifetime of the running program
// it can still be coerced to a shorter lifetime

// Make a constant with `'static` lifetime
static NUM: i32 = 18;


// Returns a reference to `NUM` whre it's `'static`
// lifetime is coerced to that of the input argument
fn coerce_static<'a>(_: &'a i32) -> &'a i32 {
    &NUM
}


fn print_it( input: impl Debug + 'static ) {
    println!("'static value passed in is: {:?}", input);
}


fn main() {

    // i is owned and contains no references, thus it's static
    let i = 5;
    print_it(i);

    // ERROR - borrowed value does not live long enoug
    //print_it(&i);
    // &i only has the lifetime defined by the scope of main()
    // so it's not 'static



    {
        // make a string literal whih has type &'static str
        let static_string = "I'm in read-only memory";
        println!("static string: {}", static_string);

        // when `static_string` goes out of scope, the reference
        // can no longer be used, but the data remains in the binary
    }



    {
        let lifetime_num = 9;
    
        // Coerce `NUM` to lifetime of `lifetime_num`
        let coerced_static = coerce_static(&lifetime_num);

        println!("coerced_static: {}", coerced_static);

    }


    println!("NUM: {} stays accessible", NUM);

    


}




