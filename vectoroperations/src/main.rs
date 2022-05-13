

#[derive(Debug)]
struct Val {
    v: u32,
}


fn main() {

    let mut vec1: Vec<Val> = vec![];

    println!("vec1: {:#?}", vec1);

    vec1.push(Val{v: 1});

    println!("vec1: {:#?}", vec1);

    vec1.push(Val{v: 2});

    println!("vec1: {:#?}", vec1);
 
    println!("last vec1: {:#?}", vec1.last());      
 
    let lst: Option<&Val> = vec1.last();

    println!("lst: {:#?}", lst);

    println!("lst unwrap: {:#?}", lst.unwrap());

    if let Some(lst2) = vec1.last() {
        println!("lst2 {:#?}", lst2);
    }

    vec1.insert(0, Val{v: 5});

    println!("extended vec: {:#?}", vec1);

}
