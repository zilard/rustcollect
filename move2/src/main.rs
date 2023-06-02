

fn create_fn() -> impl Fn() {
    let text  = "Fn".to_owned();
    move || println!("This is a: {text}")
}

fn main() {
    let fn_plain = create_fn();
    fn_plain();

    let data = vec![1, 2, 3];

    std::thread::spawn(move || {
        println!("captured {data:?} by value")
    }).join().unwrap();
}
