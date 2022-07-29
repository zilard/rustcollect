
use Event::NewLaunch;

enum Event {
  NewLaunch,
}


fn probability(_: &Event) -> f64 {
  0.95
}


fn descriptive_probability(event: Event) -> &'static str {

  match probability(&event) {
    1.0 => "true",
    0.0 => "impossible",
    0.0 ..= 0.25 => "very unlikely",
    0.25 ..= 0.5 => "unlikely",
    0.50 ..= 0.75 => "probable", 
    0.75 ..= 1.0 => "very likely",
    _ => "dont know",
  }

}



// A reference with 'static lifetime:
// let s: &'static str = "hello world";

// 'static as part of a trait bound:
// fn generic<T>(x: T) where T: 'static {}



// as a reference lifetime 'static indicates that the data pointed
// to by the referece lives for the entire lifetime of the running program


// There are 2 ways to make a variable with 'static lifetime
// and both are stored in the read-only memory of the binary
// - make a constant with the static declaration
// - make a string literal which has type: &'static str



// Make a constant with 'static lifetime
static NUM: i32 = 18;

// Returns a reference to 'NUM' where it's 'static'
// lifetime is coerced to that of the input argument
fn coerce_static<'a>(_: &'a i32) -> &'a i32 {
  &NUM
}



fn main() {

  let lifetime_num = 9;
  // Coerce 'NUM' to lifetime of 'lifetime_num'
  let coerced_static = coerce_static(&lifetime_num);
  println!("coerced_static: {}", coerced_static);




  println!("{}", descriptive_probability(NewLaunch));



  for x in 0..10 {
    println!("{}", x);
  }


  let mut range = 0..10;

  loop {
    match range.next() {
      Some(x) => {
        println!(">> {}", x);
      },
      None => {break}
    }
  }


  let number = 19;
  match number {
    1 => println!("One!"),
    2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
    13 ..= 19 => println!("A teen"),
    _ => println!("Ain't special"),
  }


  let boolean = true;
  let binary = match boolean {
    false => 0,
    true => 1,
  };
  println!("{} -> {}", boolean, binary);


}



