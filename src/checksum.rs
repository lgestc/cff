use md5::{Digest, Md5};
use std::{
    fs::File,
    io::BufReader,
    io::{Read, Result},
    path::PathBuf,
};

pub fn compute(path: &PathBuf) -> Result<String> {
    // compute hash
    let mut hasher = Md5::new();

    let mut buffer = [0; 1024];

    let input = File::open(&path).unwrap();
    let mut reader = BufReader::new(input);

    loop {
        let count = reader.read(&mut buffer).unwrap();
        if count == 0 {
            break;
        }
        // process input bytes
        hasher.update(&buffer[..count]);
    }

    let hash = hasher.finalize();
    let hash = String::from_utf8_lossy(&hash).to_string();

    Ok(hash)
}
