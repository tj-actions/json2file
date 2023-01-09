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

    struct TestContext {
        output_directory: PathBuf,
    }

    fn setup(directory: &str) -> TestContext {
        println!("Test setup...");

        let current_directory: PathBuf = env::current_dir().unwrap_or_else(|err| {
            eprintln!("Failed to get current directory: {}", err);
            std::process::exit(1);
        });

        let output_directory: PathBuf = current_directory.join(directory);

        // Create the output directory if it doesn't exist
        if !output_directory.exists() {
            println!("Creating output directory...");
            match std::fs::create_dir_all(&output_directory) {
                Ok(_) => println!("Output directory created successfully."),
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
            println!("Output directory already exists.");
        }

        TestContext { output_directory }
    }

    fn teardown(context: &TestContext) {
        println!("Test teardown...");

        // Remove the output directory
        println!("Removing output directory...");
        match std::fs::remove_dir_all(&context.output_directory) {
            Ok(_) => println!("Output directory removed successfully."),
            Err(e) => {
                eprintln!(
                    "Error removing output directory '{}': {}",
                    context.output_directory.display(),
                    e
                );
                std::process::exit(1);
            }
        }
    }

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
            "test_txt",
            "--extension",
            "txt",
            "--skip-missing-keys",
        ]);
        let context = setup(&args.directory);
        let output_directory = &context.output_directory;
        let keys: Vec<String> = args
            .keys
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();

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
        teardown(&context);
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
            "test_json",
            "--extension",
            "json",
            "--skip-missing-keys",
        ]);
        let context = setup(&args.directory);
        let output_directory = &context.output_directory;
        let keys: Vec<String> = args
            .keys
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();

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
        teardown(&context);
    }
}
