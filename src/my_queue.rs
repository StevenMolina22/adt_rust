use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
pub struct Queue<T> {
    pub size: u32,
    pub front: Option<Rc<RefCell<Node<T>>>>,
    pub back: Option<Rc<RefCell<Node<T>>>>,
}

#[derive(Debug)]
pub struct Node<T> {
    pub data: T,
    pub next: Option<Rc<RefCell<Node<T>>>>,
}

impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue {
            size: 0,
            front: None,
            back: None,
        }
    }

    pub fn enqueue(&mut self, data: T) {
        let new_node = Rc::new(RefCell::new(Node { data, next: None }));

        match self.back.take() {
            Some(old_back) => {
                old_back.borrow_mut().next = Some(Rc::clone(&new_node));
                self.back = Some(Rc::clone(&new_node));
            }
            None => {
                self.front = Some(Rc::clone(&new_node));
                self.back = Some(Rc::clone(&new_node));
            }
        }

        self.size += 1;
    }

    pub fn dequeue(&mut self) -> Option<T>{
        if let Some(old_front) = self.front.take() {
            self.front = old_front.borrow_mut().next.take();
            if self.front.is_none() {
                self.back = None;
            }
            self.size -= 1;
            return Some(Rc::try_unwrap(old_front).ok().unwrap().into_inner().data);
        }
        None
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn len(&self) -> u32 {
        self.size
    }
}

// impl Iterator for Queue {
//     type Item = ;
// }
