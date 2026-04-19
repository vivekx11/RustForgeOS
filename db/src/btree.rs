//! B-Tree implementation the system 

use alloc::vec::Vec;
use alloc::boxed::Box;

const ORDER: usize = 128;

#[derive(Debug)]
pub struct BTree<K, V> {
    root: Option<Box<Node<K, V>>>,
}

#[derive(Debug)]
struct Node<K, V> {
    keys: Vec<K>,
    values: Vec<V>,
    children: Vec<Box<Node<K, V>>>,
    is_leaf: bool,
}

impl<K: Ord, V> BTree<K, V> {
    pub fn new() -> Self {
        Self { root: None }
    }

    pub fn insert(&mut self, key: K, value: V) {
        // TODO: Implement B-Tree insert
    }

    pub fn search(&self, key: &K) -> Option<&V> {
        // TODO: Implement B-Tree search
        None
    }

    pub fn delete(&mut self, key: &K) -> Option<V> {
        // TODO: Implement B-Tree delete
        None
    }
}
