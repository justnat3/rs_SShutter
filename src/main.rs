#![allow(unused_imports)]
#![allow(non_snake_case)]
#![deny(unsafe_code)]

use std::fs;
use std::{env, fs::metadata};
//TODO:
// 1. implement cli::
//  1.top.whatever -> parse arguements
//  1.1 get cwd -> for . syntax
//  1.2 get dir -> for path
//  1.3 make -h form
//  1.4 make flags and positional arguments
//  1.5 write tests to ensure that the pathing works properly

// 2. implement IO(input/output)
// Perhaps should be a struct
//  2.1 search && create dupesdir
//  2.2 get both filename and path
//  2.3 implement items->caught

// 3. implement Hashing
//  3.1 create hashtable or hashmap
//  3.2 hash->check->insert/discard

/// A subcommand for controlling testing

fn main() {
    cli();
}

pub fn _help() {
    println!(
        "usage:
    DIR <String>
        Checks dir, and walks if dir exists
    DEBUG <BOOL>
    "
    );
}

// Owner of the CLI interface && primary functions of the application
pub fn cli() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => {
            _help();
        }
        2 => {
            if cfg!(windows) {
                if args[1].contains("/") || args[1].contains("\\") {
                    if fs::metadata(args[1].to_string()).is_ok() {
                        let dir = args[1].to_string();
                    }
                }
            }
        }
        _ => {
            _help();
        }
    }
}
