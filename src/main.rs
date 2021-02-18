use clap::{crate_version, App, Arg};
use std::{collections::HashMap, path::Path};
use std::{fs::read_dir, path::PathBuf};

mod checksum;
mod output;

use output::render_output;

fn main() {
    let matches = App::new("cff")
    .version(crate_version!())
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
    .arg(Arg::with_name("progress").short("p").long("progress").required(false).takes_value(false).help("Display progress"))
    .get_matches();

    let recursive = matches.is_present("recursive");
    let progress = matches.is_present("progress");
    let path = matches.value_of("INPUT").unwrap();

    let path = Path::new(path);

    assert!(
        path.exists(),
        "path \"{}\" does not exist",
        path.to_str().unwrap()
    );

    let mut uniques = HashMap::<String, PathBuf>::new();

    traverse_directory(&PathBuf::from(path), &mut uniques, recursive, progress);

    if progress {
        println!();
    }

    render_output(&uniques);
}

fn traverse_directory(
    path: &PathBuf,
    results: &mut HashMap<String, PathBuf>,
    recursive: bool,
    progress: bool,
) {
    match read_dir(path) {
        Ok(entries) => {
            let paths: Vec<PathBuf> = entries.map(|e| e.unwrap().path()).collect();

            for path in paths {
                if path.is_dir() {
                    if recursive {
                        traverse_directory(&path, results, recursive, progress)
                    }
                } else {
                    let hash = checksum::compute(&path).unwrap();

                    results.insert(hash, path);

                    if progress {
                        print!("\r{} files processed", results.len());
                    }
                }
            }
        }
        Err(err) => {
            panic!("could not read directory! {}", err);
        }
    }
}
