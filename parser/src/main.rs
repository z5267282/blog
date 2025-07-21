use parser::parse::dump_file::dump_blogs;

use clap::{ArgAction, Parser};
use std::io::Error;

fn main() -> Result<(), Error> {
    let args = Args::parse();
    env_logger::init();
    dump_blogs(args.pretty)
}

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long, action = ArgAction::SetTrue)]
    pretty: bool,
}
