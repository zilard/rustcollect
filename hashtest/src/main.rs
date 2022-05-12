use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};



#[derive(Hash)]
struct Person {
    id: u32,
    name: String,
    phone: u64,
}



fn main() {

    let mut contacts = HashMap::new(); 

    contacts.insert("Daniel", "798-1364");
    contacts.insert("Ashley", "645-7689");
    contacts.insert("Katie", "435-8291");
    contacts.insert("Robert", "956-1745");

    println!("contacts map {:?}", contacts);


    let person1 = Person {
        id: 5,
        name: "Janet".to_string(),
        phone: 555_666_7777

    };

    println!("hash of struct: {:?}", calculate_hash(&person1));
  

    // Takes a reference and returns Option<&V>
    match contacts.get(&"Daniel") {
        Some(&number) => println!("Daniel found! and here is his number {}", call(number)),
        _ => println!("Daniel _not_ found"),
    }

    contacts.insert("Matthew", "164-6743");


    match contacts.get(&"Donahan") {
        Some(&number) => println!("Donahan found! and here is his number {}", call(number)),
        _ => println!("Donahan _not_ found"), 
    }

    contacts.remove(&"Ashley");
 
    // HashMap::iter() returns an iterator that yields
    // (&'a key, &'a value) pairs in arbitrary order
    for (contact, number) in contacts.iter() {
        println!("Calling {}: {}", contact, number);
    }
   

}


fn call(number: &str) -> &str {
    match number {
        "798-1364" => "798-1364 called",
        _ => "Hi! Who is this again?"
    }
}



fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}


#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn t1() {

        let person1 = Person {
            id: 5,
            name: "Janet".to_string(),
            phone: 555_666_7777,
        };

        let person2 = Person {
            id: 5,
            name: "Bob".to_string(),
            phone: 555_666_7777,
        };

        assert!(calculate_hash(&person1) != calculate_hash(&person2));

    }
}


