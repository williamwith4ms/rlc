use std::{env, io::{self}};
use std::io::Read;
use rayon::prelude::*;

fn main() {
    let files: Vec<_> = env::args().skip(1).collect();

    files.par_iter().for_each(|file| {
        match count_lines(file) {
            Ok(count) => println!("{count} {file}"),
            Err(e) => {
                match e.kind() {
                    io::ErrorKind::NotFound => eprintln!("rlc: File not found - {file}"),
                    io::ErrorKind::PermissionDenied => eprintln!("rlc: Permission denied - {file}"),
                    io::ErrorKind::IsADirectory => eprintln!("rlc: Expected file but found a directory - {file}"),
                    io::ErrorKind::InvalidData => eprintln!("rlc: Invalid data in file - {file}"),
                    _ => eprintln!("rlc: Could not read file {file}: {e}"),
                }
            },
        }
    });

}

fn count_lines(file: &str) -> io::Result<usize> {
    let mut file = std::fs::File::open(file)?;
    let mut buffer = [0; 8192];
    let mut count = 0;

    loop {
        let bytes_read = file.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        count += bytecount::count(&buffer[..bytes_read], b'\n');
    }

    Ok(count)
}
