

fn main() {


    let x = vec![1,2,3]
        .iter()
        .map(|x| x + 3)
        .fold(1, |x, y| x + y);


    println!("x: {0}", x);

}



