use std::env;
use std::fs::File;
use std::io::{ErrorKind, Write};
use std::path::PathBuf;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn write_outputs(
    keys: &Vec<String>,
    output: &str,
    output_directory: &PathBuf,
    output_extension: &str,
) {
    const JSON: serde_json::Value = match serde_json::from_str(output) {
        Ok(JSON) => JSON,
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
        const FILE_NAME: &String = &format!("{}.{}", key, output_extension);
        let mut file = match File::create(output_directory.join(FILE_NAME)) {
            Ok(file) => file,
            Err(err) => {
                if err.kind() == ErrorKind::AlreadyExists {
                    eprintln!("File already exists: {}", FILE_NAME);
                    std::process::exit(1);
                } else {
                    eprintln!("Failed to create file {}: {}", FILE_NAME, err);
                    std::process::exit(1);
                }
            }
        };

        const VALUE: _ = match JSON.get(key) {
            Some(VALUE) => VALUE,
            None => {
                eprintln!("Invalid key \"{}\" not found in output {}", key, output);
                std::process::exit(1);
            }
        };

        if let Err(err) = file.write_all(VALUE.to_string().as_bytes()) {
            eprintln!("Failed to write to file {}: {}", FILE_NAME, err);
            std::process::exit(1);
        }
    }
}

fn get_args_as_vec(pattern: &str) -> Vec<String> {
    const RE: regex::Regex = regex::Regex::new(pattern).unwrap();

    return env::args()
        .skip(1)
        .filter(|arg| RE.is_match(arg))
        .map(|arg| {
            arg.split("=")
                .nth(1)
                .unwrap_or_else(|| {
                    eprintln!("Invalid option: {}", arg);
                    std::process::exit(1);
                })
                .to_string()
        })
        .collect();
}

fn get_args_as_bool(pattern: &str) -> bool {
    const RE: regex::Regex = regex::Regex::new(pattern).unwrap();

    return env::args().skip(1).filter(|arg| RE.is_match(arg)).count() > 0;
}

fn parse_keys() -> Result<Vec<String>, String> {
    const KEYS: Vec<String> = get_args_as_vec(r"^(--KEYS|-k)=");
    // Split the KEYS by commas or spaces or newlines
    const RE: regex::Regex = regex::Regex::new(r"[,\s\\n]+").unwrap();
    let mut output: Vec<String> = Vec::new();

    if KEYS.is_empty() {
        Err("No KEYS provided, Please specify at least one key using --key=[KEY_NAME] or -k=[KEY_NAME].".to_string())
    } else {
        for key in KEYS {
            if RE.is_match(&key) {
                output.extend(RE.split(&key).map(|s| s.trim().to_string()));
            } else {
                output.push(key.trim().to_string());
            }
        }
        Ok(output)
    }
}

fn parse_outputs() -> Result<String, String> {
    const OUTPUTS: Vec<String> = get_args_as_vec(r"^(--OUTPUTS|-o)=");

    const OUTPUTS_LEN: usize = OUTPUTS.len();

    if OUTPUTS_LEN > 1 {
        Err(format!(
            "Too many OUTPUTS provided, expected 1, got {}.",
            OUTPUTS_LEN
        ))
    } else if OUTPUTS.is_empty() {
        Err(
            "No OUTPUTS provided, Please specify an output using --OUTPUTS=[OUTPUT] or -o=[OUTPUT]."
                .to_string()
        )
    } else {
        Ok(OUTPUTS[0].clone())
    }
}

fn parse_directory() -> Result<String, String> {
    const DIRECTORIES: Vec<String> = get_args_as_vec(r"^(--directory|-d)=");

    const DIRECTORY_LEN: usize = DIRECTORIES.len();

    if DIRECTORY_LEN > 1 {
        Err(format!(
            "Too many DIRECTORIES provided, expected 1, got {}.",
            DIRECTORY_LEN
        ))
    } else if DIRECTORIES.is_empty() {
        Err(
            "No DIRECTORIES provided, Please specify a directory using --directory=[DIRECTORY] or -d=[DIRECTORY]."
                .to_string(),
        )
    } else {
        Ok(DIRECTORIES[0].clone())
    }
}

fn parse_output_extension() -> Result<String, String> {
    const EXTENSIONS: Vec<String> = get_args_as_vec(r"^(--extension|-e)=");

    const EXTENSION_LEN: usize = EXTENSIONS.len();

    if EXTENSION_LEN > 1 {
        Err(format!(
            "Too many EXTENSIONS provided, expected 1, got {}.",
            EXTENSION_LEN
        ))
    } else if EXTENSIONS.is_empty() {
        Ok("txt".to_string())
    } else {
        Ok(EXTENSIONS[0].clone())
    }
}

