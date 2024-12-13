#![allow(dead_code)]

pub struct BST<T> {
    size: usize,
    root: Option<Box<Node<T>>>,
}

pub struct Node<T> {
    data: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl<T: PartialOrd> Node<T> {
    pub fn new(data: T) -> Self {
        Node {
            data,
            left: None,
            right: None,
        }
    }
}

impl<T: PartialOrd + Clone> BST<T> {
    pub fn new() -> Self {
        BST {
            size: 0,
            root: None,
        }
    }

    pub fn insert(&mut self, data: T) {
        self.root = Self::insert_from(self.root.take(), data);
        self.size += 1;
    }

    pub fn remove(&mut self, data: T) -> bool {
        let (new_root, removed) = Self::remove_from(self.root.take(), data);
        self.root = new_root;
        if removed {
            self.size -= 1;
        }
        removed
    }

    pub fn get(&self, data: &T) -> Option<&T> {
        let found = Self::get_from(&self.root, data.clone());
        match found {
            Some(node) => Some(&node.data),
            None => None,
        }
    }

    pub fn size(&self) -> usize {
        self.size
    }
}

impl<T: PartialOrd + Clone> BST<T> {
    fn get_from(node: &Option<Box<Node<T>>>, data: T) -> Option<&Box<Node<T>>> {
        match node {
            None => None,
            Some(node) if data > node.data => Self::get_from(&node.right, data),
            Some(node) if data < node.data => Self::get_from(&node.left, data),
            Some(node) => Some(node),
        }
    }

    fn insert_from(node: Option<Box<Node<T>>>, data: T) -> Option<Box<Node<T>>> {
        match node {
            None => Some(Box::new(Node::new(data))),
            Some(mut node) => {
                if data > node.data {
                    node.right = Self::insert_from(node.right, data);
                } else {
                    node.left = Self::insert_from(node.left, data);
                }
                Some(node)
            }
        }
    }

    fn remove_from(node: Option<Box<Node<T>>>, data: T) -> (Option<Box<Node<T>>>, bool) {
        match node {
            None => (None, false),
            Some(mut node) => {
                if data > node.data {
                    let (right, removed) = Self::remove_from(node.right, data);
                    node.right = right;
                    (Some(node), removed)
                } else if data < node.data {
                    let (left, removed) = Self::remove_from(node.left, data);
                    node.left = left;
                    (Some(node), removed)
                } else {
                    // to remove found
                    match (node.left.take(), node.right.take()) {
                        (None, None) => (None, true),
                        (Some(left), None) => (Some(left), true),
                        (None, Some(right)) => (Some(right), true),
                        (Some(left), Some(right)) => {
                            let min = Self::find_min(&right).clone();
                            node.data = min.clone();
                            let (new_right, removed) = Self::remove_from(Some(right), min);
                            node.right = new_right;
                            node.left = Some(left);
                            (Some(node), removed)
                        }
                    }
                }
            }
        }
    }

    fn find_min(node: &Box<Node<T>>) -> &T {
        match &node.left {
            None => &node.data,
            Some(left) => Self::find_min(&left),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bst_insert_and_get() {
        let mut bst = BST::new();

        bst.insert(10);
        bst.insert(5);
        bst.insert(15);
        bst.insert(2);
        bst.insert(7);

        assert_eq!(bst.size(), 5);

        assert_eq!(bst.get(&10), Some(&10));
        assert_eq!(bst.get(&5), Some(&5));
        assert_eq!(bst.get(&15), Some(&15));
        assert_eq!(bst.get(&2), Some(&2));
        assert_eq!(bst.get(&7), Some(&7));

        assert_eq!(bst.get(&20), None);
        assert_eq!(bst.get(&1), None);
    }

    #[test]
    fn test_bst_remove() {
        let mut bst = BST::new();

        bst.insert(10);
        bst.insert(5);
        bst.insert(15);
        bst.insert(2);
        bst.insert(7);

        assert!(bst.remove(2));
        assert_eq!(bst.size(), 4);
        assert_eq!(bst.get(&2), None);

        assert!(bst.remove(5));
        assert_eq!(bst.size(), 3);
        assert_eq!(bst.get(&5), None);

        assert!(bst.remove(10));
        assert_eq!(bst.size(), 2);
        assert_eq!(bst.get(&10), None);

        assert!(!bst.remove(20));
        assert_eq!(bst.size(), 2);

        assert!(bst.remove(15));
        assert!(bst.remove(7));
        assert_eq!(bst.size(), 0);
    }

    #[test]
    fn test_bst_edge_cases() {
        let mut bst = BST::new();

        assert!(!bst.remove(10));
        assert_eq!(bst.size(), 0);

        bst.insert(10);
        assert_eq!(bst.size(), 1);
        assert!(bst.remove(10));
        assert_eq!(bst.size(), 0);

        assert_eq!(bst.get(&10), None);
    }
}
