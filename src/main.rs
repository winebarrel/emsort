mod cli;
mod emsort;

use std::io;

fn main() -> io::Result<()> {
    let opts = cli::parse_opts();
    emsort::sort(&opts.file, io::stdout(), opts.capacity, None)
}
