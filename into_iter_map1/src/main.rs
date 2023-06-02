fn main() {

    // let v: Vec<i32> = [1, 2, 3].into_iter().map(|x| x + 1)

    println!("v: {:?}", [1, 2, 3].into_iter());

    println!("v: {:?}", [1, 2, 3].into_iter().map(|x| x + 1));

    // The .collect() method consumes the iterator and collects 
    // the resulting values into a collection data type.
    let v: Vec<i32> = [1, 2, 3].into_iter().map(|x| x + 1).collect();
    println!("v: {:?}",  v);


    println!("v: {:?}", [1, 2, 3].into_iter().map(|x| x + 1).collect::<Vec<i32>>());


    println!("v: {:?}", [1, 2, 3].into_iter().map(|x| x + 1).collect::<Vec<_>>());

}
