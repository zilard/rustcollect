


#[derive(Clone, Copy, Debug, PartialEq)]
struct Location {
    row: usize,
    column: usize,
}


fn main() {

    let loc: &mut Location = &mut Location{row: 0, column: 0};

    loc.row = 1;
    loc.column = 2;

    println!("location: {:#?}", loc);

    let mut off_limits: Vec<&Location> = vec![];

    off_limits.push(loc);

    println!("off limits: {:#?}", off_limits);



 
    let mut another_vec: Vec<Location> = vec![];
    use rand::*;
    let mut rng = rand::thread_rng();
    for _ in 1..1000000 {
        another_vec.push(Location {
                            row: rng.gen_range(1..100),
                            column: rng.gen_range(1..100)           
                         });

    }
    println!("large vector: {:#?}", another_vec);

}
