use parser::parse::dump_file::dump_blogs;

use parser::parse::paths::{JSON, MARKDOWN};

use clap::{ArgAction, Parser};
use std::io::Error;
use std::path::Path;

fn main() -> Result<(), Error> {
    let args = Args::parse();
    env_logger::init();
    let markdown = Path::new(MARKDOWN);
    let json = Path::new(JSON);
    dump_blogs(markdown, json, args.pretty)?;
    println!("successfully parsed blogs");
    Ok(())
}

/// Command line arguments for the blog parser.
#[derive(Parser, Debug)]
struct Args {
    /// Whether to pretty-print the JSON output.
    #[arg(short, long, action = ArgAction::SetTrue)]
    pretty: bool,
}
