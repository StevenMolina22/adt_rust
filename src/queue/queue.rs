#![allow(dead_code)]
// its not trivial to implement a queue in rust
// because multiple reference are needed for the same data
// this shows the neccesity to use things like ref countint and ref cell

// COPIED IMPLEMENTATION, JUST FOR REFERENCE
use std::cell::RefCell;
use std::rc::Rc;

pub struct Queue<T> {
    first: Option<Rc<RefCell<Node<T>>>>,
    rear: Option<Rc<RefCell<Node<T>>>>,
    size: u32,
}

struct Node<T> {
    data: T,
    next: Option<Rc<RefCell<Node<T>>>>,
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Queue {
            first: None,
            rear: None,
            size: 0,
        }
    }

    pub fn enqueue(&mut self, data: T) {
        let new_node = Rc::new(RefCell::new(Node { data, next: None }));

        match self.rear.take() {
            Some(old_back) => {
                old_back.borrow_mut().next = Some(Rc::clone(&new_node));
                self.rear = Some(new_node);
            }
            None => {
                self.first = Some(Rc::clone(&new_node));
                self.rear = Some(new_node);
            }
        }

        self.size += 1;
    }

    pub fn dequeue(&mut self) -> Option<T> {
        self.first.take().map(|old_first| {
            if let Some(new_first) = old_first.borrow_mut().next.take() {
                self.first = Some(new_first);
            } else {
                self.rear = None;
            }
            self.size -= 1;
            Rc::try_unwrap(old_first).ok().unwrap().into_inner().data
        })
    }

    pub fn peek(&self) -> Option<T>
    where
        T: Clone,
    {
        self.first.as_ref().map(|node| node.borrow().data.clone())
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn size(&self) -> u32 {
        self.size
    }
}
