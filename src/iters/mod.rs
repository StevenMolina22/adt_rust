use iter::Counter;

pub mod iter;

pub fn _use_my_iter() {
    println!("----- WELCOME TO MY ITER -----");
    let counter = Counter::new();

    println!("Iterating over counter...");
    for number in counter {
        println!("Value: {number}");
    }
    println!();
}
