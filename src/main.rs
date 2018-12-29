extern crate serde_derive;

extern crate serde;
extern crate serde_json;
extern crate serde_yaml;
extern crate serde_transcode;

use std::io::prelude::*;
use std::fs::File;
use quicli::prelude::*;
use structopt::StructOpt;


// Add cool slogan for your app here, e.g.:
/// Get first n lines of a file
#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(long = "input", short = "i")]
    input: String,
    #[structopt(long = "output", short = "o")]
    output: String,
    #[structopt(flatten)]
    verbosity: Verbosity,
}

fn main() -> CliResult {
    let args = Cli::from_args();
    args.verbosity.setup_env_logger("json_to_yaml")?;
    let mut input_file = File::open(args.input)?;
    let mut output_file = File::create(args.output)?;

    let mut contents = String::new();
    input_file.read_to_string(&mut contents)?;

    let v: serde_yaml::Value = serde_json::from_str(contents.as_str())?;
    print!("{}", serde_yaml::to_string(&v).unwrap());
    write!(output_file, "{}", serde_yaml::to_string(&v).unwrap())?;
    Ok(())
}
