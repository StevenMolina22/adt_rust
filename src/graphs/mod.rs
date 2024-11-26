use adjacency::Graph;

mod adjacency;

pub fn use_graph() {
    let mut g = Graph::new(5);
    g.add_edge(1, 3);
    g.add_edge(1, 2);
    g.add_edge(1, 1);
    g.add_edge(2, 4);

    println!("{}", g);
}
