#![allow(non_snake_case)]
#![deny(unsafe_code)]

use std::collections::HashMap;
use std::path::PathBuf;
use std::{fs, process::exit};
use std::{io, path::Path};
use structopt::StructOpt;

// TODO: DAY 2
    //HOUSE CLEANING: 1. fill dupes folder
    //HOUSE CLEANING: 2. Nicer visuals 

#[derive(Debug, StructOpt)]
#[structopt(name = "example", about = "An example of StructOpt usage.")]
struct CliOpts {
    #[structopt(short, long)]
    path: PathBuf,
}

fn main() {
    let dir = cli();
    let bufs = visitDirs(&dir);
    _hash(bufs.unwrap());
}

pub fn cli() -> String {
    let opt = CliOpts::from_args();
    let dir = opt.path.to_str().unwrap().to_string();
    let path = Path::new(&dir);
    if path.is_dir() {
        dir
    } else {
        println!("Enter a dir");
        exit(1);
    }
}

pub fn visitDirs(dirstr: &String) -> io::Result<Vec<PathBuf>> {
    let dir = Path::new(dirstr);
    let mut v: Vec<PathBuf> = Vec::new();

    if !dir.join("dupes").is_dir() {
        fs::create_dir(dir.join("dupes")).ok();
    }
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            v.push(path);
        }
    }
    Ok(v)
}

/// Takes in Vector of Pathbufs -> returns NONE.
/// Matches on photo files and if it does not match anything in the map it preforms an insert.  
pub fn _hash(entryVector: Vec<PathBuf>) {
    let mut photoHash: HashMap<&str, &str> = HashMap::new();
    let mut c: i64 = 0;
    for e in &entryVector {
        if let Ok(ext) = pathGetExtension(e) {
            match ext {
                "jpg" | "png" => {
                    if !photoHash.contains_key(e.to_str().unwrap()) {
                        photoHash.insert(
                            e.to_str().unwrap(),
                            e.file_name().unwrap().to_str().unwrap(),
                        );
                    } else { c += 1;  } 
                }
                _ => {}
            }
        }
        println!("{:?}", photoHash);
    }
}

pub fn pathGetExtension<'a>(_pathbuf: &'a PathBuf) -> Result<&'a str, &'static str> {
    match _pathbuf.extension() {
        Some(v) => Ok(v.to_str().unwrap()),
        None => Err("err"),
    }
}
