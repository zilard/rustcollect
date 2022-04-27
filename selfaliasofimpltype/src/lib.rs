
#[derive(Debug)]
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }

    pub fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}



#[derive(Debug)]
pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}


// All functions defined within an impl block are called associated functions 
// because they’re associated with the type named after the impl.
impl Rectangle {

    pub fn area(&self) -> u32 {
        self.width * self.height
    }


    pub fn width(&self) -> bool {
        self.width > 0
    }

    
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }


    pub fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }


    pub fn print(&self) -> String {

        format!("Rectangle width: {} and height: {}", self.width, self.height)

    }



}




