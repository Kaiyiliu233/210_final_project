use std::collections::{HashSet, HashMap};

pub fn build_graph(parsed_emails: Vec<ParsedEmail>) -> HashMap<String, HashSet<String>> {
    let mut graph: HashMap<String, HashSet<String>> = HashMap::new();

    for email in parsed_emails {
        let sender = email.from;
        let recipients = email.to;

        for recipient in recipients {
            // Add the recipient to the sender's adjacency list
            graph
                .entry(sender.clone())
                .or_insert_with(HashSet::new)
                .insert(recipient.clone());

            // Ensure the recipient is in the graph even if they have no outgoing edges
            graph.entry(recipient).or_insert_with(HashSet::new);
        }
    }

    graph
}

