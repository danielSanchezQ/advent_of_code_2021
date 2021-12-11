use std::fs;
use std::io;
use std::io::{BufRead, BufReader, Read};
use std::path::Path;
use std::str::FromStr;

pub fn open_file_read(path: &Path) -> io::Result<impl BufRead> {
    Ok(BufReader::new(fs::File::open(path)?))
}

pub fn read_vec_from_file<T: FromStr>(path: &Path) -> io::Result<Vec<T>> {
    let reader = open_file_read(path)?;
    let mut res = Vec::new();

    for line in reader.lines() {
        let line = line?;
        res.push(line.trim().parse().map_err(|_e| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Invalid data: {}", line),
            )
        })?);
    }

    Ok(res)
}

pub fn read_object_from_file<T: FromStr>(path: &Path) -> io::Result<T> {
    let mut reader = open_file_read(path)?;
    let mut buff = String::new();
    reader.read_to_string(&mut buff)?;
    buff.parse().map_err(|_e| {
        std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!("Couldn't parse: \n{}", buff),
        )
    })
}
