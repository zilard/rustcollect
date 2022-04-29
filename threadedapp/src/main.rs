use std::thread;
use std::time::Duration;


fn add_one_v1 (x: u32) -> u32 { x + 1 }


fn main() {
    let simulated_user_specified_value = 26;
    let simulated_random_number = 10;

    generate_workout(simulated_user_specified_value, simulated_random_number);

    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    let add_one_v3 = |x| { x + 1 };
    let add_one_v4 = |x| x + 1;
    
    println!("1. test oneline func: {}", add_one_v1(1));
    println!("2. test oneline func: {}", add_one_v2(1));
    println!("3. test oneline func: {}", add_one_v3(1));
    println!("4. test oneline func: {}", add_one_v4(1));



    // The first time we call example_closure with the String value, 
    // the compiler infers the type of x and the return type of the closure to be String. 
    // Those types are then locked into the closure in example_closure, 
    // and we get a type error if we try to use a different type with the same closure.

    let example_closure = |x| x;

    let s = example_closure(String::from("hello"));
    let n = example_closure(5.to_string());

    println!("5. test example closure: {}", s);
    println!("6. test example closure: {}", n);

}

fn generate_workout(intensity: u32, random_number: u32) {

    let expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_closure(intensity));
        println!("Next, do {} situps!", expensive_closure(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!", 
                expensive_closure(intensity)
            );
        }

    }

}
