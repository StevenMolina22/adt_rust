mod iters;
mod lists;
mod queue;
mod stack;

fn main() {
    println!();
    println!("----- --- WELCOME TO DATA STRUCTURES --- -----");
    println!();

    stack::_use_stack();
    iters::_use_my_iter();
    queue::_use_my_queue();
    lists::_use_linked();
    lists::_use_sorted();
}
