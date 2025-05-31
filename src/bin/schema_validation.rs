use serde_json::json;

fn main() {
    let schema: serde_json::Value = serde_json::from_str(include_str!("../cv-schema.json")).unwrap();
    println!("{:#?}", schema);
    let validation_error = jsonschema::meta::validate(&schema);
    eprintln!("{:#?}", validation_error.err());
}