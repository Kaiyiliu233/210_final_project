use std::collections::{HashSet, HashMap};
use crate::ParsedEmail;

/// Struct to represent a directed, unweighted graph using an adjacency list
#[derive(Debug)]
pub struct Graph {
    pub num_vertices: usize, // Number of unique nodes in the graph
    pub adjacency_list: HashMap<String, HashSet<String>>, // Adjacency list mapping each node to its neighbors
}

impl Graph {
    /// Creates a new, empty Graph.
    pub fn new() -> Self {
        Graph {
            num_vertices: 0,
            adjacency_list: HashMap::new(),
        }
    }

    /// Adds an edge from `from_node` to `to_node`.
    /// Increments `num_vertices` if a new node is added.
    pub fn add_edge(&mut self, from_node: String, to_node: String) {
        // Insert sender node if it doesn't exist; increment num_vertices
        self.adjacency_list.entry(from_node.clone())
            .or_insert_with(|| {
                self.num_vertices += 1;
                HashSet::new()
            });

        // Insert recipient node if it doesn't exist; increment num_vertices
        self.adjacency_list.entry(to_node.clone())
            .or_insert_with(|| {
                self.num_vertices += 1;
                HashSet::new()
            });

        // Add the edge
        self.adjacency_list
            .get_mut(&from_node)
            .unwrap()
            .insert(to_node.clone());
        }

    /// Builds the graph from a list of parsed emails.
    pub fn build_from_emails(parsed_emails: Vec<ParsedEmail>) -> Self {
        let mut graph = Graph::new();

        for email in parsed_emails {
            let sender = email.from;
            let recipients = email.to;

            for recipient in recipients {
                graph.add_edge(sender.clone(), recipient);
            }
        }

        graph
    }

    /// Returns the neighbors of a given node.
    pub fn get_neighbors(&self, node: &String) -> Option<&HashSet<String>> {
        self.adjacency_list.get(node)
    }

    /// Calculates the out-degree for each node.
    pub fn calculate_out_degrees(&self) -> HashMap<String, usize> {
        let mut out_degrees = HashMap::new();

        for node in self.adjacency_list.keys() {
            let degree = self
                .get_neighbors(node)
                .map_or(0, |neighbors| neighbors.len());
            out_degrees.insert(node.clone(), degree);
        }

        out_degrees
    }

    /// Calculates the in-degree for each node.
    pub fn calculate_in_degrees(&self) -> HashMap<String, usize> {
        let mut in_degrees = HashMap::new();

        // Initialize in-degrees to zero
        for node in self.adjacency_list.keys() {
            in_degrees.insert(node.clone(), 0);
        }

        // Count in-degrees
        for neighbors in self.adjacency_list.values() {
            for neighbor in neighbors {
                if let Some(count) = in_degrees.get_mut(neighbor) {
                    *count += 1;
                }
            }
        }

        in_degrees
    }
}

