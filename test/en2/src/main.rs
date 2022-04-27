#![allow(unused)]

mod pet;

use pet::Pet::{self, Orca};

fn main() {
    let or: Pet = Orca;

    println!("{:?}", or);
}
