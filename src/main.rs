pub mod email;
pub mod graph;


use email::{ParsedEmail, read_csv};
use graph::{Graph};
use std::error::Error;
use std::collections::{HashMap, HashSet};

fn main() -> Result<(), Box<dyn Error>> {
    // Path to your CSV file
    let file_path = "test.csv";

    // Step 1: Read and parse the CSV
    let parsed_emails = read_csv(file_path)?;

    // Step 2: Build the graph
    let graph = Graph::build_from_emails(parsed_emails);
    for (i, (node, neighbors)) in graph.adjacency_list.iter().enumerate() {
        if node == "phillip.allen@enron.com" {
            println!("Node {}: {:?}", node, neighbors);
        }   
    }
    // Step 3: Perform analyses
    // Degree calculations
    let out_degrees = graph.calculate_out_degrees();
    let in_degrees = graph.calculate_in_degrees();

    // Print some degree statistics
    let max_out_degree = out_degrees.values().cloned().max().unwrap_or(0);
    let max_in_degree = in_degrees.values().cloned().max().unwrap_or(0);
    println!("Maximum out-degree: {}", max_out_degree);
    println!("Maximum in-degree: {}", max_in_degree);

    Ok(())
}

#[test]
fn test_out_degree_calculation() {
    // Create sample parsed emails
    let emails = vec![
        ParsedEmail {
            from: "alice@example.com".to_string(),
            to: vec![
                "bob@example.com".to_string(),
                "carol@example.com".to_string(),
            ],
        },
        ParsedEmail {
            from: "bob@example.com".to_string(),
            to: vec!["dave@example.com".to_string()],
        },
        ParsedEmail {
            from: "carol@example.com".to_string(),
            to: vec![
                "dave@example.com".to_string(),
                "eve@example.com".to_string(),
                "frank@example.com".to_string(),
            ],
        },
        ParsedEmail {
            from: "alice@example.com".to_string(),
            to: vec!["dave@example.com".to_string()],
        },
    ];

    // Build the graph
    let graph = Graph::build_from_emails(emails);

    // Calculate out-degrees
    let out_degrees = graph.calculate_out_degrees();
    // Expected out-degrees
    let mut expected_out_degrees = HashSet::new();
    expected_out_degrees.insert(("alice@example.com".to_string(), 3)); // bob and carol
    expected_out_degrees.insert(("bob@example.com".to_string(), 1));   // dave
    expected_out_degrees.insert(("carol@example.com".to_string(), 3)); // dave, eve, frank
    expected_out_degrees.insert(("dave@example.com".to_string(), 0));
    expected_out_degrees.insert(("eve@example.com".to_string(), 0));
    expected_out_degrees.insert(("frank@example.com".to_string(), 0));

    // Check out-degrees
    for (node, degree) in &out_degrees {
        let expected_degree = match node.as_str() {
            "alice@example.com" => 3,
            "bob@example.com" => 1,
            "carol@example.com" => 3,
            "dave@example.com" => 0,
            "eve@example.com" => 0,
            "frank@example.com" => 0,
            _ => panic!("Unexpected node in graph: {}", node),
        };
        assert_eq!(*degree, expected_degree, "Out-degree for {} should be {}", node, expected_degree);
    }

    // Also, check that all expected nodes are present
    assert_eq!(out_degrees.len(), 6, "Graph should have 6 nodes");
}

#[test]
fn test_in_degree_calculation() {
    // Create sample parsed emails
    let emails = vec![
        ParsedEmail {
            from: "alice@example.com".to_string(),
            to: vec![
                "bob@example.com".to_string(),
                "carol@example.com".to_string(),
            ],
        },
        ParsedEmail {
            from: "bob@example.com".to_string(),
            to: vec!["dave@example.com".to_string()],
        },
        ParsedEmail {
            from: "carol@example.com".to_string(),
            to: vec![
                "dave@example.com".to_string(),
                "eve@example.com".to_string(),
                "frank@example.com".to_string(),
            ],
        },
        ParsedEmail {
            from: "alice@example.com".to_string(),
            to: vec!["dave@example.com".to_string()],
        },
    ];

    // Build the graph
    let graph = Graph::build_from_emails(emails);

    // Calculate in-degrees
    let in_degrees = graph.calculate_in_degrees();

    // Expected in-degrees
    let mut expected_in_degrees = HashSet::new();
    expected_in_degrees.insert(("alice@example.com".to_string(), 0));
    expected_in_degrees.insert(("bob@example.com".to_string(), 1));   // From alice
    expected_in_degrees.insert(("carol@example.com".to_string(), 1)); // From alice
    expected_in_degrees.insert(("dave@example.com".to_string(), 3));  // From bob, carol, alice
    expected_in_degrees.insert(("eve@example.com".to_string(), 1));   // From carol
    expected_in_degrees.insert(("frank@example.com".to_string(), 1)); // From carol

    // Check in-degrees
    for (node, degree) in &in_degrees {
        let expected_degree = match node.as_str() {
            "alice@example.com" => 0,
            "bob@example.com" => 1,
            "carol@example.com" => 1,
            "dave@example.com" => 3,
            "eve@example.com" => 1,
            "frank@example.com" => 1,
            _ => panic!("Unexpected node in graph: {}", node),
        };
        assert_eq!(*degree, expected_degree, "In-degree for {} should be {}", node, expected_degree);
    }

    // Also, check that all expected nodes are present
    assert_eq!(in_degrees.len(), 6, "Graph should have 6 nodes");
}

#[test]
fn test_self_loops_and_multiple_edges() {
    // Create sample parsed emails
    let emails = vec![
        ParsedEmail {
            from: "alice@example.com".to_string(),
            to: vec!["bob@example.com".to_string()],
        },
        ParsedEmail {
            from: "alice@example.com".to_string(),
            to: vec!["bob@example.com".to_string()], // Duplicate recipient
        },
        ParsedEmail {
            from: "bob@example.com".to_string(),
            to: vec!["alice@example.com".to_string()], // Creates a cycle
        },
        ParsedEmail {
            from: "carol@example.com".to_string(),
            to: vec!["carol@example.com".to_string()], // Self-loop
        },
    ];

    // Build the graph
    let graph = Graph::build_from_emails(emails);

    // Calculate out-degrees
    let out_degrees = graph.calculate_out_degrees();

    // Expected out-degrees:
    // alice: 1 (bob)
    // bob: 1 (alice)
    // carol: 1 (carol)

    assert_eq!(out_degrees.get("alice@example.com"), Some(&1));
    assert_eq!(out_degrees.get("bob@example.com"), Some(&1));
    assert_eq!(out_degrees.get("carol@example.com"), Some(&1));

    // Remove the following line as "dave@example.com" is not part of this test
    // assert_eq!(out_degrees.get("dave@example.com"), Some(&0)); // Not present, hence should not be in the graph

    // Calculate in-degrees
    let in_degrees = graph.calculate_in_degrees();

    // Expected in-degrees:
    // alice: 1 (from bob)
    // bob: 1 (from alice)
    // carol: 1 (self-loop)

    assert_eq!(in_degrees.get("alice@example.com"), Some(&1));
    assert_eq!(in_degrees.get("bob@example.com"), Some(&1));
    assert_eq!(in_degrees.get("carol@example.com"), Some(&1));

    // Ensure only the expected nodes are present
    assert_eq!(out_degrees.len(), 3, "Graph should have 3 nodes");
    assert_eq!(in_degrees.len(), 3, "Graph should have 3 nodes");
}