use parser::parse::dump_file::dump_blogs;

use std::io::Error;

fn main() -> Result<(), Error> {
    env_logger::init();
    dump_blogs()
}
