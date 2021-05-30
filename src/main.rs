mod cli;
mod emsort;

use std::fs;
use std::io;

fn main() -> io::Result<()> {
  let opts = cli::parse_opts();
  let f = fs::File::open(opts.file)?;
  emsort::sort(f, io::stdout(), opts.capacity, None)
}
