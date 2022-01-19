#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}


/*
fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}*/


fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}


#[derive(Debug)]
struct Point2D {
  x: f64,
  y: f64,
}



#[derive(Debug)]
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);



fn main() {
    
    let email = String::from("anonym@gmail.com");
    let username = String::from("anon");

    let user1 = build_user(email, username);

    println!("user1=> {:#?}", user1);


    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        active: user1.active,
        sign_in_count: user1.sign_in_count,
    };

    println!("user2=> {:#?}", user2);
    

    let user3 = User {
        email: String::from("yetanother@example.com"),
        username: String::from("yetanotherusername"),
        ..user2
    };

    println!("user3=> {:#?}", user3);



    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("BLACK: {:#?}", black);   




    /*
    let point = Point2D { x: 3.3, y: 7.2 };
    println!("POINT: {:?}", point);
    */

}
