use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Space delimited list of keys to extract from the JSON output.
    #[clap(short, long, required = true)]
    pub keys: String,

    /// The JSON output to use.
    #[arg(short, long, required = true)]
    pub outputs: String,

    /// The directory to output the files to.
    #[arg(short, long, required = true)]
    pub directory: String,

    /// Skip missing keys.
    #[arg(short, long)]
    pub skip_missing_keys: bool,

    /// The extension to use for the files.
    #[arg(short, long, default_value = "txt")]
    pub extension: String,

    #[arg(short, long)]
    pub verbose: bool,
}
