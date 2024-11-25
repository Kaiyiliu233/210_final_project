pub mod email;

use std::fs::File;
use email::{EmailRecord, ParsedEmail};
use std::error::Error;
use std::collections::{HashSet, HashMap};
use csv::ReaderBuilder;

fn parse_recipients(recipient: &str) -> Vec<String> {
    recipient
        .split(',')
        .map(|s| s.trim().to_lowercase())
        .filter(|s| !s.is_empty())
        .collect()
}

/// Reads and parses the email data from a CSV file.
/// Returns a vector of `ParsedEmail` instances.
fn read_csv(file_path: &str) -> Result<Vec<ParsedEmail>, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(file);

    let mut parsed_emails = Vec::new();
    let mut failed_parses = 0;

    for result in rdr.deserialize() {
        let record: EmailRecord = match result {
            Ok(rec) => rec,
            Err(e) => {
                eprintln!("Failed to deserialize a record: {}", e);
                failed_parses += 1;
                continue;
            }
        };

        // Parse recipients
        let recipients = parse_recipients(&record.recipient1);

        // Check for missing sender or recipients
        if record.sender.is_empty() || recipients.is_empty() {
            eprintln!(
                "Incomplete record found at index {}: sender='{}', recipient1='{}'",
                record.index, record.sender, record.recipient1
            );
            failed_parses += 1;
            continue;
        }

        // Create ParsedEmail instance
        let parsed_email = ParsedEmail {
            from: record.sender.clone(),
            to: recipients,
        };

        parsed_emails.push(parsed_email);
    }

    println!(
        "Successfully parsed {} emails.",
        parsed_emails.len()
    );
    if failed_parses > 0 {
        println!("Failed to parse {} records.", failed_parses);
    }

    Ok(parsed_emails)
}

fn main() {
    // Path to your CSV file
    let file_path = "emaildata_100000_0.csv";

    // Step 1: Read and parse the CSV
    let parsed_emails = read_csv(file_path).unwrap();

    // Step 2: (Placeholder) Further processing can be done here
    // For demonstration, we'll print the first 5 parsed emails
    println!("\nFirst 5 Parsed Emails:");
    for email in parsed_emails.iter().take(5) {
        println!("{:?}", email);
    }
}