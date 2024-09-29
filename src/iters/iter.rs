pub struct Counter {
    count: u32,
}

impl Counter {
    pub fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32; // Define the type of items produced by the iterator

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count) // Return the next number
        } else {
            None // End of iteration
        }
    }
}
