use std::env;
use std::fs::File;
use std::io::{ErrorKind, Write};
use std::path::PathBuf;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn write_outputs(
    keys: &Vec<String>,
    output: &str,
    output_directory: &PathBuf,
    output_extension: &str,
) {
    let json: serde_json::Value = match serde_json::from_str(output) {
        Ok(json) => json,
        Err(e) => {
            eprintln!("Error parsing output: {}", e);
            std::process::exit(1);
        }
    };

    // Create the output directory if it doesn't exist
    if !output_directory.exists() {
        match std::fs::create_dir_all(output_directory) {
            Ok(_) => (),
            Err(e) => {
                eprintln!("Error creating output directory: {}", e);
                std::process::exit(1);
            }
        }
    }

    for key in keys {
        let value = match json.get(key) {
            Some(value) => value.as_str().unwrap(),
            None => {
                eprintln!("Invalid key \"{}\" not found in output {}", key, output);
                std::process::exit(1);
            }
        };
        let file_name = &format!("{}.{}", key, output_extension);
        let mut file = match File::create(output_directory.join(file_name)) {
            Ok(file) => file,
            Err(err) => {
                if err.kind() == ErrorKind::AlreadyExists {
                    eprintln!("File already exists: {}", file_name);
                    std::process::exit(1);
                } else {
                    eprintln!("Failed to create file {}: {}", file_name, err);
                    std::process::exit(1);
                }
            }
        };

        if let Err(err) = file.write_all(value.to_string().as_bytes()) {
            eprintln!("Failed to write to file {}: {}", file_name, err);
            std::process::exit(1);
        }
    }
}

fn get_args_as_vec(pattern: &str) -> Vec<String> {
    let re: regex::Regex = regex::Regex::new(pattern).unwrap();
    const PATTERN: &str = "=";

    env::args()
        .skip(1)
        .filter(|arg| re.is_match(arg))
        .map(|arg| {
            arg.split(PATTERN)
                .nth(1)
                .unwrap_or_else(|| {
                    eprintln!("Invalid option: {}", arg);
                    std::process::exit(1);
                })
                .to_string()
        })
        .collect()
}

fn get_args_as_bool(pattern: &str) -> bool {
    let re: regex::Regex = regex::Regex::new(pattern).unwrap();

    env::args().skip(1).filter(|arg| re.is_match(arg)).count() > 0
}

fn parse_keys() -> Result<Vec<String>, String> {
    let keys: Vec<String> = get_args_as_vec(r"^(--keys|-k)=");
    // Split the keys by commas or spaces or newlines
    let re: regex::Regex = regex::Regex::new(r"[,\s\\n]+").unwrap();
    let mut output: Vec<String> = Vec::new();

    println!("Keys: {:?}", keys);

    if keys.is_empty() {
        Err("No keys provided, Please specify at least one key using --key=[KEY_NAME] or -k=[KEY_NAME].".to_string())
    } else {
        for key in keys {
            if !key.is_empty() {
                output.extend(re.split(&key.replace("\n", "\\n")).filter_map(|s| {
                    if s.is_empty() {
                        None
                    } else {
                        Some(s.trim().to_string())
                    }
                }));

                println!("Output: {:?}", output);
            } else {
                Err("Invalid key provided, Please specify at least one key using --key=[KEY_NAME] or -k=[KEY_NAME].".to_string())?;
            }
        }
        Ok(output)
    }
}

fn parse_outputs() -> Result<String, String> {
    let outputs: Vec<String> = get_args_as_vec(r"^(--outputs|-o)=");

    let outputs_len: usize = outputs.len();

    if outputs_len > 1 {
        Err(format!(
            "Too many outputs provided, expected 1, got {}.",
            outputs_len
        ))
    } else if outputs.is_empty() {
        Err(
            "No outputs provided, Please specify an output using --outputs=[OUTPUT] or -o=[OUTPUT]."
                .to_string()
        )
    } else {
        Ok(outputs[0].clone())
    }
}

fn parse_directory() -> Result<String, String> {
    let directories: Vec<String> = get_args_as_vec(r"^(--directory|-d)=");

    let directory_len: usize = directories.len();

    if directory_len > 1 {
        Err(format!(
            "Too many directories provided, expected 1, got {}.",
            directory_len
        ))
    } else if directories.is_empty() {
        Err(
            "No directories provided, Please specify a directory using --directory=[DIRECTORY] or -d=[DIRECTORY]."
                .to_string(),
        )
    } else {
        Ok(directories[0].clone())
    }
}

