//use std::rc::Rc;

fn add_to_list<'a>(flight_list: &'a mut Vec<&'a [String;2]>, flight: &'a [String;2]) -> &'a mut Vec<&'a [String;2]> {

    flight_list.push(flight);

    flight_list

}



fn main() {

    let mut flight_list = Vec::new();

    println!("RETURN: {:?}", add_to_list(&mut flight_list, &[String::from("SFO"), String::from("EWR")]));


}


