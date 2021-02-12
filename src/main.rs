use clap::{App, Arg};
use md5::{Digest, Md5};
use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, Read},
    path::Path,
    println,
};
use std::{fs::read_dir, path::PathBuf};

fn main() {
    let matches = App::new("cff")
    .version("1.0")
    .author("Łukasz Gmys <lgmys@pm.me>")
    .about("
        Finds canonical files in the given directory. Output consists of file paths, that can be passed to file management
        utilities, such as cp (with -t option) or mv.
    ")
    .arg(Arg::with_name("INPUT")
    .help("directory to scan for duplicates")
    .required(true)
    .index(1))
    .arg(Arg::with_name("recursive").short("r").long("recursive").required(false).takes_value(false).help("Scan directories recursively (false by default)"))
    .get_matches();

    let recursive = matches.is_present("recursive");
    let path = matches.value_of("INPUT").unwrap();

    let path = Path::new(path);

    assert!(
        path.exists(),
        "path {} does not exist",
        path.to_str().unwrap()
    );

    let mut uniques = HashMap::<String, PathBuf>::new();

    traverse_directory(&PathBuf::from(path), &mut uniques, recursive);

    for (_, path) in uniques {
        println!("'{}'", path.to_str().unwrap())
    }
}

fn traverse_directory(path: &PathBuf, results: &mut HashMap<String, PathBuf>, recursive: bool) {
    match read_dir(path) {
        Ok(entries) => {
            let paths: Vec<PathBuf> = entries.map(|e| e.unwrap().path()).collect();

            for path in paths {
                if path.is_dir() {
                    if recursive {
                        traverse_directory(&path, results, recursive)
                    }
                } else {
                    // compute hash
                    let mut hasher: Md5 = Md5::new();

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

                    results.insert(hash, path);
                }
            }
        }
        Err(err) => {
            panic!("could not read directory! {}", err);
        }
    }
}
