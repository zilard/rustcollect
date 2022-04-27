use selfaliasofimpltype::{Tweet, Rectangle};


fn main() {

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());


    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

   println!(
       "The area of the rectangle is {} square pixels.",
       rect1.area()
   );

}
