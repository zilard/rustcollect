// Route Between Nodes:
// Give a directed graphs, design an algorithm to find out whether
// there is a route between two nodes

// cell::RefCell - a mutable memory location with dynamically 
// checked borrow rules

// T: ?Sized 
// we would read this as "T may or may not be Sized"

// fmt::Display
// format trait for an empty format {}
// Display is similar to Debug

// rc::Rc
// A single-threaded reference-counting pointer
// Rc - Reference Counted


// You can explicitly creat a Vec with Vec::new
// let v: Vec<i32> = Vec::new();

// or by using the vec! macro
// let v: Vec<i32> = vec![];


use std::cell::RefCell;
use std::fmt::Display;
use std::rc::Rc;



type VertexRef<T> = Rc<RefCell<Vertex<T>>>;

struct Graph<T> {
    head: Option<VertexRef<T>>,
}


struct Vertex<T> {
    data: T,
    edges: Vec<VertexRef<T>>,
} 



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
            // makes a clone of the Rc pointer
            // this creates another pointer to the same allocation,
            // increasing the strong reference count
            self.head = Some(ret.clone());
        } else {
            for parent in parent_vertices {
                // mutably borrows from an owned value
                parent.borrow_mut().edges.push(ret.clone());
            }
        }
        
        ret

    }




    fn dfs_from<F>(&self, 
                   f: &mut F, 
                   vertex: VertexRef<T>) -> bool
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



    // .clone() - makes a clone of the Rc pointer
    // .borrow() - Immutably borrows from an owned value


    fn dfs<F>(&self, mut f: F)
    where
        F: FnMut(&VertexRef<T>) -> bool,
    {
        if let Some(head) = &self.head {
            self.dfs_from(&mut f, head.clone());
        }
    }    



    // rc::Rc - returns true if the 2 Rc's point to the same allocation

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



// fmt::Display - format trait, for user-facing output
// fmt() formats the vale using the given formatter

// fn fmt(&self, f: &mut fmt::Formater<'_>) -> fmt::Result
// the _ identifier is used when the name does not matter
// let _ = ... is used to ignore the result of an expression
// in types, _ is used to elide types (either because you prefer to let the
// compile infer it or because the compiler judges it does not matter
// <[_]> _ used as a type is a placeholder or "wildcard", it is not a type itself
// but serves to indicate that the type should be inferred

// let letters: Vec<_> = vec!["a", "b", "c"]
// _ is a placeholder, in this context it means that there isn't enough information
// for the compiler to infer a type


// _ in a match pattern is called a wildcard, because it matches anything


// UNDERSCORES IN TYPES
// _ underscores can be used to omit type declarations
// for example it can be used to specify only part of a type,
// and let Rust infer the rest

// let v: Vec<_> = iter.collect();
// let v = iter.collect::<Vec<_>>();


// ANONYMOUS LIFETIME '_
// The anonymous lifetime can be used where an explicit lifetime must be 
// specified, even though it is used only once

// struct Foo<'a>(&'a str);
//
// impl Foo<'_> {}
//
// fn foo(s: &str) -> Foo<'_> {
//     Foo(s)
// }

// The foo() function's return type could just be written as Foo,
// but the anonymous lifetime makes it clearer that the struct borrows something

// Foo<'a> = Foo has a lifetime parameter 'a

// A lifetime is a construct the compiler, more specificall, it's borrow checker
// uses to ensure all borrows are valid
// a variable's lifetime begins when it is created and ends when it is destroyed

// take for example the case were we borrow a variable via &
// The borrow has a lifetime that is determined by whre it is declared
// As a result, the borrow is valid as long as it ends before the lender is destroyed
// However the scope of the borrow is determined by where the reference is used


// write! macro [std::write]
// writes formatted data into a buffer

// std::fmt::Formatter
// configuration for formatting
// a Formatter represents various options related to formatting
// to interact with a Formatter, you'll call various methods to change
// the various options related to formatting


// ? is used at the end of an expression returning a Result
// and is equivalent to a match expression, where Err(err) branch
// expands to an early return Err(From::from(err))
// and the Ok(ok) branch expands to an ok expression


// The question mark operator (?) unwraps valid values 
// or returns erronous values, propagating them to the calling function
// ? can only be applied to the types Result<T, E> and Option<T>
// When applied to values of the Result<T, E> type, it propagates errors.
// If the vallue is Err(e), then it will returns Err(From::from(e)) from the 
// enclosing function or closure
// If applied to Ok(x) then it will unwrap the value to evaluate to 'x'


impl<T: Display> Display for Graph<T> {

    fn fmt(&self, 
           w: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {

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
        let mut graph = Graph::<i32>::new();

        let first = graph.add_vertex(1, &[]);

        let second = graph.add_vertex(2, &[first.clone()]);

        let third = graph.add_vertex(3, &[first.clone(), second]);

        
        assert_eq!(graph.has_path(first.clone(), third.clone()), true);

        assert_eq!(graph.has_path(third, first), false);       


    }

}






fn main() {

    let mut graph = Graph::<i32>::new();
    let first = graph.add_vertex(1, &[]);
    let second = graph.add_vertex(2, &[]);

    graph.has_path(first, second);

}



