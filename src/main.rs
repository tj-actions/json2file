use std::env;
use std::path::PathBuf;

use clap::Parser;

use writer::write_outputs;

mod args;
mod writer;

fn main() {
    // codecov: disable
    let args = args::Args::parse();

    let keys: Vec<String> = args
        .keys
        .split_whitespace()
        .map(|s| s.to_string())
        .collect();

    let current_directory: PathBuf = env::current_dir().unwrap_or_else(|err| {
        eprintln!("Failed to get current directory: {err}");
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
    use std::process;

    use crate::writer::create_output_directory;

    use super::*;

    struct TestContext {
        output_directory: PathBuf,
    }

    fn setup(directory: &str, create_output_dir: bool) -> TestContext {
        println!("Test setup...");

        let current_directory: PathBuf = env::current_dir().unwrap_or_else(|err| {
            eprintln!("Failed to get current directory: {err}");
            process::exit(1);
        });

        let output_directory: PathBuf = current_directory.join(directory);

        if create_output_dir {
            create_output_directory(&output_directory);
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
                process::exit(1);
            }
        }
    }

    #[test]
    fn test_valid_extension() {
        let args = args::Args::parse_from([
            "",
            "--keys",
            "key1 key2",
            "--outputs",
            "output",
            "--directory",
            "directory",
            "--extension",
            "txt",
        ]);
        assert_eq!(args.keys, "key1 key2");
        assert_eq!(args.outputs, "output");
        assert_eq!(args.directory, "directory");
        assert_eq!(args.extension.to_string(), "txt");
        assert!(!args.skip_missing_keys);
        assert!(!args.verbose);
    }

    #[test]
    fn test_invalid_extension() {
        let err = args::Args::try_parse_from([
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

        assert!(err.is_err());
        assert!(err.unwrap_err().to_string().contains("invalid value 'ext'"));
    }

    #[test]
    fn test_main_txt() {
        let args = args::Args::parse_from([
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
        let context = setup(&args.directory, true);
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
            output_directory,
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
        let args = args::Args::parse_from([
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
        let context = setup(&args.directory, true);
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
            output_directory,
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

    #[test]
    fn test_if_output_directory_doesnt_exist_it_gets_created() {
        let args = args::Args::parse_from([
            "",
            "--keys",
            "key1 key2",
            "--outputs",
            "{ \"key1\": \"value1\", \"key2\": \"value2\", \"key3\": \"value3\" }",
            "--directory",
            "test_output_directory_doesnt_exist",
            "--extension",
            "txt",
            "--skip-missing-keys",
        ]);

        let context = setup(&args.directory, false);
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
            output_directory,
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
    fn test_skips_invalid_keys() {
        let args = args::Args::parse_from([
            "",
            "--keys",
            "key1 key2 invalid_key",
            "--outputs",
            "{ \"key1\": \"value1\", \"key2\": \"value2\", \"key3\": \"value3\" }",
            "--directory",
            "test_txt_verbose",
            "--extension",
            "txt",
            "--skip-missing-keys",
            "--verbose",
        ]);

        let context = setup(&args.directory, true);
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
            output_directory,
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
    fn test_invalid_key_without_skip_missing_keys() {
        let context = setup("test_invalid_key_without_skip_missing_keys", true);
        let output_directory = &context.output_directory;

        let status = process::Command::new("cargo")
            .args([
                "run",
                "--",
                "--keys",
                "invalid_keys",
                "--outputs",
                "{ \"key1\": \"value1\", \"key2\": \"value2\", \"key3\": \"value3\" }",
                "--directory",
                (output_directory.to_str().unwrap()),
                "--extension",
                "txt",
            ])
            .status()
            .expect("failed to execute process");

        assert!(!status.success());
        assert_eq!(status.code().unwrap(), 1);
        assert!(!output_directory.join("invalid_keys.txt").exists());
        teardown(&context);
    }
}
