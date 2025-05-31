use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::{fs, io};
use thiserror::Error;

#[derive(Serialize, Deserialize, Debug)]
pub struct InputCVData {
    cv: InputCV,
    preferences: InputPreferences,
}

#[derive(Serialize, Deserialize, Debug)]
struct InputCV {
    name: String,
    mugshot: PathBuf,
    address: InputAddress,
}

#[derive(Serialize, Deserialize, Debug)]
struct InputPreferences {
    template: String,
    style: Option<String>,
    output_format: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct InputAddress {
    street: String,
    city: String,
    #[serde(rename = "zip code")]
    zip_code: String,
    state: String,
    country: String,
}

#[derive(Error, Debug)]
pub enum InputYamlCVError {
    #[error("The input CV YAML file is too large (> 1MB)")]
    FileTooLarge,

    #[error("An IO error occurred while loading YAML file")]
    Io(#[from] io::Error),
}

pub fn read_yaml_input_cv_into_string(path: &Path) -> Result<String, InputYamlCVError> {
    let file_size = fs::metadata(path)?.len();
    if file_size > 1_000_000 {
        return Err(InputYamlCVError::FileTooLarge);
    }

    Ok(fs::read_to_string(path)?)
}

#[derive(Error, Debug)]
pub enum YamlCVJsonSchemaValidationError {
    #[error(transparent)]
    JsonSyntaxError(#[from] serde_yaml::Error),

    #[error("Error validating the input CV YAML with the JSON schema:\n{0}")]
    YamlCVValidationErrors(String),
}

pub fn validate_yaml_cv_against_json_schema(
    yaml_cv_string: String,
) -> Result<(), YamlCVJsonSchemaValidationError> {
    let json_value: serde_json::Value = serde_yaml::from_str(&yaml_cv_string)?;
    let schema = serde_json::from_str(include_str!("cv-schema.json"))
        .expect("Failed to parse embedded JSON schema");
    let validator =
        jsonschema::validator_for(&schema).expect("Failed to create JSON schema validator");

    let validation_errors: Vec<String> = validator
        .iter_errors(&json_value)
        .map(|e| format!("=> {}: {}", e.instance_path, e))
        .collect();

    if !validation_errors.is_empty() {
        return Err(YamlCVJsonSchemaValidationError::YamlCVValidationErrors(
            validation_errors.join("\n"),
        ));
    }
    Ok(())
}
