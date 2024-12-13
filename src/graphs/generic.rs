#![allow(dead_code)]

use std::collections::HashMap;
use std::fmt;
use std::fmt::Formatter;

pub struct Graph {
    edges: HashMap<String, Vec<String>>,
}

// ex:
// A: B C D
// B: A C
// C: A B D
// D: A C

impl Graph {
    pub fn new() -> Self {
        Graph {
            edges: HashMap::new(),
            // n_nodes,
        }
    }

    // pub fn from(matrix: Vec<Vec<&str>>) -> Self {
    //     let mut g = Graph {
    //         edges: HashMap::new(),
    //         // n_nodes: matrix.len(),
    //     };
    //
    //     for (i, row) in matrix.iter().enumerate() {
    //         for (j, v) in row.iter().enumerate() {
    //             if *v != 0 {
    //                 g.add_edge(i as &str, j as &str);
    //             }
    //         }
    //     }
    //     g
    // }

    pub fn add_edge(&mut self, from: &str, to: &str) -> bool {
        self.edges
            .entry(from.to_string())
            .and_modify(|vec| vec.push(to.to_string()))
            .or_insert(vec![to.to_string()]);
        true
    }

    pub fn remove_edge(&mut self, from: &str, to: &str) -> bool {
        self.edges
            .entry(from.to_string())
            .and_modify(|vec| vec.retain(|x| *x != to));
        true
    }

    pub fn has_edge(&mut self, from: &str, to: &str) -> bool {
        match self.edges.get(&from.to_string()) {
            None => false,
            Some(neighbors) => neighbors.iter().any(|v| *v == to),
        }
    }
}

impl Graph {
    pub fn depth_iter(&self) {}
    pub fn breath_iter(&self) {}
}

impl fmt::Display for Graph {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "digraph {{")?;

        self.edges.iter().for_each(|(k, v)| {
            v.iter().for_each(|v2| {
                let _ = writeln!(f, "\t{} -> {};", k, v2);
            })
        });
        writeln!(f, "}}")
    }
}
