use io::prelude::BufRead;
use std::env;
use std::fs;
use std::io;
use std::io::Seek;
use std::io::Write;
use std::path;
use std::process;

#[derive(Debug, PartialEq)]
pub enum RoughCount {
    Zero,
    One,
    Two,
    ThreeOrMore,
}

pub fn count_roughly(f: &fs::File) -> io::Result<RoughCount> {
    let mut reader = io::BufReader::new(f);
    let mut buf = String::new();
    let mut n = 0;

    while reader.read_line(&mut buf)? > 0 {
        buf.clear();
        n += 1;

        if n > 2 {
            break;
        }
    }

    let mut f = reader.into_inner();
    f.seek(io::SeekFrom::Start(0))?;

    let rc = match n {
        0 => RoughCount::Zero,
        1 => RoughCount::One,
        2 => RoughCount::Two,
        _ => RoughCount::ThreeOrMore,
    };

    Ok(rc)
}

pub fn copy<T>(fin: &fs::File, fout: T) -> io::Result<()>
where
    T: io::Write,
{
    let mut reader = io::BufReader::new(fin);
    let mut writer = io::BufWriter::new(fout);
    let mut buf = String::new();

    while reader.read_line(&mut buf)? > 0 {
        writer.write(buf.as_bytes())?;
        buf.clear();
    }

    Ok(())
}

pub fn tempdir() -> path::PathBuf {
    let dir = env::temp_dir();
    dir.join(format!("emsort-{}", process::id()))
}

pub struct RemoveDir {
    dir: path::PathBuf,
}

impl Drop for RemoveDir {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.dir).unwrap();
    }
}

pub fn create_directory() -> io::Result<RemoveDir> {
    let dir = tempdir();
    let rs = fs::create_dir(&dir);

    match rs {
        Ok(_) => Ok(RemoveDir { dir: dir }),
        Err(e) => Err(e),
    }
}

pub fn temppath() -> path::PathBuf {
    let id = ulid::Ulid::new().to_string();
    tempdir().join(id.to_string())
}
