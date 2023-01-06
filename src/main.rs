use clap::Parser;
use std::env;
use std::fs::File;
use std::io::{ErrorKind, Write};
use std::path::PathBuf;

fn write_outputs(
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
        Ok(json) => json,
        Err(e) => {
            eprintln!("Error parsing output: {}", e);
            std::process::exit(1);
        }
    };
    if *verbose {
        println!("Parsed outputs.");
    }

    // Create the output directory if it doesn't exist
    if !output_directory.exists() {
        if *verbose {
            println!("Creating output directory...");
        }

        match std::fs::create_dir_all(output_directory) {
            Ok(_) => (),
            Err(e) => {
                eprintln!(
                    "Error creating output directory '{}': {}",
                    output_directory.display(),
                    e
                );
                std::process::exit(1);
            }
        }
    } else {
        if *verbose {
            println!("Output directory already exists.");
        }
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
        let file_path = &output_directory.join(&format!("{}.{}", key, output_extension));
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

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Space delimited list of keys to extract from the JSON output converted into Vec<String>.
    #[clap(short, long, required = true)]
    keys: String,

    /// The JSON output to use.
    #[arg(short, long, required = true)]
    outputs: String,

    /// The directory to output the files to.
    #[arg(short, long, required = true)]
    directory: String,

    /// Skip missing keys.
    #[arg(short, long)]
    skip_missing_keys: bool,

    /// The extension to use for the files.
    #[arg(short, long, default_value = "txt")]
    extension: String,

    #[arg(short, long)]
    verbose: bool,
}

fn main() {
    let args = Args::parse();

    let keys: Vec<String> = args
        .keys
        .split_whitespace()
        .map(|s| s.to_string())
        .collect();

    let current_directory: PathBuf = env::current_dir().unwrap_or_else(|err| {
        eprintln!("Failed to get current directory: {}", err);
        std::process::exit(1);
    });

    let output_directory: PathBuf = current_directory.join(args.directory);

    if args.verbose {
        println!("Writing outputs to {}", output_directory.display());
    }

    write_outputs(
        &args.skip_missing_keys,
        &keys,
        &args.outputs,
        &output_directory,
        &args.extension,
        &args.verbose,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Read;

    #[test]
    fn test_args() {
        let args = Args::parse_from(&[
            "",
            "--keys",
            "key1 key2",
            "--outputs",
            "output",
            "--directory",
            "directory",
            "--extension",
            "ext",
        ]);
        assert_eq!(args.keys, "key1 key2");
        assert_eq!(args.outputs, "output");
        assert_eq!(args.directory, "directory");
        assert_eq!(args.extension, "ext");
        assert_eq!(args.skip_missing_keys, false);
        assert_eq!(args.verbose, false);
    }

    #[test]
    fn test_main() {
        let args = Args::parse_from(&[
            "",
            "--keys",
            "key1 key2",
            "--outputs",
            "{ \"key1\": \"value1\", \"key2\": \"value2\", \"key3\": \"value3\" }",
            "--directory",
            "test",
            "--extension",
            "txt",
            "--skip-missing-keys",
        ]);
        let keys: Vec<String> = args
            .keys
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();

        let current_directory: PathBuf = env::current_dir().unwrap_or_else(|err| {
            eprintln!("Failed to get current directory: {}", err);
            std::process::exit(1);
        });

        let output_directory: PathBuf = current_directory.join(args.directory);

        write_outputs(
            &args.skip_missing_keys,
            &keys,
            &args.outputs,
            &output_directory,
            &args.extension,
            &args.verbose,
        );

        // Check that the files were created.
        assert!(output_directory.join("key1.txt").exists());
        assert!(output_directory.join("key2.txt").exists());

        // Check that the files contain the correct values.
        let mut file = File::open(output_directory.join("key1.txt")).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        assert_eq!(contents, "value1");

        let mut file = File::open(output_directory.join("key2.txt")).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        assert_eq!(contents, "value2");

        // Clean up the files and the test directory.
        std::fs::remove_file(output_directory.join("key1.txt")).unwrap();
        std::fs::remove_file(output_directory.join("key2.txt")).unwrap();
        std::fs::remove_dir(output_directory).unwrap();
    }
}
