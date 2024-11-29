use serde::Deserialize;
use std::fs::File;
use std::error::Error;
use csv::ReaderBuilder;

/// Struct to represent each email record in the CSV
#[derive(Debug, Deserialize)]
pub struct EmailRecord {
    // The first column is an unnamed index, which we'll map to 'index'
    #[serde(rename = "")]
    pub index: usize,
    pub date: String,
    pub sender: String,
    pub recipient1: String,
    pub subject: String,
    pub text: String,
}

/// Struct to represent the parsed email with only 'from' and 'to' addresses
#[derive(Debug)]
pub struct ParsedEmail {
    pub from: String,
    pub to: Vec<String>,
}

pub fn parse_recipients(recipient: &str) -> Vec<String> {
    recipient
        .split(',')
        .map(|s| s.trim().to_lowercase())
        .filter(|s| !s.is_empty())
        .collect()
}

/// Reads and parses the email data from a CSV file.
/// Returns a vector of `ParsedEmail` instances.
pub fn read_csv(file_path: &str) -> Result<Vec<ParsedEmail>, Box<dyn Error>> {
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
        // For debugging: print first few parsed emails
        if parsed_emails.len() < 5 {
            println!("{:?}", parsed_email);
        }

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