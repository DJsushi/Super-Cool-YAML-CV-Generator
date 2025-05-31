use std::process::exit;

fn main() {
    let schema: serde_json::Value =
        serde_json::from_str(include_str!("../cv-schema.json")).unwrap();
    // println!("{:#?}", schema);
    let validation_error = jsonschema::meta::validate(&schema);
    if let Some(error) = validation_error.err() {
        eprintln!("{:#?}", error);
        exit(1);
    }
}
