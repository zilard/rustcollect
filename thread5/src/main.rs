use std::thread;

fn main() {

    let mut numbers = vec![1, 2, 3];

    thread::scope(|s| {

        s.spawn(|| {
            numbers.push(1);
        });

        s.spawn(|| {
            numbers.push(2);
        });

    });

}
