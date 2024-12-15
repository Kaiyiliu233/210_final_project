pub mod email;
pub mod graph;


use email::{ParsedEmail, read_csv};
use graph::{Graph};
use std::error::Error;
use std::collections::{HashMap, HashSet};

fn analyze_degree_distribution(graph: &Graph) {
    // Calculate out-degrees and in-degrees
    let out_degrees = graph.calculate_out_degrees();
    let in_degrees = graph.calculate_in_degrees();

    // Calculate statistics for out-degrees
    let total_nodes = out_degrees.len();
    let total_out_degree: usize = out_degrees.values().sum();
    let average_out_degree = total_out_degree as f64 / total_nodes as f64;
    let max_out_degree = out_degrees.values().cloned().max().unwrap_or(0);
    let min_out_degree = out_degrees.values().cloned().min().unwrap_or(0);

    // Calculate statistics for in-degrees
    let total_in_degree: usize = in_degrees.values().sum();
    let average_in_degree = total_in_degree as f64 / total_nodes as f64;
    let max_in_degree = in_degrees.values().cloned().max().unwrap_or(0);
    let min_in_degree = in_degrees.values().cloned().min().unwrap_or(0);

    // Display Out-Degree Statistics
    println!("--- Out-Degree Statistics ---");
    println!("Total Nodes: {}", total_nodes);
    println!("Total Out-Degree: {}", total_out_degree);
    println!("Average Out-Degree: {:.2}", average_out_degree);
    println!("Maximum Out-Degree: {}", max_out_degree);
    println!("Minimum Out-Degree: {}", min_out_degree);

    // Display In-Degree Statistics
    println!("\n--- In-Degree Statistics ---");
    println!("Total Nodes: {}", total_nodes);
    println!("Total In-Degree: {}", total_in_degree);
    println!("Average In-Degree: {:.2}", average_in_degree);
    println!("Maximum In-Degree: {}", max_in_degree);
    println!("Minimum In-Degree: {}", min_in_degree);
}

/// Identifies the top N senders based on out-degree.
pub fn identify_top_senders(out_degrees: &HashMap<String, usize>, top_n: usize) -> Vec<(String, usize)> {
    let mut senders: Vec<(String, usize)> = out_degrees.iter()
        .map(|(node, degree)| (node.clone(), *degree))
        .collect();
    
    // Sort senders by out-degree in descending order
    senders.sort_by(|a, b| b.1.cmp(&a.1));
    
    // Return the top N senders
    senders.into_iter().take(top_n).collect()
}

/// Identifies the top N recipients based on in-degree.
pub fn identify_top_recipients(in_degrees: &HashMap<String, usize>, top_n: usize) -> Vec<(String, usize)> {
    let mut recipients: Vec<(String, usize)> = in_degrees.iter()
        .map(|(node, degree)| (node.clone(), *degree))
        .collect();
    
    // Sort recipients by in-degree in descending order
    recipients.sort_by(|a, b| b.1.cmp(&a.1));
    
    // Return the top N recipients
    recipients.into_iter().take(top_n).collect()
}

/// Identifies key statistics about the community
fn analyze_communities(community_map: &HashMap<String, Vec<String>>) {
    let total_communities = community_map.len();
    println!("Total Detected Communities: {}", total_communities);

    // Identify the size of each community
    let mut community_sizes: Vec<usize> = community_map.values().map(|members| members.len()).collect();
    community_sizes.sort_unstable_by(|a, b| b.cmp(a)); // Sort in descending order

    // Calculate statistics
    let total_nodes = community_sizes.iter().sum::<usize>();
    let average_size = total_nodes as f64 / total_communities as f64;
    let largest_size = community_sizes.first().cloned().unwrap_or(0);
    let smallest_size = community_sizes.last().cloned().unwrap_or(0);

    println!("Total Nodes: {}", total_nodes);
    println!("Average Community Size: {:.2}", average_size);
    println!("Largest Community Size: {}", largest_size);
    println!("Smallest Community Size: {}", smallest_size);
}

