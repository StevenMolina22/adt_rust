use std::io;

mod graphs;
mod heap;
mod iters;
mod lists;
mod queue;
mod stack;
mod trees;

fn main() {
    show_options();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read");
    let input = input.trim();

    let command = Command::from_input(input.trim());
    command.execute();
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

enum Command {
    Stack,
    Iter,
    Queue,
    List,
    SortedList,
    Graph,
    Invalid,
}

impl Command {
    fn from_input(input: &str) -> Self {
        match input {
            "1" => Command::Stack,
            "2" => Command::Iter,
            "3" => Command::Queue,
            "4" => Command::List,
            "5" => Command::SortedList,
            "6" => Command::Graph,
            _ => Command::Invalid,
        }
    }

    fn execute(&self) {
        match self {
            Command::Stack => stack::use_stack(),
            Command::Iter => iters::use_my_iter(),
            Command::Queue => queue::use_queue(),
            Command::List => lists::use_list(),
            Command::SortedList => lists::use_sorte_l(),
            Command::Graph => graphs::use_graph_generic(),
            Command::Invalid => println!("Invalid option. Please try again."),
        }
    }
}
