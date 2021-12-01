use std::fs;
use std::io;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::str::FromStr;

pub fn open_file_read(path: &Path) -> io::Result<impl BufRead> {
    Ok(BufReader::new(fs::File::open(path)?))
}

pub fn read_vec_from_file<T: FromStr>(path: &Path) -> io::Result<Vec<T>> {
    let mut reader = open_file_read(path)?;
    let mut res = Vec::new();

    loop {
        let mut buff = String::new();
        match reader.read_line(&mut buff) {
            Ok(0) => break,
            Err(_) => break,
            Ok(_) => {
                res.push(buff.trim().parse().map_err(|_e| {
                    io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!("Invalid data: {}", buff),
                    )
                })?);
            }
        }
    }
    Ok(res)
}
