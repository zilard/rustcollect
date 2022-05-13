
#[derive(Debug)]
enum Vehicle {
    Ford,
    Mazda,
    Honda,
}



fn main() {

    let car1: Vehicle = Vehicle::Ford;
    let car2: Vehicle = Vehicle::Mazda;

    println!("vehicle model no.1: {:?}", car1);
    println!("vehicle model no.2: {:?}", car2);

}


