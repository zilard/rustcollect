

pub fn bubble_sort<T: Ord>(arr: &mut [T]) {

    for i in 0..arr.len() {
        for j in 0..arr.len() - 1 - i {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }

}




/*

0..5 => 0,1,2,3,4

i:0 j:0
i:0 j:1
i:0 j:2
i:0 j:3
i:1 j:0
i:1 j:1
i:1 j:2
i:2 j:0
i:2 j:1
i:3 j:0

*/



#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn descending() {
        //descending
        let mut vec1 = vec![6, 5, 4, 3, 2, 1];
        bubble_sort(&mut vec1);
        for i in 0..vec1.len() - 1 {
            assert!(vec1[i] <= vec1[i + 1]);
        }

        for i in 0..vec1.len() {
            println!("i: {}", i);
        }
       
    }

    #[test]
    fn ascending() {
        //pre-sorted
        let mut vec2 = vec![1, 2, 3, 4, 5, 6];
        bubble_sort(&mut vec2);
        for i in 0..vec2.len() - 1 {
            assert!(vec2[i] <= vec2[i + 1]);
        }

        for i in 0..vec2.len() {
            println!("i: {}", i);
        }

    }

}





