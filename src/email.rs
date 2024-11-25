use serde::Deserialize;
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