// Stack Min
// How would you design a stack which
// in addition to push and pop
// has a function min which returns the minimum element
// push, pop and min should all operate in O(1) time

#[derive(Debug)]
struct Pointer {
    start: usize,
    end: usize,
}


#[derive(Debug)]
struct ThreeInOne<T> {
    arr: Vec<T>,
    pointers: Vec<Pointer>,
}



// impl of ThreeInOne for a generic type 'T'
// `<T>` must precede the type to remain generic
impl<T> ThreeInOne<T> {


    fn new() -> Self {
        ThreeInOne {
            arr: Vec::<T>::new(),
            pointers: vec![],
        }
    }





    fn push(&mut self, stack: usize, value: T) {
 

        // is_empty() returns true if the vector contains no elements
        let last_end = if self.pointers.is_empty() {
            0
        } else {
            // last() returns last element of the slice, or None if it is empty
            // unwrap() returns the contained Some value, consuming the self value
            self.pointers.last().unwrap().end
        };

 
        if self.pointers.len() < stack + 1 {
            
            // resize_with(new_len, f)
            // calling the closure f. The return values from f will end up 
            // in the Vec in the order they have been generated.
            self.pointers.resize_with(stack + 1, || Pointer { 
                                                        start: last_end,
                                                        end: last_end
                                                    }); 

        }


        // inserts an element at position index within the vector, 
        // shifting all elements after it to the right
        self.arr.insert(self.pointers[stack].start, value);

        self.pointers[stack].end += 1;

        // iter_mut() returns an iterator that allows modifying each value

        // std::iter::Iterator 
        // skip - creates an iterator that skips the first n elements
        for pointer in self.pointers.iter_mut().skip(stack + 1) {
            pointer.start += 1;
            pointer.end += 1;
        }      
        
    }





    fn pop(&mut self, stack: usize) -> Option<T> {


        if self.pointers.len() < stack + 1 ||
           self.pointers[stack].start == self.pointers[stack].end
        {
             return None;
        }

        // remove(index: usize) removes an returns the element 
        // at position index within the vector
        // shifting all elements after it to the left
        let result = self.arr.remove(self.pointers[stack].start);

        self.pointers[stack].end -= 1;

        // iter_mut() returns a iterator that allows modifying each value
        // skip(n: usize) creates an iterator that skips the first n elements
        for pointer in self.pointers.iter_mut().skip(stack + 1) {
            pointer.start -= 1;
            pointer.end -= 1;
        }

        Some(result)

    }


}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_threeinone() {

        let mut three_in_one: ThreeInOne<i32> = ThreeInOne::new();

        three_in_one.push(0, 1);
        three_in_one.push(1, 2);
        three_in_one.push(2, 3);


        assert_eq!(three_in_one.pop(2).unwrap(), 3);
        assert_eq!(three_in_one.pop(1).unwrap(), 2);
        assert_eq!(three_in_one.pop(0).unwrap(), 1);


        three_in_one.push(0, 4);
        three_in_one.push(1, 5);
        three_in_one.push(2, 6);
        three_in_one.push(0, 7);
        three_in_one.push(1, 8);
        three_in_one.push(2, 9);
        assert_eq!(three_in_one.pop(2).unwrap(), 9);
        assert_eq!(three_in_one.pop(1).unwrap(), 8);
        assert_eq!(three_in_one.pop(0).unwrap(), 7);

    }       


}




fn main() {

    let mut three_in_one: ThreeInOne<i32> = ThreeInOne::new();

    three_in_one.push(0, 1);
    three_in_one.push(1, 2);
    three_in_one.push(2, 3);
    three_in_one.push(0, 4);
    three_in_one.push(1, 5);
    three_in_one.push(2, 6);
    three_in_one.push(0, 7);
    three_in_one.push(1, 8);
    three_in_one.push(2, 9);

    println!("ThreeInOne: {:?}", three_in_one);


/*
    println!("ThreeInOne: {:?}", three_in_one);

    three_in_one.push(0, 111);
    println!("ThreeInOne: {:?}", three_in_one);

    three_in_one.push(1, 222);
    println!("ThreeInOne: {:?}", three_in_one);

    three_in_one.push(2, 333);
    println!("ThreeInOne: {:?}", three_in_one);

 
    //let popped_val: Option<i32> = three_in_one.pop(1);
    let unwrapped_popped_val: i32 = three_in_one.pop(1).unwrap();


    println!("1 popped, check now ThreeInOne: {:?}", three_in_one);
    //println!("check popped value: {:?}", popped_val);
    println!("check popped value: {:?}", unwrapped_popped_val);
*/ 


}



