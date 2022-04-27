/*
fn main() {
    println!("Hello, world!");
}
*/

const EUROPE: u8 = 0;
const ASIA: u8 = 1;
const AFRICA: u8 = 2;
const AMERICA: u8 = 3;
const OCEANIA: u8 = 4;

//let continent = ASIA;

fn main() {
    let continent = ASIA;

    if continent == EUROPE {
        print!("E");
    } else if continent == ASIA {
        print!("As");
    } else if continent == AFRICA {
        print!("Af");
    } else if continent == AMERICA {
        print!("Am");
    } else if continent == OCEANIA {
        print!("O");
    }
}
