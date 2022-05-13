

fn get_path_to(arr1: &mut [i32]) -> &[i32] {

    for i in 0..arr1.len() - 1 {
        arr1[i] = i as i32 + 1;
    }

    // indexes are always  of type 'usize'
    /*
    for i in 0..3_i32 {
        arr1[i as usize] = i;
    }
    */

    arr1

}



fn fill_vector(vec1: &mut Vec<i32>) -> &Vec<i32> {

    for i in 1..10 {
        vec1.push(i);
    }

    vec1

}





fn main() {

    let mut arr1: [i32; 3] = [0; 3];

    println!("array {:?}", get_path_to(&mut arr1));


    let mut vec1: Vec<i32> = vec![];
  
    println!("vec {:?}", fill_vector(&mut vec1));

}

