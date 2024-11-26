**Project Description:**
`adt-rust` is a Rust project that provides implementations of various abstract data types (ADTs). This project aims to offer a collection of data structures that are commonly used in computer science, such as linked lists, stacks, and queues. Each ADT is implemented with a focus on demonstrating the idiomatic usage of Rust's type system and memory management features.

**Implemented ADTs:**
1. **LinkedList**
    - A singly linked list where each node contains a data element and a pointer to the next node.
    - Supports appending elements and provides an iterator for traversing the list.

2. **Stack**
    - A stack data structure with push, pop, and peek operations.
    - Uses a linked list under the hood to manage the elements.
    - Provides methods to check if the stack is empty and to get the number of elements.

3. **Queue**
    - A queue data structure implemented using reference counting (`Rc`) and interior mutability (`RefCell`).
    - Supports enqueue and dequeue operations.
    - Provides methods to check if the queue is empty and to get the size of the queue.

4. **MyQueue**
    - Another implementation of a queue with similar functionality to `Queue`.
    - Uses `Rc` and `RefCell` for managing nodes.

**Main Functionalities:**
- The main entry point of the project is in `src/main.rs`, where various ADTs are used and demonstrated.
- The `main` function showcases how to create and manipulate instances of the different ADTs.
- The project includes several example functions that illustrate the usage of each ADT, such as appending elements to a linked list, pushing elements onto a stack, and enqueuing elements in a queue.

**Dependencies:**
- The project relies on standard Rust libraries and does not have external dependencies.

**Setup Instructions:**
1. **Prerequisites:**
    - Ensure you have Rust installed. You can download it from [rust-lang.org](https://www.rust-lang.org/).

2. **Cloning the Repository:**
    ```sh
    git clone https://github.com/StevenMolina22/adt-rust.git
    cd adt-rust
    ```

3. **Building the Project:**
    ```sh
    cargo build
    ```

4. **Running the Examples:**
    ```sh
    cargo run
    ```

**Usage:**
- The project demonstrates the usage of ADTs through example functions in the main file.
- Users can modify these examples or add their own to further explore the capabilities of the ADTs.

**File Structure:**
- **`src/main.rs`:** Contains the main entry point and example usages of the ADTs.
- **`src/linked.rs`:** Implementation of the `LinkedList` data structure.
- **`src/stack.rs`:** Implementation of the `Stack` data structure.
- **`src/queue.rs`:** Implementation of the `Queue` data structure.
- **`src/my_queue.rs`:** Implementation of an alternative `Queue` data structure.
