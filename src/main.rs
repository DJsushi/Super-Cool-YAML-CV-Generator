mod cli;
mod input;

use crate::cli::parse_cli;
use crate::input::{read_yaml_input_cv_into_string, validate_yaml_cv_against_json_schema};
use anyhow::Context;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::{fs, io};

fn copy_dir_recursive(src: &PathBuf, dest: &PathBuf) -> io::Result<()> {
    if !dest.exists() {
        fs::create_dir_all(dest)?;
    }

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let path = entry.path();
        let dest_path = dest.join(entry.file_name());

        if path.is_dir() {
            copy_dir_recursive(&path, &dest_path)?;
        } else {
            // Skip the handlebars template file as we'll render it separately
            if path.extension().and_then(|ext| ext.to_str()) != Some("hbs") {
                fs::copy(&path, &dest_path)?;
            }
        }
    }
    Ok(())
}

fn copy_file_to_dir(src_file: &Path, target_dir: &Path) -> io::Result<PathBuf> {
    println!("Copying {} to {}", src_file.display(), target_dir.display());
    if !target_dir.is_dir() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Target is not a directory",
        ));
    }

    let file_name = src_file
        .file_name()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Source has no filename"))?;

    let target_path = target_dir.join(file_name);
    fs::copy(src_file, &target_path)?;
    Ok(target_path)
}

fn open_in_browser(file_path: &Path) -> std::io::Result<()> {
    #[cfg(target_os = "macos")]
    {
        Command::new("open").arg(file_path).spawn()?;
    }

    #[cfg(target_os = "linux")]
    {
        println!("Running xdg-open with {}", file_path.display());
        let mut cmd = Command::new("sh")
            .args([
                "-c",
                &format!("nohup xdg-open {} >/dev/null 2>&1 &", file_path.display()),
            ])
            .spawn()?;
        cmd.wait()?;
    }

    Ok(())
}

fn main() -> anyhow::Result<()> {
    let cli = parse_cli().context("Failed to parse or verify command line arguments")?;

    // 1. Parse the YAML CV file into a string
    // 2. Convert the string into a JSON and check it against the JSON schema
    // 3. If it's all good, try to parse it with our custom, more advanced, validation logic into
    //    the InputCVData

    let yaml_cv_string = read_yaml_input_cv_into_string(&cli.cv_path)
        .context("An error occurred while trying to load the YAML CV file")?;

    validate_yaml_cv_against_json_schema(yaml_cv_string).context("Failed to validate YAML CV")?;
    
    // let cv_data: CVYaml = serde_yaml::from_str(&yaml_content).unwrap_or_else(|e| {
    //     println!("Failed to deserialize CV data: {}", e);
    //     exit(1);
    // });
    //
    // // Create temporary directory
    // let temp_dir = std::env::temp_dir().join(format!("cv_render_{}", std::process::id()));
    //
    // if let Err(e) = fs::create_dir_all(&temp_dir) {
    //     println!("Failed to create temporary directory: {}", e);
    //     exit(1);
    // }
    //
    // println!("Created temporary directory: {:?}", temp_dir);
    //
    // // Copy all template files (except .hbs files) to temp directory
    // if let Err(e) = copy_dir_recursive(&cli.template, &temp_dir) {
    //     println!("Failed to copy template files: {}", e);
    //     exit(1);
    // }
    //
    // // Setup Handlebars and render
    // let mut handlebars = Handlebars::new();
    // handlebars.set_strict_mode(true);
    //
    // let template_file = cli.template.join("cv.hbs");
    // if !template_file.exists() {
    //     println!("Template file 'cv.hbs' not found in template directory");
    //     exit(1);
    // }
    //
    // let cv_dir = cli.cv.parent().unwrap();
    //
    // handlebars
    //     .register_template_file("cv", &template_file)
    //     .unwrap_or_else(|e| {
    //         println!("Failed to register template: {}", e);
    //         exit(1);
    //     });
    //
    // let rendered = handlebars.render("cv", &cv_data).unwrap_or_else(|e| {
    //     println!("Failed to render template: {}", e);
    //     exit(1);
    // });
    //
    // // Write rendered HTML to temp directory
    // let output_file = temp_dir.join("index.html");
    // if let Err(e) = fs::write(&output_file, rendered) {
    //     println!("Failed to write rendered HTML: {}", e);
    //     exit(1);
    // }
    //
    // copy_file_to_dir(&cv_dir.join(&cv_data.cv.mugshot), &temp_dir).unwrap();
    //
    // println!("Rendered CV saved to: {:?}", output_file);
    //
    // // Open in default browser
    // if let Err(e) = open_in_browser(&output_file) {
    //     println!("Failed to open browser: {}", e);
    //     println!("You can manually open: {}", output_file.display());
    // } else {
    //     println!("Opening CV in default browser...");
    // }
    //
    // println!("Temporary files will remain at: {:?}", temp_dir);
    // println!("You may want to clean them up manually later.");

    Ok(())
}
