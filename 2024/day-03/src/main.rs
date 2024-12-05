use std::{
    fs::File,
    io::{BufRead, BufReader, Read},
    path::PathBuf,
};

use clap::Parser;
use regex::Regex;
use scan_rules::{
    scan, scan_rules_impl,
    scanner::{max_width, min_width, scan_a, Everything},
};

#[derive(Parser)]
struct Args {
    input: PathBuf,
}

fn main() {
    let args = Args::parse();

    let input = File::open(&args.input).unwrap();
    let mut memory = String::new();
    BufReader::new(input).read_to_string(&mut memory).unwrap();

    let mut sum = 0;
    let mut enabled = true;
    for chunk in Regex::new(r#"((mul\(\d{1,3},\d{1,3}\))|(do(n't)?\(\)))"#).unwrap().find_iter(&memory) {
        println!("{}", chunk.as_str());
        let _ = scan! { chunk.as_str();
            (
                "mul(",
                let a <| min_width(1, max_width(3, scan_a::<isize>())),
                ",",
                let b <| min_width(1, max_width(3, scan_a::<isize>())),
                ")",
                .. _
            ) => {
                if enabled {
                    sum += a * b;
                }
            },
            ("do()") => enabled = true,
            ("don't()") => enabled = false,
        };
    }
    println!("{sum}");
}
