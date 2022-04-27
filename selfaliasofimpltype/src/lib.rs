
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

impl Rectangle {
    pub fn area(&self) -> u32 {
        self.width * self.height
    }
}




