use clap::Parser;
use serde::Deserialize;
use std::fs;
use std::path::PathBuf;
use std::process::exit;

#[derive(Parser, Debug)]
struct Cli {
    #[arg(long, value_name = "CV")]
    cv: PathBuf,
}

#[derive(Deserialize, Debug)]
struct CVYaml {
    cv: CV,
}

#[derive(Deserialize, Debug)]
struct CV {
    name: String,
    mugshot: PathBuf,
    address: Address,
}

#[derive(Deserialize, Debug)]
struct Address {
    street: String,
    city: String,
    #[serde(rename = "zip code")]
    zip_code: String,
    state: String,
    country: String,
}

fn main() {
    let cli = Cli::parse();
    println!("{:?}", cli);

    if !cli.cv.exists() {
        println!("CV file not found");
        exit(1);
    }

    if !cli.cv.is_file() {
        println!("CV file is not a file");
        exit(1);
    }

    let yaml_content = fs::read_to_string(cli.cv).unwrap();

    // Parse YAML directly into JSON Value
    let json_value: serde_json::Value = serde_yaml::from_str(&yaml_content).unwrap();

    let schema: serde_json::Value = serde_json::from_str(include_str!("cv-schema.json")).unwrap();
    jsonschema::validate(&schema, &json_value).unwrap();
    
    println!("{:?}", json_value);
}
