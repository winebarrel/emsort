use super::file_utils;
use file_utils::RoughCount;
use io::prelude::BufRead;
use std::cmp::Ordering;
use std::fs;
use std::io;
use std::io::Write;
use std::path;

pub struct Chunk {
    pub capacity: u64,
    pub rough_count: file_utils::RoughCount,
    pub file_path: String,
}

impl Chunk {
    pub fn new(file_path: &str, cap: u64) -> io::Result<Chunk> {
        let path = path::Path::new(file_path);
        let file = fs::File::open(path)?;
        let rc = file_utils::count_roughly(&file)?;

        Ok(Chunk {
            file_path: file_path.to_string(),
            capacity: cap,
            rough_count: rc,
        })
    }

    pub fn fit_in_buffer(&self) -> io::Result<bool> {
        let path = path::Path::new(&self.file_path);
        let file = fs::File::open(path).unwrap();
        Ok(file.metadata().unwrap().len() <= self.capacity)
    }

    pub fn sort(&self, cmp: fn(&String, &String) -> Ordering) -> io::Result<Chunk> {
        let src_file = fs::File::open(path::Path::new(&self.file_path))?;
        let mut reader = io::BufReader::new(src_file);
        let mut lines = vec![];
        let mut buf = String::new();

        while reader.read_line(&mut buf)? > 0 {
            lines.push(buf.clone());
            buf.clear();
        }

        lines.sort_unstable_by(cmp);

        let dst_path = file_utils::temppath();
        let dst_file = fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open(&dst_path)?;
        let mut writer = io::BufWriter::new(dst_file);

        for l in lines {
            writer.write(l.as_bytes())?;
        }

        Ok(Chunk::new(dst_path.to_str().unwrap(), self.capacity)?)
    }

    pub fn split(&self) -> io::Result<(Chunk, Chunk)> {
        assert!(self.rough_count == RoughCount::Two || self.rough_count == RoughCount::ThreeOrMore);

        let src_file = fs::File::open(&self.file_path)?;
        let mid = src_file.metadata()?.len() / 2;
        let mut reader = io::BufReader::new(src_file);

        let c1_path = file_utils::temppath();
        let c2_path = file_utils::temppath();
        let c1_file = fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open(&c1_path)?;
        let c2_file = fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open(&c2_path)?;
        let mut writer1 = io::BufWriter::new(&c1_file);
        let mut writer2 = io::BufWriter::new(&c2_file);

        let mut sum = 0;
        let mut buf = String::new();

        while reader.read_line(&mut buf)? > 0 {
            sum += buf.len() as u64;
            writer1.write(buf.as_bytes())?;
            buf.clear();

            if sum >= mid || self.rough_count == RoughCount::Two {
                break;
            }
        }

        while reader.read_line(&mut buf)? > 0 {
            writer2.write(buf.as_bytes())?;
            buf.clear();
        }

        Ok((
            Chunk::new(c1_path.to_str().unwrap(), self.capacity)?,
            Chunk::new(c2_path.to_str().unwrap(), self.capacity)?,
        ))
    }
}
