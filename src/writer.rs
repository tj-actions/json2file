use std::fmt;
use std::fs::File;
use std::io::{ErrorKind, Write};
use std::path::PathBuf;

use clap::ValueEnum;
use csv::{StringRecord, WriterBuilder};
use unescaper::unescape;

#[derive(ValueEnum, Clone, Debug, PartialEq)]
pub enum Extension {
    Txt,
    Json,
    Csv,
}

impl fmt::Display for Extension {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Extension::Txt => write!(f, "txt"),
            Extension::Json => write!(f, "json"),
            Extension::Csv => write!(f, "csv"),
        }
    }
}

pub fn create_output_directory(path: &PathBuf) {
    if !path.try_exists().unwrap_or(false) {
        println!("Creating output directory...");
        match std::fs::create_dir_all(path) {
            Ok(_) => println!("Output directory created successfully."),
            Err(e) => {
                eprintln!(
                    "Error creating output directory '{}': {}",
                    path.display(),
                    e
                );
                std::process::exit(1);
            }
        }
    } else {
        println!("Output directory already exists.");
    }
}

pub fn write_outputs(
    skip_missing_keys: &bool,
    keys: &Vec<String>,
    output: &str,
    output_directory: &PathBuf,
    output_extension: &Extension,
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
            eprintln!("Error parsing output: {e}");
            std::process::exit(1);
        }
    };

    create_output_directory(output_directory);

    for key in keys {
        if *verbose {
            println!("Writing output for key '{key}'...");
        }
        let value = match json.get(key) {
            Some(value) => {
                if *output_extension == Extension::Json {
                    let bytes = value.to_string().into_bytes();
                    let json: serde_json::Value = serde_json::from_slice(&bytes).unwrap();

                    if json.is_string() {
                        let string = json.as_str().unwrap();
                        let unescaped = unescape(string).unwrap();

                        match serde_json::from_slice(&unescaped.into_bytes()) {
                            Ok(json) => json,
                            Err(_) => json,
                        }
                    } else {
                        json
                    }
                } else if *output_extension == Extension::Csv {
                    // Return a CSV representation with comma separated
                    let bytes = value.to_string().into_bytes();
                    let json: serde_json::Value = serde_json::from_slice(&bytes).unwrap();

                    if json.is_string() {
                        let string = json.as_str().unwrap();
                        let unescaped = unescape(string).unwrap();

                        serde_json::from_slice::<serde_json::Value>(&unescaped.into_bytes())
                            .map(|json| {
                                serde_json::Value::String(
                                    serde_json::to_string(json.as_array().unwrap()).unwrap(),
                                )
                            })
                            .unwrap_or(json)
                    } else {
                        json
                    }
                } else {
                    value.clone()
                }
            }
            None => {
                if *skip_missing_keys {
                    if *verbose {
                        println!("Key '{key}' not found, skipping...");
                    }
                    continue;
                } else {
                    eprintln!("Invalid key \"{key}\" not found in output {output}");
                    std::process::exit(1);
                }
            }
        };
        let file_path = &output_directory.join(format!("{key}.{output_extension}"));
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

        if output_extension == &Extension::Json {
            serde_json::to_writer(&mut file, &value).unwrap();
        } else if output_extension == &Extension::Csv {
            let mut wtr = WriterBuilder::new().from_writer(file);
            let mut record = StringRecord::new();
            if let Some(items) = value.as_array() {
                items
                    .iter()
                    .for_each(|item| record.push_field(item.as_str().unwrap()));
            } else {
                record.push_field(value.as_str().unwrap());
            }
            wtr.write_record(&record).unwrap();
        } else if let Err(err) = file.write_all(value.as_str().unwrap().as_bytes()) {
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
