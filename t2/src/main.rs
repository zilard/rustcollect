
use std::{
    collections::HashMap,
    hash::Hash,
};


fn main() {

    // The mode consensus function selects the most common item from a list
    let v = vec![1, 2, 3, 3, 3];
    let c = mode_consensus(v.iter(), 51);

    println!("c: {:?}", c);

}


/// Given a list of elements, return the most common one.
/// In case of tie, return `None`.
pub fn mode_consensus<I, V>(pb: I, threshold: usize) -> Option<V>
where
    I: Iterator<Item = V>,
    V: Eq + Hash + std::fmt::Debug,
{
    let mut bp = HashMap::new();
    let mut len_pb = 0;
    for k in pb {
        *bp.entry(k).or_insert(0) += 1;
        len_pb += 1;
    }

    println!("bp hashmap: {:?}", bp);

  
    let mut bpv: Vec<_> = bp.into_iter().collect();
    println!("bpv vector from hashmap: {:?}", bpv);
     

    // Sort (beacon, peers) by number of peers
    bpv.sort_unstable_by(|a, b| b.1.cmp(&a.1));
    println!("bpv sorted vector from hashmap: {:?}", bpv);


    if bpv.len() >= 2 && (bpv[0].1 * 100) / len_pb < threshold {
        // In case of tie, no consensus
        None
    } else {
        // Otherwise, the first element is the most common
        bpv.into_iter().map(|(k, _count)| k).next()
    }


//The .iter() and .into_iter() functions are being used to convert a vector into an iterable.
//The .collect() method consumes the iterator and collects the resulting values into a collection data type.

}