/// Prints the top N senders and recipients.
pub fn print_top_individuals(out_degrees: &HashMap<String, usize>, in_degrees: &HashMap<String, usize>, top_n: usize) {
    let top_senders = identify_top_senders(out_degrees, top_n);
    let top_recipients = identify_top_recipients(in_degrees, top_n);
    
    println!("\n--- Top {} Senders (Prolific Communicators) ---", top_n);
    for (i, (sender, degree)) in top_senders.iter().enumerate() {
        println!("{}. {} - Sent {} emails", i + 1, sender, degree);
    }
    
    println!("\n--- Top {} Recipients (Information Hubs) ---", top_n);
    for (i, (recipient, degree)) in top_recipients.iter().enumerate() {
        println!("{}. {} - Received {} emails", i + 1, recipient, degree);
    }
}

/// Identify and print the smallest and the largest community
fn identify_extreme_communities(community_map: &HashMap<String, Vec<String>>) {
    // Find the largest community
    if let Some((largest_label, largest_members)) = community_map.iter().max_by_key(|&(_, members)| members.len()) {
        println!("\n--- Largest Community ---");
        println!("Community Label: {}", largest_label);
        println!("Number of Members: {}", largest_members.len());
        println!("Top 10 Members: {:?}", largest_members.iter().take(10).collect::<Vec<&String>>());
    }

    // Find the smallest community
    if let Some((smallest_label, smallest_members)) = community_map.iter().min_by_key(|&(_, members)| members.len()) {
        println!("\n--- Smallest Community ---");
        println!("Community Label: {}", smallest_label);
        println!("Number of Members: {}", smallest_members.len());
        println!("Member: {:?}", smallest_members);
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // Path to your CSV file
    let file_path = "emaildata_100000_0.csv";

    // Read and parse the CSV
    let parsed_emails = read_csv(file_path)?;

    // Build the graph
    let graph = Graph::build_from_emails(parsed_emails);

    // Perform Degree Distribution Analysis
    analyze_degree_distribution(&graph);

    // Calculate out-degrees and in-degrees
     let out_degrees = graph.calculate_out_degrees();
     let in_degrees = graph.calculate_in_degrees();
        
     // Identify and print top N senders and recipients
     let top_n = 10; // Define how many top individuals to identify
     print_top_individuals(&out_degrees, &in_degrees, top_n);
        
    // Perform Label Propagation
    let communities = graph.label_propagation();

    // Organize nodes by communities
    let mut community_map: HashMap<String, Vec<String>> = HashMap::new();
    for (node, label) in communities {
        community_map.entry(label).or_insert(Vec::new()).push(node);
    }
    // Analyze Communities
    analyze_communities(&community_map);

    // Identify Extreme Communities
    identify_extreme_communities(&community_map);

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

#[test]
fn test_label_propagation_small_graph() {
    let mut graph = Graph::new();

    // Community 1: A, B, C
    graph.add_edge("A".to_string(), "B".to_string());
    graph.add_edge("A".to_string(), "C".to_string());
    graph.add_edge("B".to_string(), "C".to_string());

    // Community 2: D, E, F
    graph.add_edge("D".to_string(), "E".to_string());
    graph.add_edge("D".to_string(), "F".to_string());
    graph.add_edge("E".to_string(), "F".to_string());
    

    // Perform Label Propagation
    let labels = graph.label_propagation();

    // Organize nodes by communities
    let mut community_map: HashMap<String, Vec<String>> = HashMap::new();
    for (node, label) in labels {
        community_map.entry(label).or_insert(Vec::new()).push(node);
    }

    // Expect two communities
    assert_eq!(community_map.len(), 2, "There should be 2 communities");

    // Check sizes
    for members in community_map.values() {
        assert_eq!(members.len(), 3, "Each community should have 3 members");
    }
}