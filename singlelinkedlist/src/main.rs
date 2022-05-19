
#[derive(Debug)]
enum ListNode<T> {
    Empty,
    NonEmpty(ListNodeValue<T>),
}


#[derive(Debug)]
struct ListNodeValue<T> {
    item: T,
    next: Box<ListNode<T>>,
}


impl<T> ListNodeValue<T> {

    fn new(item: T, next: Box<ListNode<T>>) -> Self {
        Self { item, next }
    }

}

impl<T> ListNode<T> {

    fn new(item: T, next: Box<ListNode<T>>) -> Self {
        ListNode::NonEmpty(ListNodeValue::new(item, next))
    }


    // we are swapping the current value with Empty and returing it
    fn take(&mut self) -> Self {
        let mut cur = Self::Empty;

        // std::mem::swap<T>(x: &mut T, y: &mut T)
        // swaps the values at 2 mutable locations,
        // without deinitializing either one
        std::mem::swap(&mut cur, self);
        cur
    }


}



// next we should create a public struct SinglyLinkedList
// which will contain methods push, pop and len

#[derive(Debug)]
pub struct SinglyLinkedList<T> {
    head: Box<ListNode<T>>,
    size: usize,
}



// head is a pointer to the heap that contains ListNode	

impl<T> SinglyLinkedList<T> {

    pub fn new() -> Self {
        Self {
            head: Box::new(ListNode::Empty),
            size: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.size
    }


    // Borrow checker will not allow you to borrow 'head' and then reasssign it
    // To trick the compile, we use the 'take' method creating a new node with 
    // that value and assigning it to the 'head'
    pub fn push(&mut self, item: T) {
        let cur_head = self.head.take();
        let new_node = Box::new(ListNode::new(item, Box::new(cur_head)));
       
        self.head = new_node;
        self.size += 1;
    }



    // if our list is empty then we should return None
    // otherwise we shoud decrease the size by one
    // repoing 'head' to the next node and return the current node value

    pub fn pop(&mut self) -> Option<T> {

        let node = self.head.take();
        
        if let ListNode::NonEmpty(node) = node {
            self.head = node.next;
            self.size -= 1;
            Some(node.item)
        } else {
            None
        }

    }



}



#[cfg(test)]
mod tests {

    use super::SinglyLinkedList;

    #[test]
    fn it_works() {

        let mut linked_list: SinglyLinkedList<usize> = SinglyLinkedList::new();

        for i in 1..=10 {
            linked_list.push(i);
        }

        for i in (1..=10).rev() {
            let cur = linked_list.pop();
            assert_eq!(Some(i), cur);
        }

        assert_eq!(None, linked_list.pop());

    }


    #[test]
    fn test_series_of_pops_and_inserts() {

        let mut list: SinglyLinkedList<usize> = SinglyLinkedList::new();

        assert_eq!(list.pop(), None);

        list.push(3);
        list.push(42);
        assert_eq!(list.pop(), Some(42));
        assert_eq!(list.len(), 1);

        list.push(93);
        assert_eq!(list.len(), 2);
        assert_eq!(list.pop(), Some(93));
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), None);
        assert_eq!(list.len(), 0);

        list.push(20);
        assert_eq!(list.pop(), Some(20));
        assert_eq!(list.pop(), None);

    }


}



fn main() {

        let mut linked_list: SinglyLinkedList<usize> = SinglyLinkedList::new();

        for i in 1..=10 {
            linked_list.push(i);
        }


        println!("linked list: {:?}", linked_list);


}


