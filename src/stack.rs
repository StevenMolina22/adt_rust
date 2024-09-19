pub struct Stack<T> {
    top: Option<Box<Node<T>>>,
}

struct Node<T> {
    data: T,
    prev: Box<Node<T>>,
}

impl<T> Stack<T> {
    pub fn new() -> Stack<T> {
        Stack {
            top: None,
        }
    }
    fn push(&mut self, data: T) {
    }

    fn pop(&mut self) {
    }

    fn peek(&self) {
    }

    fn is_empty(&self) {
    }

    fn quantity(&self) {

    }
}
