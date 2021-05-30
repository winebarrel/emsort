use io::prelude::BufRead;
use std::fs;
use std::io;
use std::io::Seek;
use std::io::Write;

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

pub fn rewind(mut f: &fs::File) -> io::Result<u64> {
  f.seek(io::SeekFrom::Start(0))
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
