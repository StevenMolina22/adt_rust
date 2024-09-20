mod linked;
mod sorted_linked;
mod stack;
// mod queue;

use stack::Stack;
use sorted_linked::SortedLinked;

fn main() {
    println!("Hello to different ADTs");

    // _use_stack();
    // _use_queue();

    // _use_sorted_list();
}

fn _use_sorted_list() {
    let mut list = SortedLinked::new();
    list.append(4);

    let first = list.first();

    if let Some(value) = first {
        println!("{}", value.data);
    }
}

fn _use_stack() {
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
}

fn _use_queue() {
}

fn _take_list(_list: SortedLinked<i32>) {}
