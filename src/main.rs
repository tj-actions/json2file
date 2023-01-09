use std::env;
use std::path::PathBuf;

use clap::Parser;

use writer::write_outputs;

mod args;
mod writer;

fn main() {
    let args = args::Args::parse();

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
    use std::fs::File;
    use std::io::Read;

    use super::*;

    #[test]
    fn test_args() {
        let args = args::Args::parse_from(&[
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
    fn test_main_txt() {
        let args = args::Args::parse_from(&[
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
        std::fs::remove_dir_all(output_directory).unwrap();
    }

    #[test]
    fn test_main_json() {
        let args = args::Args::parse_from(&[
            "",
            "--keys",
            "key1 key2",
            "--outputs",
            "{ \"key1\": \"value1\", \"key2\": \"value2\", \"key3\": \"value3\" }",
            "--directory",
            "test",
            "--extension",
            "json",
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
        assert!(output_directory.join("key1.json").exists());
        assert!(output_directory.join("key2.json").exists());

        // Check that the files contain the correct values.
        let mut file = File::open(output_directory.join("key1.json")).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        assert_eq!(contents, "\"value1\"");

        let mut file = File::open(output_directory.join("key2.json")).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        assert_eq!(contents, "\"value2\"");

        // Clean up the files and the test directory.
        std::fs::remove_dir_all(output_directory).unwrap();
    }
}
