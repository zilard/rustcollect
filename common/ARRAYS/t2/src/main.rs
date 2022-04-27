fn main() {
    let arr: [i32; 5] = [1, 2, 3, 4, 5];

    println!("{} => {:?}", arr.len(), arr);

    for i in 0..arr.len() {
        println!("{}", i)
    }

    for i in 0..arr.len() {
        for j in 0..arr.len() - 1 - i {
            println!("i:{} j:{}", i, j);
        }
    }
}
