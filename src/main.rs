mod linked;
mod sorted_linked;
mod stack;
mod queue;
mod my_queue;

use linked::LinkedList;
// use queue::Queue;
use sorted_linked::SortedLinked;
use stack::Stack;

fn main() {
    println!("Hello to different ADTs");
    println!();

    _use_stack();
    // _use_queue();

    _use_linked_list();
    _use_sorted_list();

    _use_my_queue();
}

fn _use_sorted_list() {
    println!("----- WELCOME TO SORTED LIST -----");

    let mut list = SortedLinked::new();
    list.append(4);

    let first = list.first();

    if let Some(value) = first {
        println!("{}", value.data);
    }
    println!()
}

fn _use_stack() {
    println!("----- WELCOME TO STACK -----");

    let mut stack: Stack<i32> = Stack::new();
    stack.push(3);
    if let Some(n) = stack.peek() {
        println!("{n}");
    }

    stack.pop();
    match stack.peek() {
        Some(n) => println!("{n}"),
        None => println!("Empty 😭"),
    }

    stack.push(5);
    stack.push(4);

    let n = stack.quantity();
    println!("Len is: {n}");

    let is_empty = stack.is_empty();

    if is_empty {
        println!("Its empty 😭");
    } else {
        println!("Its not empty 🤑");
    }

    println!()
}

// fn _use_queue() {
//     println!("----- WELCOME TO QUEUE -----");

//     let mut queue: Queue<i32> = Queue::new();
//     queue.enqueue(1);
//     queue.enqueue(2);
//     queue.enqueue(3);

//     println!("Queue after enqueuing 1, 2, 3:");
//     println!("Queue size: {}", queue.size());

//     while !queue.is_empty() {
//         if let Some(value) = queue.peek() {
//             println!("Front of the queue: {value}");
//         }
//         queue.dequeue();
//     }
//     println!()
// }

fn _use_linked_list() {
    println!("----- WELCOME TO LINKED LIST -----");

    let mut list: LinkedList<i32> = LinkedList::new();
    list.append(1);
    list.append(2);
    list.append(3);
    list.append(4);

    println!("Linked list after appending 1, 2, 3:");

    for value in list.iter() {
        println!("Value: {}", value);
    }

    let sum: i32 = list.iter().sum();
    println!("Sum of all elements: {}", sum);

    let even_squared: Vec<i32> = list.iter()
        .filter(|&&x| x % 2 == 0)
        .map(|&x| x * x)
        .collect();

    println!("Even numbers squared: {:?}", even_squared);
    println!()
}

fn _use_my_queue() {
    println!("----- WELCOME TO MY QUEUE -----");

    let mut my_queue: my_queue::Queue<i32> = my_queue::Queue::new();

    // Enqueue some elements
    my_queue.enqueue(10);
    my_queue.enqueue(20);
    my_queue.enqueue(30);

    println!("Queue after enqueuing 10, 20, 30:");
    println!("Queue size: {}", my_queue.size);

    // Print and dequeue elements
    while my_queue.size > 0 {
        if let Some(front) = &my_queue.front {
            let value = front.borrow().data;
            println!("Front of the queue: {}", value);
            my_queue.dequeue();
        }
    }

    println!("Queue is now empty. Size: {}", my_queue.size);
    println!()

}