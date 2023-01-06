use std::fs::File;
use std::io::{ErrorKind, Write};
use std::path::PathBuf;

pub fn write_outputs(
    skip_missing_keys: &bool,
    keys: &Vec<String>,
    output: &str,
    output_directory: &PathBuf,
    output_extension: &str,
    verbose: &bool,
) {
    if *verbose {
        println!("Parsing outputs...");
    }
    let json: serde_json::Value = match serde_json::from_str(output) {
        Ok(json) => {
            if *verbose {
                println!("Outputs parsed successfully.");
            }
            json
        }
        Err(e) => {
            eprintln!("Error parsing output: {}", e);
            std::process::exit(1);
        }
    };

    // Create the output directory if it doesn't exist
    if !output_directory.exists() {
        if *verbose {
            println!("Creating output directory...");
        }

        match std::fs::create_dir_all(output_directory) {
            Ok(_) => {
                if *verbose {
                    println!("Output directory created successfully.");
                }
            }
            Err(e) => {
                eprintln!(
                    "Error creating output directory '{}': {}",
                    output_directory.display(),
                    e
                );
                std::process::exit(1);
            }
        }
    } else if *verbose {
        println!("Output directory already exists.");
    }

    for key in keys {
        if *verbose {
            println!("Writing output for key '{}'...", key);
        }
        let value = match json.get(key) {
            Some(value) => value.as_str().unwrap(),
            None => {
                if *skip_missing_keys {
                    continue;
                } else {
                    eprintln!("Invalid key \"{}\" not found in output {}", key, output);
                    std::process::exit(1);
                }
            }
        };
        let file_path = &output_directory.join(format!("{}.{}", key, output_extension));
        if *verbose {
            println!("Writing output to file '{}'...", file_path.display());
        }
        let mut file = match File::create(file_path) {
            Ok(file) => file,
            Err(err) => {
                if err.kind() == ErrorKind::AlreadyExists {
                    eprintln!("File '{}' already exists.", file_path.display());
                    std::process::exit(1);
                } else {
                    eprintln!("Error creating file '{}': {}", file_path.display(), err);
                    std::process::exit(1);
                }
            }
        };

        if let Err(err) = file.write_all(value.to_string().as_bytes()) {
            eprintln!("Error writing to file '{}': {}", file_path.display(), err);
            std::process::exit(1);
        }

        if *verbose {
            println!(
                "Wrote output to file '{}' for key '{}'.",
                file_path.display(),
                key
            );
        }
    }

    if *verbose {
        println!("Finished writing outputs.");
    }
}
