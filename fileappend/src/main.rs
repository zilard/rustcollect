
use std::fs::File;
use std::io::prelude::*;
use std::io;
use std::path::Path;

struct Info {
    name: String,
    age: i32,
    rating: i32,
}


fn write_info(info: &Info) -> io::Result<()> {


    //let mut file = File::create("people.txt")?;
    //let mut file = File::create("people.txt").unwrap();


    let path = Path::new("people.txt");
    let display = path.display();

    println!("PATH {:?}", display);



    let mut file = File::options().append(true).open(&path)?;


    file.write_all(format!("name: {}\n", info.name).as_bytes())?;

    



/*    
    match file.write_all(format!("name: {}\n", info.name).as_bytes()) {
        Err(e) => {
            println!("Write error {:?}", e);
        }
        Ok(_) => {
            println!("Write success");
        }
    }
*/

    file.write_all(format!("age: {}\n", info.age).as_bytes())?;
    file.write_all(format!("rating: {}\n", info.rating).as_bytes())?;
    Ok(())
}



fn main() {

    let info = Info {
        name: String::from("Alan"),
        age: 32,
        rating: 10,
    };

    
    // ok() converts Result<T,E> to Option<T>
    // converting errors to None
    // which you won't get a warning for ignoring.
    
    //write_info(&info).map_err(|err| println!("{:?}", err)).ok();

    match write_info(&info) {
        Err(e) => println!("ERROR !!! {:?}", e),
        _ => ()
    }



}

