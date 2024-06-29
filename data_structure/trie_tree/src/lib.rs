//! 
//! # TrieTree
//!
//! Define [`TrieTree`]
//! 
//! 
mod trie_node;
use std::hash::Hash;

use trie_node::TrieNode;
/// struct of TrieTree
pub struct TrieTree<T> {
    top: TrieNode<T>,
}
impl<T> TrieTree<T> 
where 
    T: Clone + Copy + Hash + Eq,
{
    /// Constructor of TrieTree with no data
    pub fn new() -> Self {
        Self {
            top: TrieNode::new(),
        }
    }

    /// Insert `s: Vec<T>`
    pub fn insert(&mut self, values: Vec<T>) {
        let mut node = &mut self.top;
        for value in values.iter() {
            node.pass();
            node.add_new_if_not(value);
            node = node.get_mut(value).unwrap();
        }
        node.stop();
    }

    /// Return the number of word starting with `values`
    pub fn n_match(&self, values: &Vec<T>) -> usize {
        let mut node = &self.top;
        values.iter().map(|value| {
            if node.contains_key(value) {
                node = node.get(value).unwrap();
                Some(node.n_pass() + node.n_stop())
            } else {
                None
            }
        }).fold(0, |acc, x| {
            if x == None {
                return acc;
            } else {
                acc + x.unwrap()
            }
        })
    }
}