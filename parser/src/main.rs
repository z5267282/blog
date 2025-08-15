use parser::parse::dump_file::dump_blogs;

use clap::{ArgAction, Parser};
use std::io::Error;

fn main() -> Result<(), Error> {
    let args = Args::parse();
    env_logger::init();
    dump_blogs(args.pretty)?;
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
