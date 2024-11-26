pub mod my_queue;
pub mod queue;

pub fn use_queue() {
    println!("----- WELCOME TO MY QUEUE -----");

    let mut my_queue = my_queue::Queue::new();

    // Enqueue some elements
    my_queue.enqueue(10);
    my_queue.enqueue(20);
    my_queue.enqueue(30);

    println!("Queue after enqueuing 10, 20, 30:");
    println!("Queue size: {}", my_queue.size());

    println!("Peek at first: {:?}", my_queue.peek().unwrap());

    // Print and dequeue elements
    println!("Removing elements...");
    println!("Dequeue: {:?}", my_queue.dequeue());
    println!("Dequeue: {:?}", my_queue.dequeue());
    println!("Dequeue: {:?}", my_queue.dequeue());

    println!(
        "Queue is now empty? {} Size: {}",
        my_queue.is_empty(),
        my_queue.size()
    );
    println!()
}
