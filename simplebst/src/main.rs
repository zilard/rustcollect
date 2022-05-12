
/*
enum Option<T> {
    None,
    Some(T),
}

enui Result<T, E> {
    Ok(T),
    Err(E),
}
*/



// each BinaryTree value is either Empty or NonEmpty
// if it's Empty, then it contains no data at all
// if NonEmpty then is has a Box, a pointer to a heap-allocated TreeNode


#[derive(Debug)]
enum BinaryTree<T> {
    Empty,
    NonEmpty(Box<TreeNode<T>>),
}


#[derive(Debug)]
struct TreeNode<T> {
    element: T,
    left: BinaryTree<T>,
    right: BinaryTree<T>,
}



fn main() {

    use self::BinaryTree::*;



    let mercury_tree = NonEmpty(Box::new(TreeNode {
        element: "Mercury",
        left: Empty,
        right: Empty,
    }));


    let jupiter_tree = NonEmpty(Box::new(TreeNode {
        element: "Jupiter",
        left: Empty,
        right: Empty,
    }));


    let mars_tree = NonEmpty(Box::new(TreeNode {
        element: "Mars",
        left: jupiter_tree,
        right: mercury_tree,
    }));


    let venus_tree = NonEmpty(Box::new(TreeNode {
        element: "Venus",
        left: Empty,
        right: Empty,
    }));


    let uranus_tree = NonEmpty(Box::new(TreeNode {
        element: "Uranus",
        left: Empty,
        right: venus_tree,
    }));


    let saturn_tree = NonEmpty(Box::new(TreeNode {
        element: "Saturn",
        left: mars_tree,
        right: uranus_tree,
    }));



    println!("tree: {:#?}", saturn_tree)

}

