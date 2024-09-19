mod linked;
mod sorted_linked;
mod stack;

use stack::Stack;

use crate::sorted_linked::SortedLinked;

fn main() {
    println!("Hello, world!");

    // stack implementation
    let mut stack: Stack<i32> = Stack::new();
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

fn _take_list(_list: SortedLinked<i32>) {}
