**Project Description:**
`adt-rust` is a Rust project that provides implementations of various abstract data types (ADTs). This project aims to offer a collection of data structures that are commonly used in computer science, such as linked lists, stacks, queues, trees, heaps, and graphs. Each ADT is implemented with a focus on demonstrating the idiomatic usage of Rust's type system and memory management features.

**Implemented ADTs:**

1. **LinkedList**

   - A singly linked list where each node contains a data element and a pointer to the next node.
   - Supports appending elements and provides an iterator for traversing the list.
   - Example:
     ```rust
     let mut list = LinkedList::new();
     list.append(1);
     list.append(2);
     list.append(3);
     for value in list.iter() {
         println!("{}", value);
     }
     ```

2. **Stack**

   - A stack data structure with push, pop, and peek operations.
   - Uses a linked list under the hood to manage the elements.
   - Provides methods to check if the stack is empty and to get the number of elements.
   - Example:
     ```rust
     let mut stack = Stack::new();
     stack.push(1);
     stack.push(2);
     println!("{}", stack.peek().unwrap()); // Outputs: 2
     stack.pop();
     println!("{}", stack.peek().unwrap()); // Outputs: 1
     ```

3. **Queue**

   - A queue data structure implemented using reference counting (`Rc`) and interior mutability (`RefCell`).
   - Supports enqueue and dequeue operations.
   - Provides methods to check if the queue is empty and to get the size of the queue.
   - Example:
     ```rust
     let mut queue = Queue::new();
     queue.enqueue(1);
     queue.enqueue(2);
     println!("{}", queue.dequeue().unwrap()); // Outputs: 1
     println!("{}", queue.dequeue().unwrap()); // Outputs: 2
     ```

4. **MyQueue**

   - Another implementation of a queue with similar functionality to `Queue`.
   - Uses `Rc` and `RefCell` for managing nodes.
   - Example:
     ```rust
     let mut my_queue = MyQueue::new();
     my_queue.enqueue(1);
     my_queue.enqueue(2);
     println!("{}", my_queue.dequeue().unwrap()); // Outputs: 1
     println!("{}", my_queue.dequeue().unwrap()); // Outputs: 2
     ```

5. **Trees**

   - **Binary Search Tree (BST)**

     - A binary search tree where each node has at most two children.
     - Supports insertion, deletion, and search operations.
     - Example:
       ```rust
       let mut bst = BST::new();
       bst.insert(10);
       bst.insert(5);
       bst.insert(15);
       println!("{}", bst.get(&10).unwrap()); // Outputs: 10
       bst.remove(10);
       ```

   - **B-Tree**
     - A self-balancing tree data structure that maintains sorted data and allows searches, sequential access, insertions, and deletions in logarithmic time.
     - Example:
       ```rust
       let mut btree = BTree::new(3); // Minimum degree 3
       btree.insert(10);
       btree.insert(20);
       println!("{}", btree.get(&10).unwrap()); // Outputs: 10
       btree.remove(10);
       ```

6. **Heap**

   - A binary heap data structure that supports efficient retrieval of the maximum (or minimum) element.
   - Example:
     ```rust
     let mut heap = vec![3, 2, 1, 5, 4];
     build(&mut heap);
     println!("{:?}", heap); // Outputs a valid max heap
     ```

7. **Graphs**

   - **Adjacency List**

     - A graph implementation using an adjacency list.
     - Example:
       ```rust
       let mut graph = ListGraph::new(5);
       graph.add_edge(1, 3);
       graph.add_edge(1, 2);
       println!("{}", graph);
       ```

   - **Adjacency Matrix**

     - A graph implementation using an adjacency matrix.
     - Example:
       ```rust
       let mut graph = MatrixGraph::new(5);
       graph.add_edge(1, 3);
       graph.add_edge(1, 2);
       println!("{}", graph);
       ```

   - **Generic Graph**
     - A graph implementation that can handle generic node types.
     - Example:
       ```rust
       let mut graph = GenericGraph::new();
       graph.add_edge("A", "B");
       graph.add_edge("A", "C");
       println!("{}", graph);
       ```

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
- **`src/lists/linked.rs`:** Implementation of the `LinkedList` data structure.
- **`src/stack/stack.rs`:** Implementation of the `Stack` data structure.
- **`src/queue/queue.rs`:** Implementation of the `Queue` data structure.
- **`src/queue/my_queue.rs`:** Implementation of an alternative `Queue` data structure.
- **`src/trees/bst.rs`:** Implementation of the `Binary Search Tree` data structure.
- **`src/trees/btree.rs`:** Implementation of the `B-Tree` data structure.
- **`src/heap/heap.rs`:** Implementation of the `Heap` data structure.
- **`src/graphs/list.rs`:** Implementation of the `Adjacency List` graph data structure.
- **`src/graphs/matrix.rs`:** Implementation of the `Adjacency Matrix` graph data structure.
- **`src/graphs/generic.rs`:** Implementation of the `Generic Graph` data structure.
