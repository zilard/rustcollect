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


    if rect1.width() {
        println!(
            "The rectangle has a nonzero width; it is {}",
            rect1.width);
    };


    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
   
    
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));


    // the :: syntax is used for both associated functions 
    // and namespaces created by modules.
    let sq = Rectangle::square(3);

    println!("The size of the newly created rectangle: {}", sq.print());

}
