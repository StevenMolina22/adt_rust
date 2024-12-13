use generic::Graph as GenericGraph;
use list::Graph as ListGraph;
use matrix::Graph as MatrixGraph;

pub mod generic;
mod list;
mod matrix;

pub fn use_graph() {
    let mut g = ListGraph::new(5);
    g.add_edge(1, 3);
    g.add_edge(1, 2);
    g.add_edge(1, 1);
    g.add_edge(2, 4);
    g.remove_edge(2, 4);

    println!("{}", g);
    println!("Edge 1, 2 {}", g.has_edge(1, 2));
    println!("Edge 2, 4 {}", g.has_edge(2, 4));
}

pub fn use_graph_from() {
    let mut g = MatrixGraph::new(5);
    g.add_edge(1, 3);
    g.add_edge(1, 2);
    g.add_edge(1, 1);
    g.add_edge(2, 4);

    println!("{}", ListGraph::from(g.edges()));
}

pub fn use_graph_generic() {
    let mut g = GenericGraph::new();
    g.add_edge("A", "C");
    g.add_edge("A", "B");
    g.add_edge("A", "A");
    g.add_edge("B", "D");

    println!("{}", g);
}
