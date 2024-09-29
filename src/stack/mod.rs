use stack::Stack;

pub mod stack;

pub fn _use_stack() {
    println!("----- WELCOME TO STACK -----");

    let mut stack = Stack::new();

    println!("Adding 3, 5, 4");
    stack.push(3);
    stack.push(5);
    stack.push(4);

    println!("Last is: {}", stack.peek().unwrap());
    stack.pop();
    println!("After pop, last is: {}", stack.peek().unwrap());

    println!("Removing all...");
    stack.pop();
    stack.pop();

    println!("Is it empty now? 🤑 {}", stack.is_empty());
    println!();
}
