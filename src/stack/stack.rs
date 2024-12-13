#![allow(dead_code)]

pub struct Stack<T> {
    top: Option<Box<Node<T>>>,
    len: u32,
}

struct Node<T> {
    data: T,
    prev: Option<Box<Node<T>>>,
}

impl<T> Stack<T> {
    pub fn new() -> Stack<T> {
        Stack { top: None, len: 0 }
    }
    pub fn push(&mut self, data: T) {
        let node = Some(Box::new(Node {
            data,
            prev: self.top.take(),
        }));
        self.top = node;
        self.len += 1;
    }

    pub fn pop(&mut self) {
        // Beginner ownership explination:
        // node in this case takes ownership of the self.top Node ('take()' changes it to borrowed)
        if let Some(node) = self.top.take() {
            self.top = node.prev;
            self.len -= 1;
        }
    }

    pub fn peek(&self) -> Option<&T> {
        match &self.top {
            Some(node) => Some(&node.data),
            None => None,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn quantity(&self) -> u32 {
        self.len
    }
}
