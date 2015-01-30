use std::hash::Hash;
use std::collections::HashMap;
use std::collections::hash_map::Hasher;

use std::cell::RefCell;
use std::rc::Rc;
use std::rc::Weak;

/// Struct that represents the [Disjoint-Set](http://en.wikipedia.org/wiki/Disjoint-set_data_structure) data structure.
#[derive(Clone)]
pub struct DisjointSet<T> {
    elements: HashMap<T, Rc<RefCell<SubSet<T>>>>
}

impl<T> DisjointSet<T>
    where T: Eq + PartialEq + Hash<Hasher> + Clone
{
    pub fn new() -> DisjointSet<T> {
        DisjointSet {
            elements: HashMap::new()
        }
    }
    
    /// Makes a singleton set of the value inside the `DisjointSet`.
    pub fn make_set(&mut self, value: T) -> () {
        self.elements.insert(value.clone(), Rc::new(RefCell::new(SubSet::new(value))));
    }
    
    /// Finds the value of the root of the set that the value belongs to and performs path compression on the visited nodes.
    ///
    /// Returns `None` if the value is not in the `DisjointSet`.
    pub fn find(&mut self, value: T) -> Option<T> {
        let mut root;
        let mut changed_nodes = Vec::new();
        
        // Finding the root
        match self.elements.get(&value) {
            None => return None,
            Some(n) => {
                root = n.clone();
                while root.borrow().parent.is_some() {
                    changed_nodes.push(root.clone());
                    root = root.borrow().parent.as_ref().unwrap().clone().upgrade().unwrap();
                }
            }
        }
        
        // Path compression on visited nodes
        for changed_node in changed_nodes.iter() {
            changed_node.borrow_mut().parent = Some(root.clone().downgrade());
        }
        
        Some(root.borrow().value.clone())
    }
    
    /// Unions the two sets that each value belongs to using union by rank.
    ///
    /// Returns `None` if one of the values does not exist in the `DisjointSet`.
    pub fn union(&mut self, value_one: T, value_two: T) -> Option<T> {
        let root_one;
        match self.find(value_one) {
            Some(r) => root_one = r,
            None => return None
        }
        
        let root_two;
        match self.find(value_two) {
            Some(r) => root_two = r,
            None => return None
        }
        
        let root_one_pointer = self.elements.get(&root_one).unwrap().clone();
        let root_two_pointer = self.elements.get(&root_two).unwrap().clone();
        
        let root_one_rank = root_one_pointer.borrow().rank;
        let root_two_rank = root_two_pointer.borrow().rank;
        
        if root_one == root_two {
            return Some(root_one);
        }
        
        if root_one_rank < root_two_rank {
            root_one_pointer.borrow_mut().parent = Some(root_two_pointer.clone().downgrade());
            return Some(root_one);
        } else if root_one_rank > root_two_rank {
            root_two_pointer.borrow_mut().parent = Some(root_one_pointer.clone().downgrade());
            return Some(root_two);
        } else {
            root_two_pointer.borrow_mut().parent = Some(root_one_pointer.clone().downgrade());
            root_one_pointer.borrow_mut().rank = root_one_rank + 1;
            return Some(root_two);
        }
    }
}



#[derive(Clone)]
struct SubSet<T> {
    rank: u32,
    value: T,
    parent: Option<Weak<RefCell<SubSet<T>>>>
}

impl<T> SubSet<T> {
    fn new(value: T) -> SubSet<T> {
        SubSet {
            rank: 0,
            value: value,
            parent: None
        }
    }
}