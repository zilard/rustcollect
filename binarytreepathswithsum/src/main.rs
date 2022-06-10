// Path with Sum:
// Given a binary tree in which each node contains
// an integer value, which might be positive or negative.
// Design an algorithm to count the number of paths that sum
// to a given value
// The path does not need to start or end at the root or a leaf
// bit it must go downwards, travelling only from parent nodes
// to child nodes


extern crate num;

use std::cell::RefCell;
use std::fmt::Display;
use std::rc::Rc;



type NodeRef = Rc<RefCell<Node>>;


struct BinaryTree {
    head: Option<NodeRef>,
}


#[derive(Debug)]
struct Node {
    data: i32,
    left: Option<NodeRef>,
    right: Option<NodeRef>,
}



impl BinaryTree {

    fn new() -> Self {
        Self { head: None }
    }


    fn insert(&mut self, value: i32) -> NodeRef {


        let ret = Rc::new(RefCell::new(Node {
            data: value,
            left: None,
            right: None,
        }));


        if self.head.is_none() {
            self.head = Some(ret.clone());
            ret
        } else {
            let mut head = self.head.as_mut().unwrap().clone();
            self.insert_at(&mut head, ret)
        }
    }








}



fn main() {

}

