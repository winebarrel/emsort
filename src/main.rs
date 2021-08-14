mod cli;
use ex_merge_sort;

use std::fs;
use std::io;

fn main() -> io::Result<()> {
    let opts = cli::parse_opts();
    let f = fs::File::open(opts.file)?;
    ex_merge_sort::sort(f, io::stdout(), opts.capacity)
}
