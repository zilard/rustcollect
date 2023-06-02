use std::collections::HashMap;

fn main() {

    let mut map: HashMap<&str, u32> = HashMap::new();

    map.entry("poneyland").or_insert(3);

    println!("hashmap: {:?}", map);


    *map.entry("poneyland").or_insert(10) *= 2;

    println!("hashmap2: {:?}", map);
}
