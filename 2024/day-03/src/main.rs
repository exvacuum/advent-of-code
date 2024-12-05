use std::{
    fs::File,
    io::{BufRead, BufReader, Read},
    path::PathBuf,
};

use clap::Parser;
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
    for chunk in memory.split("m") {
        let _ = scan! { chunk;
            (
                "ul(",
                let a <| min_width(1, max_width(3, scan_a::<isize>())),
                ",",
                let b <| min_width(1, max_width(3, scan_a::<isize>())),
                ")",
                .. _
            ) => {
                sum += a * b;
            }
        };
    }
    println!("{sum}");
}
