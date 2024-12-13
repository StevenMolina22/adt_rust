#![allow(dead_code)]

// t: minimum degree
// k: max number of keys
// m: order of the tree
// n: number of keys in a given node

pub struct BTree<T: PartialOrd> {
    root: Option<Box<Node<T>>>,
    size: usize,
}

pub struct Node<T: PartialOrd> {
    keys: Vec<T>,
    children: Vec<Box<Node<T>>>,
    is_leaf: bool,
    degree: usize, // min degree (t)
}

impl<T: PartialOrd> BTree<T> {
    pub fn new(degree: usize) -> Self {
        Self {
            root: None,
            size: 0,
        }
    }

    pub fn insert(&mut self, data: T) {
        todo!()
    }

    pub fn remove(&mut self, data: T) -> bool {
        todo!()
    }

    pub fn get(&self, data: &T) -> Option<&T> {
        todo!()
    }

    pub fn size(&self) -> usize {
        todo!()
    }
}

impl<T: PartialOrd + Clone> Node<T> {
    pub fn new(degree: usize, is_leaf: bool) -> Self {
        Self {
            children: Vec::new(),
            degree,
            is_leaf,
            keys: Vec::new(),
        }
    }
}

trait Tree<T>
where
    T: PartialOrd,
{
    fn new(k: usize) -> Self;
    fn insert(&mut self, data: T);
    fn remove(&mut self, data: T) -> bool;
    fn get(&self, data: &T) -> Option<&T>;
    fn size(&self) -> usize;
}
