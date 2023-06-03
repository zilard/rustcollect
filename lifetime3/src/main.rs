#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}


fn main() {
    let novel = String::from("Whatever novel you read. Last night ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a second sentence");
    let i = ImportantExcerpt {
        part: first_sentence,
    };

    println!("excerpt: {:?}", i);
    // After first_sentence has gone out of scope, we would get an error
}
