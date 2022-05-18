

fn main() {

    //let x: Vec<i32> = Vec::<i32>::new();

    let x: &mut [i32] = &mut [1, 2, 3, 4, 5]; 

    println!("x: {:?}", x);

    for elem in x.iter_mut() {
        *elem += 2;
    }

    println!("x: {:?}", x);

    for elem in x.iter_mut().skip(2) {
        *elem += 10;
    }

    println!("x: {:?}", x);

}


