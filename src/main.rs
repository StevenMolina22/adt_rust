use std::io;

mod graphs;
mod iters;
mod lists;
mod queue;
mod stack;

fn main() {
    show_options();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read");
    let input = input.trim();

    match input {
        "1" => stack::use_stack(),
        "2" => iters::use_my_iter(),
        "3" => queue::use_queue(),
        "4" => lists::use_list(),
        "5" => lists::use_sorte_l(),
        "6" => graphs::use_graph(),
        _ => (),
    }
}

fn show_options() {
    println!();
    println!("----- --- WELCOME TO DATA STRUCTURES --- -----");
    println!();

    println!("Select option: ");
    println!("\tStack: 1");
    println!("\tIter: 2");
    println!("\tQueue: 3");
    println!("\tList: 4");
    println!("\tSorted List: 5");
    println!("\tGraph: 6");

    println!()
}
