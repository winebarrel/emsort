mod chunk;
pub mod file_utils;

use chunk::Chunk;
use file_utils::RoughCount;
use io::prelude::BufRead;
use std::cmp::Ordering;
use std::fs;
use std::io;
use std::io::Write;
use std::path;

fn default_compare(a: &String, b: &String) -> Ordering {
    let a = a.trim_end_matches(|c| c == '\r' || c == '\n');
    let b = b.trim_end_matches(|c| c == '\r' || c == '\n');
    a.partial_cmp(b).unwrap()
}

pub fn sort<T>(
    path: &str,
    fout: T,
    cap: u64,
    opt_cmp: Option<fn(&String, &String) -> Ordering>,
) -> io::Result<()>
where
    T: io::Write,
{
    let cmp = opt_cmp.unwrap_or(default_compare);
    let remove_dir = file_utils::create_directory()?;
    let chunk = Chunk::new(path, cap)?;
    let sorted = sort_chunk(chunk, cmp)?;
    let file = fs::File::open(sorted.file_path)?;
    file_utils::copy(&file, fout)?;
    drop(remove_dir);
    Ok(())
}

fn sort_chunk(chunk: Chunk, cmp: fn(&String, &String) -> Ordering) -> io::Result<Chunk> {
    if chunk.rough_count == RoughCount::Zero || chunk.rough_count == RoughCount::One {
        return Ok(chunk);
    }

    if chunk.fit_in_buffer()? {
        return chunk.sort(cmp);
    }

    let (c1, c2) = chunk.split()?;
    Ok(merge(sort_chunk(c1, cmp)?, sort_chunk(c2, cmp)?, cmp)?)
}

fn merge(c1: Chunk, c2: Chunk, cmp: fn(&String, &String) -> Ordering) -> io::Result<Chunk> {
    assert!(c1.capacity == c2.capacity);

    let c1_path = path::Path::new(&c1.file_path);
    let c2_path = path::Path::new(&c2.file_path);
    let c1_file = fs::File::open(c1_path)?;
    let c2_file = fs::File::open(c2_path)?;
    let mut reader1 = io::BufReader::new(c1_file);
    let mut reader2 = io::BufReader::new(c2_file);

    let dst_path = file_utils::temppath();
    let dst_file = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open(&dst_path)?;
    let mut writer = io::BufWriter::new(dst_file);

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

    Ok(Chunk::new(dst_path.to_str().unwrap(), c1.capacity)?)
}
