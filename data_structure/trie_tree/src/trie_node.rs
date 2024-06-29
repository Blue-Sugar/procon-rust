use std::hash::Hash;

type HashMap<K, V> = std::collections::HashMap<K, V>;
/// struct for TrieTree's node.
#[derive(Clone)]
pub struct TrieNode<T> {
    pass: usize,
    stop: usize,
    children: HashMap<T, TrieNode<T>>,
}
impl<T> TrieNode<T> 
where
    T: Clone + Copy + Hash + Eq,
{
    /// Constructor of TrieNode
    pub fn new() -> Self {
        Self {
            pass: 0,
            stop: 0,
            children: HashMap::new(),
        }
    }

    /// Return if there is child of value
    pub fn contains_key(&self, value: &T) -> bool {
        self.children.contains_key(value)
    }

    /// Return children of `value`.
    pub fn get(&self, value: &T) -> Option<&Self> {
        self.children.get(value)
    }

    /// Return children of `value` as mutable.
    pub fn get_mut(&mut self, value: &T) -> Option<&mut Self> {
        self.children.get_mut(value)
    }

    /// Insert new node
    pub fn add_new_if_not(&mut self, value: &T) -> bool {
        if self.children.contains_key(value) {
            return false;
        }
        self.children.insert(*value, Self::new());
        true
    } 

    /// Pass the node
    pub fn pass(&mut self) {
        self.pass += 1;
    }

    /// Stop the node
    pub fn stop(&mut self) {
        self.stop += 1;
    }

    /// Return how many times pass
    pub fn n_pass(&self) -> usize {
        self.pass
    }

    /// Return how many times stop
    pub fn n_stop(&self) -> usize {
        self.stop
    }
}