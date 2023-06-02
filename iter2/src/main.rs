

// just a wrapper over Vec<T>

#[derive(Debug)]
struct MyCollection(Vec<i32>);


impl MyCollection {
    fn new() -> MyCollection {
        MyCollection(Vec::new())
    }

    fn add(&mut self, elem: i32) {
        // struct with one field, which you can access with .0
        self.0.push(elem);
    }
}

// and we'll implement IntoIterator
impl IntoIterator for MyCollection {
    type Item = i32;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}



fn main() {
    let mut c = MyCollection::new();

    c.add(100);
    c.add(111);
    c.add(112);

    /*
    for (i, n) in c.into_iter().enumerate() {
        println!("i:{} n:{}", i, n);
    }
    */

    c.0.iter().enumerate().for_each(|(i, n)| {
        println!("i:{} n:{}", i, n);
    })

}
