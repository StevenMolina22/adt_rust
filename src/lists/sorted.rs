#![allow(dead_code)]

pub struct Node<T> {
    pub data: T,
    next: Option<Box<Node<T>>>,
}

pub struct SortedLinked<T> {
    head: Option<Box<Node<T>>>,
}
impl<T> SortedLinked<T> {
    pub fn new() -> SortedLinked<T> {
        SortedLinked { head: None }
    }

    pub fn append(&mut self, data: T) {
        let new_node = Box::new(Node { data, next: None });

        match self.head.as_mut() {
            None => self.head = Some(new_node),
            Some(mut current) => {
                // loop till next node is None
                while let Some(ref mut next) = current.next {
                    current = next;
                }
                current.next = Some(new_node);
            }
        };
    }

    pub fn first(&self) -> Option<&Box<Node<T>>> {
        // as ref: method to take a value inside an option not owned but borrowed
        // &self.head would make the option itself borrowed not its inside
        self.head.as_ref()
    }

    pub fn iter(&self) -> SLIter<T> {
        SLIter {
            next: self.head.as_deref(),
        }
    }
}

// ----------------- Iterator -----------------
pub struct SLIter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for SLIter<'a, T> {
    // Notes:
    //      As Deref: (Option<T>) -> (Option<&T>)
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.data
        })
    }
}
// impl<T> Iterator for SortedLinked<T> {
//     type Item = T;

//     fn next(&mut self) -> Option<Self::Item> {}
// }