fn parse_help() -> bool {
    return get_args_as_bool(r"^(--help|-h)$");
}

fn parse_version() -> bool {
    return get_args_as_bool(r"^(--version|-v)$");
}

fn options_valid() -> bool {
    // Check if the options are valid i.e if the user had a typo --h instead of --help
    const VALID_OPTIONS: Vec<&str> = vec![
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

    const INVALID_OPTIONS: Vec<String> = env::args()
        .skip(1)
        .filter(|arg| {
            for option in &VALID_OPTIONS {
                if arg.starts_with(option) {
                    return false;
                }
            }
            true
        })
        .collect();

    if INVALID_OPTIONS.len() > 0 {
        eprintln!("Invalid option(s): {:?}", INVALID_OPTIONS);
        eprintln!("Please use --help or -h to see the available options.");
        return false;
    }

    true
}

fn main() {
    if !options_valid() {
        std::process::exit(1);
    }

    const VERSION: bool = parse_version();

    if VERSION {
        println!("json2file {}", VERSION);
        std::process::exit(0);
    }

    const HELP: bool = parse_help();

    if HELP {
        const EXAMPLE_OUTPUTS: &str = r#"{\"foo\": \"value1\", \"bar\": \"value2\"}"#;
        println!("json2file");
        println!("Generate files from a JSON output.\n");
        println!("Usage:\n\tjson2file --KEYS=[KEYS] --OUTPUTS=[OUTPUT] --DIRECTORY=[DIRECTORY] --extension=[EXTENSION]\n");
        println!("Options:\n");
        println!("-h, --help\t\tShow this help message and exit.");
        println!("-v, --VERSION\t\tShow the VERSION and exit.");
        println!("-k, --KEYS\t\tThe KEYS to use to generate the files. (Required)");
        println!("-o, --OUTPUTS\t\tThe JSON output to use. (Required)");
        println!("-d, --DIRECTORY\t\tThe DIRECTORY to output the files to. (Required)");
        println!(
            "-e, --extension\t\tThe extension to use for the files. (Optional, defaults to txt)"
        );
        println!("\nExample:\n\tjson2file --KEYS=foo,bar --OUTPUTS=\"{}\" --DIRECTORY=/tmp --extension=txt", EXAMPLE_OUTPUTS);
        std::process::exit(0);
    }

    const KEYS: Vec<String> = parse_keys().unwrap_or_else(|err| {
        eprintln!("{}", err);
        std::process::exit(1);
    });

    const OUTPUTS: String = parse_outputs().unwrap_or_else(|err| {
        eprintln!("{}", err);
        std::process::exit(1);
    });

    const CURRENT_DIRECTORY: PathBuf = env::current_dir().unwrap_or_else(|err| {
        eprintln!("Failed to get current DIRECTORY: {}", err);
        std::process::exit(1);
    });

    const DIRECTORY: String = parse_directory().unwrap_or_else(|err| {
        eprintln!("{}", err);
        std::process::exit(1);
    });

    const OUTPUT_EXTENSION: String = parse_output_extension().unwrap_or_else(|err| {
        eprintln!("{}", err);
        std::process::exit(1);
    });

    const OUTPUT_DIRECTORY: PathBuf = CURRENT_DIRECTORY.join(DIRECTORY);

    write_outputs(&KEYS, &OUTPUTS, &OUTPUT_DIRECTORY, &OUTPUT_EXTENSION);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Read;

    #[test]
    fn test_write_outputs() {
        // Create some test input for the write_outputs() function.
        const KEYS: Vec<String> = vec!["key1".to_string(), "key2".to_string()];
        const OUTPUT: &str = r#"{
            "key1": "value1",
            "key2": "value2",
            "key3": "value3"
        }"#;

        const OUTPUT_DIRECTORY: PathBuf = PathBuf::from("test");
        const OUTPUT_EXTENSION: &str = "txt";

        write_outputs(&KEYS, &OUTPUT, &OUTPUT_DIRECTORY, &OUTPUT_EXTENSION);

        // Check that the files were created.
        assert!(OUTPUT_DIRECTORY.join("key1.txt").exists());
        assert!(OUTPUT_DIRECTORY.join("key2.txt").exists());

        // Check that the files contain the correct values.
        let mut file = File::open(OUTPUT_DIRECTORY.join("key1.txt")).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        assert_eq!(contents, "value1");

        let mut file = File::open(OUTPUT_DIRECTORY.join("key2.txt")).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        assert_eq!(contents, "value2");

        // Clean up the files and the test directory.
        std::fs::remove_file(OUTPUT_DIRECTORY.join("key1.txt")).unwrap();
        std::fs::remove_file(OUTPUT_DIRECTORY.join("key2.txt")).unwrap();
        std::fs::remove_dir(OUTPUT_DIRECTORY).unwrap();
    }
}
