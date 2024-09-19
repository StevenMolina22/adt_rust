use std::iter::Iterator;

#[derive(Debug)]
pub struct Node<T>  {
    pub data: T,
    pub next: Option<Box<Node<T>>>,
}

pub struct LinkedList<T> {
    pub head: Option<Box<Node<T>>>,
}

// Note: (ref mute) like &mut but for pattern matching
impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList { head: None }
    }

    pub fn append(&mut self, data: T) {
        let new_node = Box::new(Node { data, next: None });

        // head will be mutated (it's not mut yet)
        // so it needs the explicit mutability
        match self.head.as_mut() {
            None => self.head = Some(new_node),
            Some(mut current) => {
                // patterns match, till current.next == None
                // ref mut used for mut borrow (?)
                while let Some(ref mut next) = current.next {
                    current = next;
                }
                current.next = Some(new_node);
            }
        }
    }

    // don't ask me how this iterator works
    pub fn iter(&self) -> LinkedListIterator<T> {
        LinkedListIterator {
            next: self.head.as_deref(),
        }
    }
}

// don't ask me who implemented this
pub struct LinkedListIterator<'a, T> {
    next: Option<&'a Node<T>>,
}

// wtf is that ' for
impl<'a, T> Iterator for LinkedListIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.data
        })
    }
}

// making this linked list work was pure suffering, f you gemini

//impl<T> Node<T> {
//    pub fn new() -> Node<T>{
//        Node {
//            data: None,
//            next: None,
//        }
//    }
//}