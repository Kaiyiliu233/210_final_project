use std::collections::{HashSet, HashMap};
use crate::ParsedEmail;
use rand::seq::SliceRandom;
use rand::thread_rng;

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

    /// Performs community detection using the Label Propagation Algorithm.
    /// Returns a HashMap where each node is mapped to its community label.
    pub fn label_propagation(&self) -> HashMap<String, String> {
        // Initialize labels: each node is its own label
        let mut labels: HashMap<String, String> = self.adjacency_list
            .keys()
            .map(|node| (node.clone(), node.clone()))
            .collect();

        let mut rng = thread_rng();

        let max_iterations = 100; // Prevent infinite loops
        for iteration in 0..max_iterations {
            let mut changed = false;

            // Collect all nodes and shuffle their order for random updates
            let mut nodes: Vec<&String> = self.adjacency_list.keys().collect();
            nodes.shuffle(&mut rng);

            for node in nodes {
                let neighbors = match self.get_neighbors(node) {
                    Some(neigh) => neigh,
                    None => continue, // Isolated node
                };

                if neighbors.is_empty() {
                    continue; // No neighbors to influence the label
                }

                // Count the frequency of each label in the neighborhood
                let mut label_counts: HashMap<&String, usize> = HashMap::new();
                for neighbor in neighbors {
                    if let Some(label) = labels.get(neighbor) {
                        *label_counts.entry(label).or_insert(0) += 1;
                    }
                }

                // Find the label(s) with the highest count
                if let Some((&max_label, &max_count)) = label_counts.iter().max_by_key(|&(_, count)| count) {
                    let current_label = labels.get(node).unwrap();
                    if current_label != max_label {
                        labels.insert(node.clone(), max_label.clone());
                        changed = true;
                    }
                }
            }

            println!("Iteration {}: Labels changed: {}", iteration + 1, changed);

            if !changed {
                println!("Converged after {} iterations.", iteration + 1);
                break;
            }
        }

        labels
    }
}

