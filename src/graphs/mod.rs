use list::Graph as ListGraph;
use matrix::Graph as MatrixGraph;

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
