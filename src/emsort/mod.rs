mod chunk;
pub mod file_utils;

use chunk::Chunk;
use file_utils::RoughCount;
use io::prelude::BufRead;
use std::cmp::Ordering;
use std::fs;
use std::io;
use std::io::Write;

fn default_compare(a: &String, b: &String) -> Ordering {
    let a = a.trim_end_matches(|c| c == '\r' || c == '\n');
    let b = b.trim_end_matches(|c| c == '\r' || c == '\n');
    a.partial_cmp(b).unwrap()
}

pub fn sort<T>(
    fin: fs::File,
    fout: T,
    cap: u64,
    opt_cmp: Option<fn(&String, &String) -> Ordering>,
) -> io::Result<()>
where
    T: io::Write,
{
    let cmp = opt_cmp.unwrap_or(default_compare);
    let chunk = Chunk::new(fin, cap)?;
    let sorted = sort_chunk(chunk, cmp)?;
    file_utils::copy(&sorted.file, fout)
}

fn sort_chunk(chunk: Chunk, cmp: fn(&String, &String) -> Ordering) -> io::Result<Chunk> {
    if chunk.rough_count == RoughCount::Zero || chunk.rough_count == RoughCount::One {
        return Ok(chunk);
    }

    if chunk.fit_in_buffer() {
        return chunk.sort(cmp);
    }

    let (c1, c2) = chunk.split()?;

    if c2.rough_count == RoughCount::Zero {
        return c1.sort(cmp);
    } else {
        Ok(merge(sort_chunk(c1, cmp)?, sort_chunk(c2, cmp)?, cmp)?)
    }
}

fn merge(c1: Chunk, c2: Chunk, cmp: fn(&String, &String) -> Ordering) -> io::Result<Chunk> {
    assert!(c1.capacity == c2.capacity);

    let mut reader1 = io::BufReader::new(&c1.file);
    let mut reader2 = io::BufReader::new(&c2.file);
    let mut writer = io::BufWriter::new(tempfile::tempfile()?);
    let mut r1_buf = String::new();
    let mut r2_buf = String::new();

    let mut r1_read = reader1.read_line(&mut r1_buf)?;
    let mut r2_read = reader2.read_line(&mut r2_buf)?;

    while r1_read > 0 && r2_read > 0 {
        if cmp(&r1_buf, &r2_buf) == Ordering::Less {
            // r1_buf < r2_buf
            writer.write(&r1_buf.as_bytes())?;
            r1_buf.clear();
            r1_read = reader1.read_line(&mut r1_buf)?
        } else {
            // r1_buf >= r2_buf
            writer.write(&r2_buf.as_bytes())?;
            r2_buf.clear();
            r2_read = reader2.read_line(&mut r2_buf)?
        }
    }

    while r1_read > 0 {
        writer.write(&r1_buf.as_bytes())?;
        r1_buf.clear();
        r1_read = reader1.read_line(&mut r1_buf)?
    }

    while r2_read > 0 {
        writer.write(&r2_buf.as_bytes())?;
        r2_buf.clear();
        r2_read = reader2.read_line(&mut r2_buf)?
    }

    let cap = c1.capacity;
    Ok(Chunk::new(writer.into_inner()?, cap)?)
}
