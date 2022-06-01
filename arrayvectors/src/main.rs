use std::cell::RefCell;
use std::rc::Rc;

//type Reff<T> = Rc<RefCell<T>>;

/*
fn add_to_list<'a>(flight_list: &'a mut Vec<&'a [String;2]>, flight: &'a [String;2]) -> &'a mut Vec<&'a [String;2]> {

    flight_list.push(flight);

    flight_list

}



fn main() {

    let mut flight_list = Vec::new();

    println!("RETURN: {:?}", add_to_list(&mut flight_list, &[String::from("SFO"), String::from("EWR")]));

}
*/


/*
fn add_to_list<T, U>(flight_list: T, flight: U) -> T
where
    T: Rc<RefCell<Vec<[String;2]>>>,
    U: [String;2]
{
    flight_list
        .borrow_mut()
        .push(flight);

    flight_list

}
*/



fn add_to_list(flight_list: Rc<RefCell<Vec<[String;2]>>>, 
               flight: [String;2]) -> Rc<RefCell<Vec<[String;2]>>>
{
    flight_list
        .borrow_mut()
        .push(flight);

    flight_list

}




fn main() {

    let flight_list: Rc<RefCell<Vec<[String;2]>>> = Rc::from(RefCell::from(Vec::new()));


    //let ret: Rc<RefCell<Vec<[String;2]>>>;



    //let param1: Rc<RefCell<[String;2]>> = Rc::from(RefCell::from([String::from("SFO"), String::from("EWR")]));


    let param1: [String;2] = [String::from("SFO"), String::from("EWR")];


    /*
    flight_list
        .borrow_mut()
        .push(param1);
    */

    add_to_list(flight_list.clone(), param1.clone());
  
    println!("flight_list: {:?}", flight_list);
  

    println!("flight_list updated: {:?}", add_to_list(flight_list.clone(), param1.clone()));
    

    add_to_list(flight_list.clone(), param1.clone());
    println!("flight_list updated: {:?}", flight_list);






    // -----------------------------------------



    let flight_list2: Rc<RefCell<Vec<Rc<RefCell<[String;2]>>>>> = Rc::from(RefCell::from(Vec::new()));
    let param2: Rc<RefCell<[String;2]>> = Rc::from(RefCell::from([String::from("SFO"), String::from("EWR")]));


    flight_list2
        .borrow_mut()
        .push(param2);

    println!("flight_list2: {:?}", flight_list2);







    /*

    ret = add_to_list(flight_list.clone(), param1.clone());

    println!("RETURN: {:?}", ret);

    */


}



