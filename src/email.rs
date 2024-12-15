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

/// Parses the recipient string into a vector of individual email addresses
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

    let mut parsed_emails = Vec::new(); // Vector to store successfully parsed emails
    let mut failed_parses = 0; // Counter for the number of failed parse attempts

    // Iterate over each deserialized record in the CSV
    for result in rdr.deserialize() {
        // Attempt to deserialize the current record into an EmailRecord struct
        let record: EmailRecord = match result {
            Ok(rec) => rec, // Successfully deserialized record
            Err(e) => {
                // Log the error and increment the failed parse counter
                eprintln!("Failed to deserialize a record: {}", e);
                failed_parses += 1;
                continue; // Skip to the next record
            }
        };

        // Parse the recipients string into a vector of email addresses
        let recipients = parse_recipients(&record.recipient1);

        // Check for missing sender or recipients to ensure data completeness
        if record.sender.is_empty() || recipients.is_empty() {
            // Log the incomplete record details and increment the failed parse counter
            eprintln!(
                "Incomplete record found at index {}: sender='{}', recipient1='{}'",
                record.index, record.sender, record.recipient1
            );
            failed_parses += 1;
            continue; // Skip to the next record
        }

        // Create a ParsedEmail instance with the sender and parsed recipients
        let parsed_email = ParsedEmail {
            from: record.sender.clone(), // Clone the sender's email address
            to: recipients, // Assign the vector of recipient email addresses
        };

        parsed_emails.push(parsed_email); // Add the ParsedEmail to the collection
    }
    
    // Print the number of successfully parsed emails
    println!(
        "Successfully parsed {} emails.",
        parsed_emails.len()
    );

    // If there were any failed parses, log the total count
    if failed_parses > 0 {
        println!("Failed to parse {} records.", failed_parses);
    }
    
    // Return the vector of ParsedEmail instances
    Ok(parsed_emails)
}