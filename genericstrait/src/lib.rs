
pub trait Display {

}

pub trait Debug {

}


pub trait Clone {

}



pub trait Summary {

    fn summarize_author(&self) -> String {
        format!("(Read more...)")
    }

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }

}





pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}


impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

impl Display for Tweet {

}




pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}


impl Summary for NewsArticle {
}

impl Display for NewsArticle {

}

impl Clone for NewsArticle {

}





pub struct Dm {

}

impl Clone for Dm {

}

impl Debug for Dm {

}




