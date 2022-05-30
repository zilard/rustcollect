// Route Between Nodes
// -------------------
// Given a directed path, design an algorithm to find out 
// whether there is a route between 2 nodes

// This problem can be solved by just a simple graph traversal
// such as depth-first search or breadth-first search
// We start with one of the 2 nodes and during traversal, check
// if the other node is found.
// We should mark any node found in the course of the algorithm as
// "already visited" to avoid cycles and repetition of the nodes

use std::cell::RefCell;
use std::fmt::Display;
use std::rc::Rc;

// RefCell - a mutable memory location with dynamically checked borrow rules

type VertexRef<T> = Rc<RefCell<Vertex<T>>>;

struct Graph<T> {
    head: Option<VertexRef<T>>,
}

struct Vertex<T> {
    data: T,
    edges: Vec<VertexRef<T>>,
}


// std::vec::Vec - contiguous growable array type, written as Vec<T>, 
// short for 'vector'
// let mut vec = Vec::new();
// vec.push(1);
// vec.len()
// vec.pop()


impl<T> Graph<T> {


    fn new() -> Self {
        Self { head: None }
    }


    fn add_vertex(&mut self,
                  value: T,
                  parent_vertices: &[VertexRef<T>]) -> VertexRef<T> {

        let ret = Rc::new(RefCell::new(Vertex {
            data: value,
            edges: Vec::<VertexRef<T>>::new(),
        }));
        
        if parent_vertices.is_empty() {
            self.head = Some(ret.clone());
        } else {
            for parent in parent_vertices {
                parent.borrow_mut().edges.push(ret.clone());
            }
        }

        ret
    }

    
    fn dfs_from<F>(&self, f: &mut F, vertex: VertexRef<T>) -> bool
    where
        F: FnMut(&VertexRef<T>) -> bool,
    {
        for v in vertex.borrow().edges.iter() {
            if !self.dfs_from(f, v.clone()) {
                return false;
            }
        }
        f(&vertex)
    }


    fn dfs<F>(&self, mut f: F)
    where
        F: FnMut(&VertexRef<T>) -> bool,
    {
        if let Some(head) = &self.head {
            self.dfs_from(&mut f, head.clone());
        }
    }


    fn has_path(&self, from: VertexRef<T>, to: VertexRef<T>) -> bool {
        let mut found_path = false;
        self.dfs_from(
            &mut |v| {
                if Rc::ptr_eq(v, &to) {
                    // a path has been found
                    found_path = true;
                    false
                } else {
                    true
                }
            },  
            from, 
        );
        found_path

    }  

}



impl<T: Display> Display for Graph<T> {

    fn fmt(&self, w: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {

        write!(w, "[")?;

        self.dfs(|v| {
            write!(w, "{}, ", v.borrow().data).unwrap();
            true
        });

        write!(w, "]")

    }

}



#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_route_between_nodes() {

        let mut graph = Graph::<String>::new();

        let first = graph.add_vertex(String::from("SFO"), &[]);
        let second = graph.add_vertex(String::from("ATL"), &[first.clone()]);
        let third = graph.add_vertex(String::from("EWR"), &[second.clone()]);

        assert_eq!(graph.has_path(first.clone(), third.clone()), true);
        assert_eq!(graph.has_path(third, first), false);

    }

}




fn main() {

    let mut graph = Graph::<String>::new();

    let first = graph.add_vertex(String::from("SFO"), &[]);
    let second = graph.add_vertex(String::from("ATL"), &[first.clone()]);
    let third = graph.add_vertex(String::from("EWR"), &[second.clone()]);

    println!("has path? 'SFO' -> 'EWR': {}", graph.has_path(first.clone(), third.clone()));
    println!("has path? 'EWR' -> 'SFO': {}", graph.has_path(third, first));


    println!("graph {}", graph);

}


