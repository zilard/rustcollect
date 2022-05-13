

fn main() {

    let maybe_name = Some(String::from("Josh"));

    // The variable 'maybe_name' is consumed here ...

    // The 'maybe_name' value was partially moved here
    // partial move occurs because value has type `String`, 
    // which does not implement the `Copy` trait

    match maybe_name {
        Some(n) => println!("Hello, {}", n),
        _ => println!("Hello, world"),
    }

    // unwrap_or <= returns the contained 'Some' value or a provided default
    // unwrap    <= returns the contained 'Some' value, consuming the self value

    // String:from("world)
    // "world".into()

    // ... and is now unavailable
    // The 'maybe_name' value was partially moved in the match statement, 
    // so it cannot be used anymore here
    // partial move occurs because value has type `String`, 
    // which does not implement the `Copy` trait
    // So the solution is to 'borrow' using 'ref'
    // println!("Hello again, {}", maybe_name.unwrap_or("world".into()));

    
    // !!! Good solution 'borrowing' with 'ref'
    let maybe_name2 = Some(String::from("Alice"));
    
    // Using 'ref', the value is borrowed, not moved ...
    match maybe_name2 {
        Some(ref n) => println!("Hello, {}", n),
        _ => println!("Hello, world"),
    }

    
   // ... so it's available here!
   println!("Hello again, {}", maybe_name2.unwrap_or("world".into()));

}

