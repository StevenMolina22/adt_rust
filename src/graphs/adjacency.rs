use core::fmt;

#[derive(Default)]
pub struct Graph {
    edges: Vec<Vec<u32>>,
    n_nodes: usize,
}

impl Graph {
    pub fn new(n_nodes: usize) -> Graph {
        Graph {
            edges: vec![vec![0; n_nodes]; n_nodes],
            n_nodes,
        }
    }

    pub fn add_edge(&mut self, from: usize, to: usize) -> bool {
        if self.edges[from][to] != 0 {
            return false;
        }
        self.edges[from][to] = 1;
        true
    }

    pub fn remove_edge(&mut self, from: usize, to: usize) -> bool {
        if self.edges[from][to] == 0 {
            return false;
        }
        self.edges[from][to] = 0;
        true
    }

    pub fn has_edge(&self, from: usize, to: usize) -> bool {
        self.edges[from][to] != 0
    }

    pub fn display(&self) {
        println!("diagraph {{");

        self.edges.iter().enumerate().for_each(|(from, row)| {
            row.iter()
                .enumerate()
                .filter(|&(_, edge)| *edge != 0)
                .for_each(|(to, _)| {
                    println!("{} -> {}", from, to);
                });
        });
        println!("}}");
    }
}

impl fmt::Display for Graph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "diagraph {{")?;

        self.edges.iter().enumerate().for_each(|(from, row)| {
            row.iter()
                .enumerate()
                .filter(|&(_, edge)| *edge != 0)
                .for_each(|(to, _)| {
                    let _ = writeln!(f, "{} -> {}", from, to);
                });
        });
        writeln!(f, "}}")
    }
}
