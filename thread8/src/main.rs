
fn main() {

    let a = [123, 456, 789];

    let b = unsafe { a.get_unchecked(1) };

    println!("b {}", b);

}
