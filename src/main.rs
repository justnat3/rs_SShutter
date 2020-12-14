#![allow(non_snake_case)]
#![deny(unsafe_code)]

use std::collections::HashMap;
use std::path::PathBuf;
use std::{fs, process::exit};
use std::{io, path::Path};
use structopt::StructOpt;
//TODO:
// 1. implement cli
//  [x] 1.top.whatever -> parse arguements
//    [x] 1.1 get cwd -> for . syntax
//    [x] 1.2 get dir -> for path
//    [x] 1.3 make -h form
//    [] 1.4 write tests to ensure that the pathing works properly

// 2. implement IO(input/output)
// [x] verify and iterate over path
// [x] 2.1 search && create dupesdir
// [x] 2.2 get both filename
// [x] 2.whatever get file path
// [] 2.3 implement items->caught

// 3. implement Hashing
//  [x]3.1 create hashtable or hashmap
//  [x]3.2 hash->check->insert/discard
//  []3.3 hash only photos

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

fn visitDirs(dirstr: &String) -> io::Result<Vec<PathBuf>> {
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

pub fn _hash(entryVector: Vec<PathBuf>) {
    let mut photoHash: HashMap<&str, &str> = HashMap::new();
    for e in &entryVector {
        if ".jpg" == pathGetExtension(e) || ".png" == pathGetExtension(e) {
            photoHash.insert(
                e.to_str().unwrap(),
                e.file_name().unwrap().to_str().unwrap(),
            );
        }
    }
    println!("{:?}", photoHash);
}

pub fn pathGetExtension<'a>(_pathbuf: &'a PathBuf) -> &'a str {
    _pathbuf.extension().unwrap().to_str().unwrap()
}
