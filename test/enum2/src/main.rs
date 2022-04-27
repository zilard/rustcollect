enum Continent {
    Europe,
    Asia,
    Africa,
    America,
    Oceania,
}

fn check(contin: Continent) {
    match contin {
        Continent::Europe => print!("_____EUROPE\n\n"),
        Continent::Asia => print!("_____ASIA\n\n"),
        Continent::Africa => print!("_____AFRICA\n\n"),
        Continent::America => print!("_____AMERICA\n\n"),
        Continent::Oceania => print!("_____OCEANIA\n\n"),
    }
}

fn main() {
    let mut contin: Continent;

    contin = Continent::Asia;
    check(contin);

    contin = Continent::Europe;
    check(contin);
}
