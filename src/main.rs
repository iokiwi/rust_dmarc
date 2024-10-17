use clap::Parser;
use serde::Deserialize;
// use std::io::Read;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    path: String,
}

#[derive(Debug, Deserialize)]
struct DMARCReport {
    record: Vec<Record>,
}

#[derive(Debug, Deserialize)]
struct Record {
    row: Row,
    // identifiers: Identifiers,
    // auth_results: AuthResults,
}

#[derive(Debug, Deserialize)]
struct Row {
    source_ip: String,  
    count: u32,
    policy_evaluated: PolicyEvaluated,
}

#[derive(Debug, Deserialize)]
struct PolicyEvaluated {
    disposition: String,
    dkim: String,
    spf: String,
}


fn main() -> Result<(), Box<dyn std::error::Error>> {

    let args = Args::parse();
    let file_path = args.path;
    let file = std::fs::File::open(file_path)?;
    let reader = std::io::BufReader::new(file);

    // Deserialize the XML to the DMARCReport struct
    let report: DMARCReport = serde_xml_rs::from_reader(reader)?;

    // Debug print the report to see if it's loaded correctly
    println!("{:#?}", report);

    Ok(())
}