fn parse_output_extension() -> Result<String, String> {
    let extensions: Vec<String> = get_args_as_vec(r"^(--extension|-e)=");

    let extension_len: usize = extensions.len();

    if extension_len > 1 {
        Err(format!(
            "Too many extensions provided, expected 1, got {}.",
            extension_len
        ))
    } else if extensions.is_empty() {
        Ok("txt".to_string())
    } else {
        Ok(extensions[0].clone())
    }
}

fn parse_help() -> bool {
    get_args_as_bool(r"^(--help|-h)$")
}

fn parse_version() -> bool {
    get_args_as_bool(r"^(--version|-v)$")
}

fn options_valid() -> bool {
    // Check if the options are valid i.e if the user had a typo --h instead of --help
    let valid_options: Vec<&str> = vec![
        "--keys",
        "-k",
        "--outputs",
        "-o",
        "--directory",
        "-d",
        "--extension",
        "-e",
        "--help",
        "-h",
        "--version",
        "-v",
    ];

    let invalid_options: Vec<String> = env::args()
        .skip(1)
        .filter(|arg| {
            for option in &valid_options {
                if arg.starts_with(option) {
                    return false;
                }
            }
            true
        })
        .collect();

    if !invalid_options.is_empty() {
        eprintln!("Invalid option(s): {:?}", invalid_options);
        eprintln!("Please use --help or -h to see the available options.");
        return false;
    }

    true
}

// TODO: Switch to use https://rust-cli.github.io/book/tutorial/index.html
// #[derive(Parser)]
// struct Options {
//     help: bool,
//     version: bool,
//     keys: Vec<String>,
//     outputs: String,
//     directory: String,
//     extension: String,
// }

fn main() {
    if !options_valid() {
        std::process::exit(1);
    }

    let version: bool = parse_version();

    if version {
        println!("json2file {}", VERSION);
        std::process::exit(0);
    }

    let help: bool = parse_help();

    if help {
        let example_outputs: &str = r#"{\"foo\": \"value1\", \"bar\": \"value2\"}"#;
        println!("json2file");
        println!("Generate files from a JSON output.\n");
        println!("Usage:\n\tjson2file --keys=[keys] --outputs=[OUTPUT] --directory=[directory] --extension=[EXTENSION]\n");
        println!("Options:\n");
        println!("-h, --help\t\tShow this help message and exit.");
        println!("-v, --VERSION\t\tShow the VERSION and exit.");
        println!("-k, --keys\t\tThe keys to use to generate the files. (Required)");
        println!("-o, --outputs\t\tThe JSON output to use. (Required)");
        println!("-d, --directory\t\tThe directory to output the files to. (Required)");
        println!(
            "-e, --extension\t\tThe extension to use for the files. (Optional, defaults to txt)"
        );
        println!("\nExample:\n\tjson2file --keys=foo,bar --outputs=\"{}\" --directory=/tmp --extension=txt", example_outputs);
        std::process::exit(0);
    }

    let keys: Vec<String> = parse_keys().unwrap_or_else(|err| {
        eprintln!("{}", err);
        std::process::exit(1);
    });

    let outputs: String = parse_outputs().unwrap_or_else(|err| {
        eprintln!("{}", err);
        std::process::exit(1);
    });

    let current_directory: PathBuf = env::current_dir().unwrap_or_else(|err| {
        eprintln!("Failed to get current directory: {}", err);
        std::process::exit(1);
    });

    let directory: String = parse_directory().unwrap_or_else(|err| {
        eprintln!("{}", err);
        std::process::exit(1);
    });

    let output_extension: String = parse_output_extension().unwrap_or_else(|err| {
        eprintln!("{}", err);
        std::process::exit(1);
    });

    let output_directory: PathBuf = current_directory.join(directory);

    write_outputs(&keys, &outputs, &output_directory, &output_extension);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Read;

    #[test]
    fn test_write_outputs() {
        // Create some test input for the write_outputs() function.
        let keys: Vec<String> = vec!["key1".to_string(), "key2".to_string()];
        let output: &str = r#"{
            "key1": "value1",
            "key2": "value2",
            "key3": "value3"
        }"#;

        let output_directory: PathBuf = PathBuf::from("test");
        let output_extension: &str = "txt";

        write_outputs(&keys, &output, &output_directory, &output_extension);

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
