#![allow(dead_code)]

use core::fmt;

#[derive(Default)]
pub struct Graph {
    edges: Vec<Vec<u32>>,
}

// ex:
// . A B C D
// A 0 1 1 1
// B 1 0 1 0
// C 1 1 0 1
// D 1 0 1 0

impl Graph {
    pub fn new(n_nodes: usize) -> Graph {
        Graph {
            edges: vec![vec![0; n_nodes]; n_nodes],
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
}

impl Graph {
    pub fn edges(&self) -> Vec<Vec<u32>> {
        self.edges.clone()
    }
}

impl fmt::Display for Graph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "digraph {{")?;

        self.edges.iter().enumerate().for_each(|(from, row)| {
            row.iter()
                .enumerate()
                .filter(|&(_, edge)| *edge != 0)
                .for_each(|(to, _)| {
                    let _ = writeln!(f, "\t{} -> {};", from, to);
                });
        });
        writeln!(f, "}}")
    }
}
