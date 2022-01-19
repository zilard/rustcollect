
#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}


fn main() {


    /*
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    println!("\nuser1: {:?}\n", user1);
    user1.email = String::from("anotheremail@example.com");
    println!("\nuser1 V2: {:?}\n", user1);

    */

    let mut user1 = build_user(String::from("someone@example.com"),
                               String::from("someusername123"));


    println!("\nuser1: {:?}\n", user1);


}



fn build_user(email: String, username: String) -> User {

    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }

}


