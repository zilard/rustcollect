/*
Robot in a Grid: 
Imagine a robot sitting on the upper left corner of grid with r rows and c columns.
The robot can only move in two directions, right and down, but certain cells are "off limits" 
such that the robot cannot step on them. 
Design an algorithm to find a path for the robot from the top left to
the bottom right.
*/


/*
[ clippy::same_item_push ] checks whether a for loop is being used to push 
a constatant value into a Vec
why is this bad? this kin of operation can be expressed more succintly with 
vec![item;SIZE] or vec.resize(NEW_SIZE,item) 
and using these alternatives may also have better performance

[ clippy::unknown_clippy_lints ] is great for environments with heterogenous 
rustc/clippy versions but it only applies to lints mentioned in attributes
*/


/*
#[derive(Clone)]
For a generic struct, #[derive(Clone)] implements Clone conditionally by adding bound Clone
on generic parameters
'derive' implements Clone trait for STRUCT<T> when T is Clone

#[derive(Copy)]
Derive a 'Copy' implementation
'derive' implements Copy trait

PartialEq - trait for equality comparisons which are partial equivalence relations
this trait allows for partial equality, for types that do not have a full equivalence relation


*/




/*
Sometimes it's desirable to catch the failure of some parts of a program
instead of calling panic!
This can be accomplished using the Option enum

The Option<T> enum has two variants:
- None, to indicate failure or lack of value
- Some(value), a tuple struct that wraps a 'value' with type 'T'
*/



#![allow(clippy::same_item_push, clippy::unknown_clippy_lints)]


#[derive(Clone, Copy, Debug, PartialEq)]
struct Location {
    x: usize,
    y: usize,
}



fn get_path_to(
    current_location: Location,
    target_location: Location,
    off_limits: &[Location],
) -> Vec<Location> {


    if current_location == target_location {
        return vec![target_location];           
    }




    if current_location.y < target_location.y {

        let location_down = Location {
            x: current_location.x,
            y: current_location.y + 1,
        };

        if !off_limits.contains(&location_down) {
            //location down
            let mut path = get_path_to(location_down, target_location, off_limits);

            if let Some(last_location) = path.last() {
                if last_location == &target_location {

                    // Inserts an element at position index within the vector, 
                    // shifting all elements after it to the right.

                    path.insert(0, location_down);
                    return path;
                }
            }
        }  
    }




    if current_location.x < target_location.x {

        let location_right = Location {
            x: current_location.x + 1,
            y: current_location.y,
        };

        if !off_limits.contains(&location_right) {
            // location_right
            let mut path = get_path_to(location_right, target_location, off_limits);
            if let Some(last_location) = path.last() {
                if last_location == &target_location {
                    path.insert(0, location_right);
                    return path;
                } 
            }
        }
    }




    vec![]

}




#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    #[test]
    fn test_robot_in_a_grid() {
        let mut off_limits: Vec<Location> = vec![];
        let mut rng = rand::thread_rng();
        for _ in 0..100 {
            let x = rng.gen_range(1..100);
            let y = rng.gen_range(1..100);
            off_limits.push( Location {x, y} ); 
        }
        let path = get_path_to(
            Location {x: 0, y: 0},
            Location {x: 100, y: 100 },
            &off_limits,
        );
        assert_eq!(path.last().unwrap(), &Location { x: 100, y: 100 });
    }



}



// We call unwrap on that Result so that, in that case, 
// we'll panic too, and the user will get a report.
fn main() {

    let path1 = get_path_to(Location { x: 0, y: 0 },
                            Location { x: 100, y: 100 },
                            &[]);

    println!("path without blockage: {:#?}", path1);

    println!("{:=<100}", "");   // '=' * 100

    use rand::Rng;
    let mut rng = rand::thread_rng();

    let mut off_limits: Vec<Location> = vec![];

    for _ in 0..2000 {
        let x = rng.gen_range(0..100);
        let y = rng.gen_range(0..100);
        if (x != 0 && y != 0) || (x != 100 && y != 100) {
            off_limits.push( Location {x, y} );
        }
    }


    /*
    for _ in 0..50 {
        let y = rng.gen_range(0..100);
        if y != 0 {
            off_limits.push( Location {x: 0, y} );
        }
    }

    for _ in 0..50 {
        let x = rng.gen_range(0..100);
        if x != 100 {
            off_limits.push( Location {x, y: 100} );
        }
    }
    */


    println!("off limits: {:#?}", off_limits);


    println!("{:-<100}", "");   // '-' * 100


    let path2 = get_path_to(Location { x: 0, y: 0 },
                            Location { x: 100, y: 100 },
                            &off_limits);
   
    println!("path with limits: {:#?}", path2);     

}


